#![allow(unused_imports)]
use std::str::FromStr;

use crate::prelude::*;
use axum::{Json, Router, extract::State, routing::post};
use directus::{
  on_item_update,
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
  //

  tokio::spawn(async move {
    let mm = mm_clone;

    let article = Articles::select()
      .where_("articles.id = ?")
      .bind(Uuid::from_str("078cd2db-28a3-42cd-95a2-e67975009846").unwrap())
      .join(Articles::author())
      .fetch_one(mm.orm())
      .await
      .unwrap();

    if article.tags.is_none() {
      directus::tasks::add_tags(&mm, &article).await.unwrap();
    }

    if article.section.is_none() {
      directus::tasks::select_section(&mm, &article)
        .await
        .unwrap();
    }

    if article.subtitle.is_none() {
      directus::tasks::add_subtitle(&mm, &article).await.unwrap();
    }

    //directus::tasks::handle_images(&mm, &article).await.unwrap();
    directus::tasks::handle_videos(&mm, &article).await.unwrap();
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

  lib_substack::export_draft(&mm, *article_id).await?;

  Ok("OK".to_owned())
}
