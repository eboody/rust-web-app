use axum::extract::{Json, State};
use directus::trigger::Event;
use lib_core::model::{articles, Articles, ArticlesSubstackStatus};

use super::trigger;
use crate::prelude::*;

pub async fn on_item_update(
	State(mm): State<ModelManager>,
	Json(trigger): Json<trigger::Body>,
) -> Result<String> {
	dbg!("trigger: {}", &trigger);

	match trigger.event {
		Event::Article(articles::Event::Update) => {
			for article_id in trigger.keys {
				let article = Articles::select()
					.where_("id = ?")
					.bind(article_id)
					.fetch_one(mm.orm())
					.await?;

				dbg!("article: {}", &article);

				let substack_status = ArticlesSubstackStatus::select()
					.where_("articles_id = ?")
					.bind(article_id)
					.fetch_one(mm.orm())
					.await;

				if let Ok(substack_status) = substack_status {
					dbg!("substack_status: {}", &substack_status);
				} else {
					dbg!("no substack status found");
				}
			}
		}
	}

	Ok("OK".to_owned())
}
