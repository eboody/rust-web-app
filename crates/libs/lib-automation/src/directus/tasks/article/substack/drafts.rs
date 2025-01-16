#![allow(unused)]
use async_recursion::async_recursion;
use die_exit::die;
use lib_core::model::directus::{self, Tags, VecString};
use lib_substack::{drafts, prelude::*};
use ormlite::types::Uuid;
use statum::*;
use tokio::time::sleep;
use tracing::{info, warn};

const BYLINE_ID: i64 = 292604153;

/// Export an article to Substack as a draft
#[async_recursion]
pub async fn create(mm: &ModelManager, article_id: Uuid) -> Result<directus::SubstackDraft> {
    let substack_draft = directus::SubstackDraft::select()
        .where_("articles_id = ?")
        .bind(article_id)
        .fetch_one(mm.orm())
        .await;

    if let Ok(substack_draft) = substack_draft {
        warn!("Draft already exists for article {}", article_id);
        return Ok(substack_draft);
    }

    // Fetch the article
    let article = directus::Articles::select()
        .where_("articles.id = ?")
        .bind(article_id)
        .join(directus::Articles::author())
        .fetch_one(mm.orm())
        .await?;

    let byline = super::author::get_byline(mm, &article.author).await;

    if let Ok(linked_articles) = directus::RelatedArticles::select()
        .where_("articles_id = ?")
        .bind(article_id)
        .fetch_all(mm.orm())
        .await
    {
        for linked_article in linked_articles {
            create(mm, linked_article.related_articles_id).await?;
        }
    }

    let tags_in_substack = lib_substack::Tag::list(mm.reqwest()).await?;

    // Export to Substack
    let draft_response =
        lib_substack::drafts::Request::export_from_article(mm, &article, BYLINE_ID).await?;

    for tag_name in article.tags.as_vec_string() {
        let substack_tag = Tags::select()
            .where_("name = ?")
            .bind(&tag_name)
            .fetch_one(mm.orm())
            .await;

        let mut tag: lib_substack::Tag;

        match (
            tags_in_substack
                .iter()
                .map(|t| &t.name)
                .any(|t| t == &tag_name),
            &substack_tag,
        ) {
            // Case 1: `tag_name` not in `tags_in_substack` and `substack_tag` is an error
            (false, Err(_)) => {
                tag = lib_substack::Tag::create(mm.reqwest(), tag_name.to_string())
                    .await
                    .expect("Failed to create tag case 1");
            }

            // Case 2: `tag_name` is in `tags_in_substack` and `substack_tag` is valid
            (true, Ok(substack_tag)) => {
                tag = substack_tag.to_owned().into();
            }

            // Case 3: `tag_name` not in `tags_in_substack`, but `substack_tag` is valid
            (false, Ok(_)) => {
                substack_tag?.delete(mm.orm()).await?;
                tag = lib_substack::Tag::create(mm.reqwest(), tag_name.to_string())
                    .await
                    .expect("Failed to create tag case 3");
            }

            // Case 4: `tag_name` is in `tags_in_substack`, but `substack_tag` is an error
            (true, Err(_)) => {
                tag = tags_in_substack
                    .iter()
                    .find(|t| t.name == tag_name)
                    .inspect(|&t| {
                        tracing::error!("Tag not found: {:?}", t);
                    })
                    .expect("Tag not found")
                    .clone();
            }
        }

        // Use the `tag` after handling all cases
        tag.add_to_post(mm.reqwest(), draft_response.id)
            .await
            .expect("Failed to add tag to post");

        let directus_tag: Tags = tag.into();
        directus_tag
            .insert(mm.orm())
            .on_conflict(OnConflict::Ignore)
            .await?;
        info!("Added tag to post in Substack: {}", tag_name);
    }

    info!(
        "Exported Substack draft {} for article {}",
        &draft_response.id, &article_id
    );

    let substack_draft = draft_response
        .clone()
        .into_substack_draft(article_id)
        .insert(mm.orm())
        .await;

    if substack_draft.is_err() {
        warn!("Failed to insert Substack draft into database");
        delete(mm, article_id).await?;
    }

    let substack_draft = substack_draft?;

    // Update article with export timestamp
    article
        .update_partial()
        .date_updated(Some(OffsetDateTime::now_utc()))
        .update(mm.orm())
        .await?;

    Ok(substack_draft)
}

/// Delete a draft from Substack and update article status
pub async fn delete(mm: &ModelManager, article_id: Uuid) -> Result<()> {
    let substack_draft = directus::SubstackDraft::select()
        .where_("articles_id = ?")
        .bind(article_id)
        .fetch_one(mm.orm())
        .await?;

    // Delete from Substack
    drafts::Request::delete(mm.reqwest(), substack_draft.substack_draft_id).await?;

    info!(
        "Deleted Substack draft {} for article {}",
        substack_draft.substack_draft_id, article_id
    );

    substack_draft.delete(mm.orm()).await?;

    // Update article status
    directus::Articles::select()
        .where_("id = ?")
        .bind(article_id)
        .fetch_one(mm.orm())
        .await?
        .update_partial()
        .date_updated(Some(OffsetDateTime::now_utc()))
        .update(mm.orm())
        .await?;

    Ok(())
}

/// Update an existing Substack draft from article content
pub async fn update_substack_draft(
    mm: &ModelManager,
    article_id: Uuid,
    draft_id: i64,
) -> Result<directus::SubstackDraft> {
    // First delete existing draft
    drafts::Request::delete(mm.reqwest(), draft_id).await?;

    // Create new draft with updated content
    let response = create(mm, article_id).await?;

    info!(
        "Updated Substack draft {} for article {} (new draft ID: {})",
        draft_id, article_id, response.id
    );

    Ok(response)
}

/// Publish a Substack draft
pub async fn publish_substack_draft(
    mm: &ModelManager,
    article_id: Uuid,
    _draft_id: i64,
) -> Result<()> {
    // TODO: Implement publish API call when available
    warn!("Publishing functionality not yet implemented");

    // Update article status
    directus::Articles::select()
        .where_("id = ?")
        .bind(article_id)
        .fetch_one(mm.orm())
        .await?
        .update_partial()
        .date_updated(Some(OffsetDateTime::now_utc()))
        .update(mm.orm())
        .await?;

    Ok(())
}

/// Get the current status of an article's Substack draft
pub async fn get_draft_status(
    mm: &ModelManager,
    article_id: Uuid,
) -> Result<Option<drafts::Response>> {
    // TODO: Implement status check logic
    // This would typically involve checking your article_substack_status table
    // and then validating against Substack's API
    warn!("Draft status check not yet implemented");
    Ok(None)
}
