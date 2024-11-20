use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
	id: String,
	image: String,
	status: Status,
	user_created: String,
	date_created: OffsetDateTime,
	user_updated: String,
	date_updated: OffsetDateTime,
	author: u32,

	translations: Vec<ArticlesTranslations>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticlesTranslations {
	article_id: String,
	language_code: String,
	title: String,
	summary: String,
	content: String,
}

//pub struct ArticleManager {}
//
//impl ArticleManager {
//	pub fn new() -> Self {
//		ArticleManager {}
//	}
//
//	pub fn get_article(&self, id: &str) -> Article {
//		Article {
//			id: id.to_string(),
//			image: "image".to_string(),
//			status: Status::Published,
//			user_created: "user_created".to_string(),
//			date_created: OffsetDateTime::now_utc(),
//			user_updated: "user_updated".to_string(),
//			date_updated: OffsetDateTime::now_utc(),
//			author: 1,
//			translations: vec![],
//		}
//	}
//}
