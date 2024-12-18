#![allow(unused)]
use lib_core::model::directus;
use lib_substack::{drafts, prelude::*};
use ormlite::types::Uuid;
use tracing::{info, warn};

use async_recursion::async_recursion;

/// Export an article to Substack as a draft
#[async_recursion]
pub async fn create_substack_draft(
  mm: &ModelManager,
  article_id: Uuid,
) -> Result<directus::SubstackDraft> {
  let substack_draft = directus::SubstackDraft::select()
    .where_("article_id = ?")
    .bind(article_id)
    .fetch_one(mm.orm())
    .await;

  if let Ok(substack_draft) = substack_draft {
    warn!("Draft already exists for article {}", article_id);
    return Ok(substack_draft);
  }

  // Fetch the article
  let article = directus::Articles::select()
    .where_("id = ?")
    .bind(article_id)
    .fetch_one(mm.orm())
    .await?;

  if let Ok(linked_articles) = directus::RelatedArticles::select()
    .where_("articles_id = ?")
    .bind(article_id)
    .fetch_all(mm.orm())
    .await
  {
    for linked_article in linked_articles {
      create_substack_draft(mm, linked_article.related_articles_id).await?;
    }
  }

  // Export to Substack
  let draft_response =
    drafts::Request::export_from_article(mm, &article, 292604153).await?;

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
    delete_substack_draft(mm, article_id, draft_response.id).await?;
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
pub async fn delete_substack_draft(
  mm: &ModelManager,
  article_id: Uuid,
  draft_id: i64,
) -> Result<()> {
  // Delete from Substack
  drafts::Request::delete(mm.reqwest(), draft_id).await?;

  info!(
    "Deleted Substack draft {} for article {}",
    draft_id, article_id
  );

  directus::SubstackDraft::select()
    .where_("article_id = ?")
    .bind(article_id)
    .fetch_one(mm.orm())
    .await?
    .delete(mm.orm())
    .await?;

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
  let response = create_substack_draft(mm, article_id).await?;

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
