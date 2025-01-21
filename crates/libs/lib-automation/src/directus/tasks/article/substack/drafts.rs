#![allow(unused)]

use std::{io::Write, sync::LazyLock};

use async_recursion::async_recursion;
use die_exit::die;
use lib_core::model::directus::{self, Tags, VecString};
use lib_substack::{drafts, prelude::*};
use statum::*;
use tokio::{sync::OnceCell, time::sleep};
use tracing::{info, warn};

const BYLINE_ID: i64 = 292604153;
use std::collections::HashSet;
use tokio::sync::Mutex;
use uuid::Uuid;

static RECURSING: LazyLock<Mutex<HashSet<Uuid>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// Export an article to Substack as a draft
#[async_recursion]
pub async fn create(mm: &ModelManager, article_id: Uuid) -> Result<directus::SubstackDraft> {
    // Initialize the RECURSING mutex lazily
    let visited_mutex = &*RECURSING;
    let mut visited = visited_mutex.lock().await;

    // If we've seen this article, skip
    if visited.contains(&article_id) {
        warn!(
            "Article {:?} already visited—skipping to avoid recursion",
            article_id
        );
        // Return an appropriate error or propagate as needed
        return Err(lib_substack::Error::AlreadyVisited);
    }
    visited.insert(article_id);
    drop(visited); // Explicitly release the lock

    // Attempt to fetch the Substack draft
    let substack_draft = directus::SubstackDraft::select()
        .where_("articles_id = ?")
        .bind(article_id)
        .fetch_one(mm.orm())
        .await;

    //match substack_draft {
    //    Ok(draft) => {
    //        // Draft already exists
    //        warn!("Draft already exists for article {}", article_id);
    //        Ok(draft)
    //    }
    //    Err(err) => {
    //        // Handle error or proceed to create a draft
    //        tracing::error!("Failed to fetch draft: {:?}", err);
    //        Err(err)
    //    }
    //}

    // Fetch the article
    let article = directus::Articles::select()
        .where_("articles.id = ?")
        .bind(article_id)
        .join(directus::Articles::author())
        .fetch_one(mm.orm())
        .await?;

    let byline = super::author::get_byline(mm, &article.author).await;

    if let Ok(article_links) = directus::RelatedArticles::select()
        .where_("articles_id = ?")
        .bind(article_id)
        .fetch_all(mm.orm())
        .await
    {
        for article_link in article_links {
            let substack_draft = directus::SubstackDraft::select()
                .where_("articles_id = ?")
                .bind(article_link.related_articles_id)
                .fetch_one(mm.orm())
                .await;

            let rel_article = directus::Articles::select()
                .where_("articles.id = ?")
                .bind(article_link.related_articles_id)
                .join(directus::Articles::author())
                .fetch_one(mm.orm())
                .await?;

            if let Ok(substack_draft) = substack_draft {
                warn!("Draft already exists for article {:#?}", rel_article.title);
                continue;
            }

            let ss_draft = create(mm, article_link.related_articles_id).await?;

            let res: lib_substack::drafts::Response = ss_draft.clone().into();

            //let get_res =
            //    lib_substack::drafts::Request::get(mm.reqwest(), ss_draft.substack_draft_id).await;
            //
            ////tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            //
            //if let Ok(get_res) = &get_res {
            //    if get_res.is_published == Some(true) {
            //        warn!("Draft already published: {:?}", get_res.title);
            //        continue;
            //    }
            //}

            let pub_res = lib_substack::drafts::Request::publish(
                mm.reqwest(),
                ss_draft.substack_draft_id,
                drafts::PublishArgs {
                    send: false,
                    share_automatically: false,
                },
            )
            .await;

            if pub_res.is_err() {
                tracing::warn!(
                    "Failed to publish draft: {:?} {:#?}",
                    ss_draft.substack_draft_id,
                    ss_draft.title
                );
                delete(mm, article_link.related_articles_id).await?;
                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("articles.txt")
                    .unwrap();
                let article_info = format!(
                    "Article: {}\nSubstack Draft ID: {}\n\n",
                    article_link.related_articles_id, res.id
                );

                file.write_all(article_info.as_bytes()).unwrap();
            } else {
                warn!("Published draft: {:?}", rel_article.title);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    //let tags_in_substack = lib_substack::Tag::list(mm.reqwest()).await?;

    // Export to Substack
    let draft_response =
        lib_substack::drafts::Request::export_from_article(mm, &article, BYLINE_ID).await?;

    //for tag_name in article.tags.as_vec_string() {
    //    let substack_tag = Tags::select()
    //        .where_("name = ?")
    //        .bind(&tag_name)
    //        .fetch_one(mm.orm())
    //        .await;
    //
    //    let mut tag: lib_substack::Tag;
    //
    //    match (
    //        tags_in_substack
    //            .iter()
    //            .map(|t| &t.name)
    //            .any(|t| t == &tag_name),
    //        &substack_tag,
    //    ) {
    //        // Case 1: `tag_name` not in `tags_in_substack` and `substack_tag` is an error
    //        (false, Err(_)) => {
    //            tag = lib_substack::Tag::create(mm.reqwest(), tag_name.to_string())
    //                .await
    //                .expect("Failed to create tag case 1");
    //        }
    //
    //        // Case 2: `tag_name` is in `tags_in_substack` and `substack_tag` is valid
    //        (true, Ok(substack_tag)) => {
    //            tag = substack_tag.to_owned().into();
    //        }
    //
    //        // Case 3: `tag_name` not in `tags_in_substack`, but `substack_tag` is valid
    //        (false, Ok(_)) => {
    //            substack_tag?.delete(mm.orm()).await?;
    //            tag = lib_substack::Tag::create(mm.reqwest(), tag_name.to_string())
    //                .await
    //                .expect("Failed to create tag case 3");
    //        }
    //
    //        // Case 4: `tag_name` is in `tags_in_substack`, but `substack_tag` is an error
    //        (true, Err(_)) => {
    //            tag = tags_in_substack
    //                .iter()
    //                .find(|t| t.name == tag_name)
    //                .inspect(|&t| {
    //                    tracing::error!("Tag not found: {:?}", t);
    //                })
    //                .expect("Tag not found")
    //                .clone();
    //        }
    //    }
    //
    //    // Use the `tag` after handling all cases
    //    tag.add_to_post(mm.reqwest(), draft_response.id)
    //        .await
    //        .expect("Failed to add tag to post");
    //
    //    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    //
    //    let directus_tag: Tags = tag.into();
    //
    //    directus_tag
    //        .insert(mm.orm())
    //        .on_conflict(OnConflict::Ignore)
    //        .await?;
    //
    //    info!("Added tag to post in Substack: {}", tag_name);
    //}

    info!(
        "Exported Substack draft {} for article {}",
        &draft_response.id, &article_id
    );

    let substack_draft = draft_response
        .clone()
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
