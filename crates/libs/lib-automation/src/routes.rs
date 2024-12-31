#![allow(unused_imports)]
use std::str::FromStr;

use crate::prelude::*;
use axum::{Json, Router, extract::State, routing::post};
use directus::{
  on_item_update, tasks,
  trigger::{self, Event},
};
use json::json;
use lib_core::model::directus::{Articles, Collection};

pub fn routes(mm: ModelManager) -> Router {
  let mm_clone = mm.clone();
  //let fake_payload = trigger::Body {
  //	event: Event::Articles(articles::Event::Update),
  //	collection: Collection::Articles,
  //	keys: vec![Uuid::from_str("e3e09466-9829-4f9b-9f8b-e468ad291605").unwrap()],
  //	payload: json!({
  //		"status": "published",
  //	}),
  //};
  //
  //tokio::spawn(async move {
  //	let res = on_item_update(State(mm_clone), Json(fake_payload)).await;
  //	debug!("->> {:<12} - res: {:#?}", file!(), res);
  //});

  tokio::spawn(async move {
    let _mm = mm_clone;

    //tasks::sync_sections(&mm).await.unwrap();
    //
    //let article = Articles::select()
    //  .where_("articles.id = ?")
    //  .bind(Uuid::from_str("ed403190-6c5d-4e34-8dae-774f098ced61").unwrap())
    //  .join(Articles::author())
    //  .fetch_one(mm.orm())
    //  .await
    //  .unwrap();
    //
    //let res = tasks::create_substack_draft(&mm, article.id).await;
    //
    //if let Ok(draft) = res {
    //  tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    //
    //  let related_articles = lib_core::model::directus::RelatedArticles::select()
    //    .where_("articles_id = ?")
    //    .bind(article.id)
    //    .fetch_all(mm.orm())
    //    .await
    //    .expect("Failed to fetch related articles");
    //
    //  for related_article in related_articles {
    //    let article = lib_core::model::directus::Articles::select()
    //      .where_("id = ?")
    //      .bind(related_article.related_articles_id)
    //      .fetch_one(mm.orm())
    //      .await
    //      .unwrap();
    //
    //    let related_draft = lib_core::model::directus::SubstackDraft::select()
    //      .where_("articles_id = ?")
    //      .bind(article.id)
    //      .fetch_one(mm.orm())
    //      .await
    //      .expect("Failed to fetch related draft");
    //
    //    tasks::delete_substack_draft(
    //      &mm,
    //      article.id,
    //      related_draft.substack_draft_id,
    //    )
    //    .await
    //    .expect("Failed to delete related draft");
    //  }
    //
    //  tasks::delete_substack_draft(&mm, article.id, draft.substack_draft_id)
    //    .await
    //    .unwrap();
    //}

    //
    //if article.tags.is_none() {
    //  directus::tasks::add_tags(&mm, &article).await.unwrap();
    //}
    //
    //if article.section.is_none() {
    //  directus::tasks::select_section(&mm, &article)
    //    .await
    //    .unwrap();
    //}
    //
    //if article.subtitle.is_none() {
    //  directus::tasks::add_subtitle(&mm, &article).await.unwrap();
    //}

    //directus::tasks::handle_images(&mm, &article).await.unwrap();
    //directus::tasks::handle_videos(&mm, &article).await.unwrap();
  });

  Router::new()
    .route("/substack_export", post(test))
    //.route("/on_file_upload", post(directus::on_file_upload))
    .route("/item_update", post(directus::on_item_update))
    .route("/check", post(|| async { "OK" }))
    .with_state(mm)
}

async fn test(
  State(mm): State<ModelManager>,
  Json(trigger): Json<directus::trigger::Request>,
) -> Result<String> {
  let trigger = trigger.body;
  println!("Exporting article to substack");
  let Some(article_id) = trigger.keys.first() else {
    return Err(Error::NoKeyInTrigger(trigger.clone()));
  };

  tasks::create_substack_draft(&mm, *article_id).await?;

  Ok("OK".to_owned())
}
