#![allow(unused)]

use std::{io::Write, sync::LazyLock};

use async_recursion::async_recursion;
use die_exit::die;
use lib_core::model::directus::{self, Tags, VecString};
use lib_substack::{drafts, prelude::*};
use lib_utils::retry::RetryableRequest;
use statum::*;
use time::{Time, UtcOffset};
use tokio::{sync::OnceCell, time::sleep};
use tracing::{info, warn};
use url::Url;

const BYLINE_ID: i64 = 292604153;
use std::collections::HashSet;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::directus::tasks::handle_images;

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

    if let Ok(substack_draft) = substack_draft {
        // Draft already exists
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

            let get_res =
                lib_substack::drafts::Request::get(mm.reqwest(), ss_draft.substack_draft_id).await;

            if let Ok(get_res) = &get_res {
                if get_res.is_published == Some(true) {
                    warn!("Draft already published: {:?}", get_res.title);
                    continue;
                }
            }

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
                info!("Published draft: {:?}", rel_article.title);
            }
        }
    }

    let mut tags_in_substack = lib_substack::Tag::list(mm.reqwest())
        .await?
        .into_iter()
        .map(|mut t| {
            t.name = t.name.to_lowercase();
            t
        })
        .collect::<Vec<_>>();

    // Export to Substack
    let draft_response =
        lib_substack::drafts::Request::export_from_article(mm, &article, BYLINE_ID).await?;

    for tag_name in article
        .tags
        .as_vec_string()
        .into_iter()
        .map(|t| t.to_lowercase())
    {
        let substack_tag = Tags::select()
            .where_("name = ?")
            .bind(&tag_name)
            .fetch_one(mm.orm())
            .await;

        let mut tag: lib_substack::Tag;

        match (
            tags_in_substack
                .iter()
                .map(|t| t.name.to_lowercase())
                .any(|t| t == tag_name),
            &substack_tag,
        ) {
            // Case 1: `tag_name` not in `tags_in_substack` and `substack_tag` isnt in db
            (false, Err(_)) => {
                info!("case1: {}", tag_name);
                tag = lib_substack::Tag::create(mm.reqwest(), tag_name.to_string())
                    .await
                    .expect("Failed to create tag case 1");
            }

            // Case 2: `tag_name` is in `tags_in_substack` and `substack_tag` is valid
            (true, Ok(substack_tag)) => {
                tag = substack_tag.to_owned().into();
            }

            // Case 3: `tag_name` not in `tags_in_substack`, but `substack_tag` is in db
            (false, Ok(_)) => {
                substack_tag?.delete(mm.orm()).await?;

                info!("case3, {}", tag_name);
                tag = lib_substack::Tag::create(mm.reqwest(), tag_name.to_string())
                    .await
                    .expect("Failed to create tag case 3");
            }

            // Case 4: `tag_name` is in `tags_in_substack`, but `substack_tag` not in db
            (true, Err(_)) => {
                tag = tags_in_substack
                    .iter()
                    .find(|t| t.name == tag_name)
                    .inspect(|&t| {
                        tracing::info!("Tag not found in db: {:?}", t);
                    })
                    .expect("Tag not found")
                    .clone();
            }
        }

        // Use the `tag` after handling all cases
        info!("Adding tag to post: {}", tag_name);
        tag.add_to_post(mm.reqwest(), draft_response.id)
            .await
            .expect("Failed to add tag to post");

        let directus_tag: Tags = tag.into();

        directus_tag
            .insert(mm.orm())
            .on_conflict(OnConflict::do_update_on_pkey("id"))
            .await?;

        info!("Added tag to post in Substack: {}", tag_name);
    }

    info!(
        "Exported Substack draft {} for article {}",
        &draft_response.id, &article_id
    );

    let draft_response = drafts::Request::get(mm.reqwest(), draft_response.id).await?;

    let substack_draft = draft_response
        .into_substack_draft(article_id)
        .insert(mm.orm())
        .on_conflict(OnConflict::do_update_on_pkey("articles_id"))
        .await;

    if substack_draft.is_err() {
        warn!("Failed to insert Substack draft into database");
        delete(mm, article_id).await?;
    }

    let mut substack_draft = substack_draft?;

    let cover_image_file = directus::Files::select()
        .where_("id = ?")
        .bind(article.featured_image)
        .fetch_one(mm.orm())
        .await;

    if let Ok(cover_image_file) = cover_image_file {
        let substack_cover_image = lib_substack::Image::upload_to_substack(
            mm,
            cover_image_file,
            substack_draft.substack_draft_id,
        )
        .await
        .expect("Failed to upload cover image");

        let url = Url::parse(&format!(
            "{}/drafts/{}",
            &config().API_URL,
            substack_draft.substack_draft_id
        ))?;

        substack_draft.cover_image = Some(substack_cover_image.url.clone());
        substack_draft.slug = article.slug.clone();

        let res = mm
            .reqwest()
            .put(url)
            .headers(config().HEADERS.clone())
            .body(json::to_string(&substack_draft).unwrap())
            .retry()
            .send::<lib_substack::drafts::Response>()
            .await
            .expect("Failed to parse response");

        info!(
            "Uploaded cover image for Substack draft {:#?}",
            substack_draft.draft_title
        );
    }

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
