//use crate::{api::drafts, prelude::*};
//use lib_core::model::directus::Articles;
//use ormlite::types::Uuid;
//
//pub async fn export_draft(mm: &ModelManager, article_id: Uuid) -> Result<()> {
//  let article = Articles::select()
//    .where_("id = ?")
//    .bind(article_id)
//    .fetch_one(mm.orm())
//    .await?;
//
//  let draft_response: drafts::Response =
//    drafts::Request::export_from_article(mm.reqwest(), &article, 292604153).await?;
//
//  // Update the article with the new draft status
//  article
//    .update_partial()
//    .date_updated(Some(OffsetDateTime::now_utc()))
//    .update(mm.orm())
//    .await?;
//
//  Ok(())
//}
