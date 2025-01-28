#![allow(unused)]

use std::{io::Write, str::FromStr, sync::LazyLock};

use async_recursion::async_recursion;
use bon::builder;
use die_exit::die;
use lib_core::model::{self, Status, Tags, VecString};
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

/// Delete a draft from Substack and update article status
pub async fn delete(mm: &ModelManager, article_id: Uuid) -> Result<()> {
    let substack_draft = model::SubstackDraft::select()
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
    model::Articles::select()
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
) -> Result<model::SubstackDraft> {
    todo!()
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

#[builder]
pub async fn create_iterative(
    model_manager: &ModelManager,
    root_article_id: Uuid,
    should_publish: Option<bool>,
) -> Result<()> {
    let mut stack = vec![root_article_id]; // Initialize the stack with the root article ID
    let mut visited = HashSet::new(); // Keep track of visited articles to avoid cycles

    while let Some(article_id) = stack.pop() {
        if visited.contains(&article_id) {
            warn!("Article {:?} already visited—skipping", article_id);
            continue;
        }
        visited.insert(article_id);

        // Attempt to fetch the Substack draft
        if let Ok(existing_draft) = model::SubstackDraft::select()
            .where_("articles_id = ?")
            .bind(article_id)
            .fetch_one(model_manager.orm())
            .await
        {
            // Check if the draft is published
            let substack_draft_status = lib_substack::drafts::Request::get(
                model_manager.reqwest(),
                existing_draft.substack_draft_id,
            )
            .await;

            if let Ok(status) = substack_draft_status {
                if status.is_published.unwrap_or(false) {
                    info!(
                        "Draft for article {:?} is already published—skipping.",
                        existing_draft.draft_title
                    );
                    continue;
                } else if should_publish.unwrap_or(false) {
                    // Publish the existing draft
                    lib_substack::drafts::Request::publish(
                        model_manager.reqwest(),
                        existing_draft.substack_draft_id,
                        drafts::PublishArgs {
                            send: false,
                            share_automatically: false,
                        },
                    )
                    .await?;
                    info!(
                        "Published existing draft for article {:?}",
                        existing_draft.draft_title
                    );
                    continue;
                }
            }
        }

        // Fetch the article
        let mut article = model::Articles::select()
            .where_("articles.id = ?")
            .bind(article_id)
            .join(model::Articles::author())
            .fetch_one(model_manager.orm())
            .await?;

        // Process related articles
        if let Ok(article_links) = model::RelatedArticles::select()
            .where_("articles_id = ?")
            .bind(article_id)
            .fetch_all(model_manager.orm())
            .await
        {
            for article_link in article_links {
                // Check if the related article already has a Substack draft
                if let Ok(substack_draft) = model::SubstackDraft::select()
                    .where_("articles_id = ?")
                    .bind(article_link.related_articles_id)
                    .fetch_one(model_manager.orm())
                    .await
                {
                    //warn!(
                    //    "Draft already exists for related article {:#?}",
                    //    substack_draft.draft_title
                    //);
                    continue;
                }

                // Add related article to the stack for further processing
                stack.push(article_link.related_articles_id);
            }
        }

        println!("\n\n\nCreating: {:?}", article.title);

        // Proceed with Substack export logic
        let draft_response =
            lib_substack::drafts::Request::export_from_article(model_manager, &article, BYLINE_ID)
                .await?;

        let substack_draft = draft_response
            .clone()
            .into_substack_draft(article_id)
            .insert(model_manager.orm())
            .on_conflict(OnConflict::do_update_on_pkey("articles_id"))
            .await;

        if substack_draft.is_err() {
            warn!("Failed to insert Substack draft into database");
            delete(model_manager, article_id).await?;
            continue;
        }

        let mut substack_draft = substack_draft?;

        article = article
            .update_partial()
            .substack_draft(Some(substack_draft.id))
            .status(Status::Draft)
            .update(model_manager.orm())
            .await?;

        // Upload and set the cover image
        let cover_image_file = if let Ok(file) = model::Files::select()
            .where_("id = ?")
            .bind(article.featured_image)
            .fetch_one(model_manager.orm())
            .await
        {
            file
        } else {
            model::Files::select()
                .where_("id = ?")
                .bind(Uuid::from_str("a8fd08ec-82bf-484e-a032-fea9a7892878").expect("Invalid UUID"))
                .fetch_one(model_manager.orm())
                .await
                .expect("Failed to fetch fallback image")
        };

        let substack_cover_image = lib_substack::Image::upload_to_substack(
            model_manager,
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

        model_manager
            .reqwest()
            .put(url)
            .headers(config().HEADERS.clone())
            .body(json::to_string(&substack_draft).unwrap())
            .retry()
            .send::<lib_substack::drafts::Response>()
            .await
            .expect("Failed to update draft with cover image");

        info!(
            "Uploaded cover image for Substack draft {:#?}",
            substack_draft.draft_title
        );

        // Add tags to Substack draft
        for tag_name in article
            .tags
            .as_vec_string()
            .into_iter()
            .map(|t| t.to_lowercase())
            .collect::<HashSet<String>>()
            .into_iter()
        {
            // Fetch or create the tag in Substack
            let tag = match lib_substack::Tag::list(model_manager.reqwest())
                .await?
                .into_iter()
                .find(|t| t.name.eq_ignore_ascii_case(&tag_name))
            {
                Some(existing_tag) => existing_tag,
                None => lib_substack::Tag::create(model_manager.reqwest(), tag_name.clone())
                    .await
                    .expect("Failed to create tag"),
            };

            // Add the tag to the draft
            tag.add_to_post(model_manager.reqwest(), draft_response.id)
                .await?;
        }

        info!(
            "Exported Substack draft {} for article {:#?}",
            &draft_response.id, &article.title
        );

        // Publish the draft after tags are added if publishing is enabled
        if should_publish.unwrap_or(false) {
            lib_substack::drafts::Request::publish(
                model_manager.reqwest(),
                draft_response.id,
                drafts::PublishArgs {
                    send: false,
                    share_automatically: false,
                },
            )
            .await?;
            info!("Published new draft for article {:?}", article_id);
        }

        // Update article status
        article
            .update_partial()
            .date_updated(Some(OffsetDateTime::now_utc()))
            .status(Status::Published)
            .update(model_manager.orm())
            .await?;
    }

    Ok(())
}
