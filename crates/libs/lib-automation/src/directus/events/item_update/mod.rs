use axum::extract::{Json, State};
use lib_core::model::directus::{
  Articles, ArticlesSubstackStatus, ArticlesUpdate, SubsctackArticleStatus, articles,
};
use lib_substack::draft;

use crate::{directus::trigger, prelude::*};

pub async fn on_item_update(
  State(mm): State<ModelManager>,
  Json(trigger): Json<trigger::Body>,
) -> Result<String> {
  if let Some(event) = &trigger.event {
    match event {
      trigger::Event::Articles(articles_event) => {
        on_article_event(&mm, &trigger, articles_event).await?;
      }
    }
  }

  Ok("OK".to_owned())
}

pub async fn on_article_event(
  _mm: &ModelManager,
  trigger: &trigger::Body,
  event: &articles::Event,
) -> Result<String> {
  match event {
    articles::Event::Update => {
      if let Some(payload) = trigger.clone().payload {
        let payload = json::from_value::<ArticlesUpdate>(payload)?;
        debug!("->> {:<12} - payload: {:#?}", file!(), payload);
      }

      //if let Some(status) = &payload.status {
      //	if *status == Status::Published {
      //		on_articles_publish(&mm, trigger).await?;
      //	}
      //}
      //on_article_update(&mm, trigger).await?;
    }
  }

  Ok("OK".to_owned())
}

//pub trait ToSubstack {
//	fn to_substack(&self) -> Result<String>;
//}
//
//impl ToSubstack for Articles {
//	fn to_substack(&self) -> Result<String> {
//		if let Some(content) = &self.content {
//			let doc = md_to_prosemirror(content)?;
//			let mut doc: lib_substack::prose_mirror::Node = doc.into();
//
//			transform_to_substack_format(&mut doc);
//
//			if let Some(endnotes) = &self.endnotes {
//				debug!("->> {:<12} - endnotes: {:#?}", file!(), endnotes);
//
//				let endnotes = md_to_prosemirror(endnotes)?;
//				let mut endnotes = transform_endnotes_for_substack(&endnotes.into());
//				doc.content.as_mut().unwrap().append(&mut endnotes);
//			}
//
//			Ok(json::to_string(&doc)?)
//		} else {
//			Err(Error::Substack(lib_substack::Error::NoArticleContent))
//		}
//	}
//}

#[allow(unused)]
pub async fn on_articles_publish(
  mm: &ModelManager,
  trigger: trigger::Body,
) -> Result<String> {
  //TODO: verify that the article isn't already on substack
  for article_id in trigger.keys {
    let article = Articles::select()
      .where_("id = ?")
      .bind(article_id)
      .fetch_one(mm.orm())
      .await?;

    let substack_draft_response = draft::Request {
      audience: draft::Audience::Everyone,
      r#type: draft::Type::Newsletter,
      draft_body: article.as_ref().try_into()?,
      draft_title: article.title.clone().unwrap_or_default(),
      draft_subtitle: article.subtitle.clone().unwrap_or_default(),
      ..Default::default()
    }
    .post(mm.reqwest())
    .await?;

    let mut internal_ss_status = ArticlesSubstackStatus::select()
      .where_("articles_id = ?")
      .bind(article_id)
      .fetch_one(mm.orm())
      .await
      .ok();

    if internal_ss_status.is_none() {
      internal_ss_status = ArticlesSubstackStatus {
        id: Uuid::new_v4(),
        articles_id: article_id,
        substack_id: substack_draft_response.id,
        status: SubsctackArticleStatus::Draft,
        sort: None,
        date_updated: OffsetDateTime::now_utc(),
        message: "Successfully exported to substack".to_owned().into(),
      }
      .insert(mm.orm())
      .await
      .ok();
    }

    article
      .update_partial()
      .substack_status(internal_ss_status.map(|s| s.id))
      .date_updated(Some(OffsetDateTime::now_utc()))
      .update(mm.orm())
      .await?;
  }

  Ok("OK".to_owned())
}
