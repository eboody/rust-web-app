use super::Json;
use ormlite::model::{Join, JoinMeta, Model};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, ormlite::Model, Default)]
pub struct Articles {
	#[ormlite(primary_key)]
	pub id: Uuid,
	pub status: String,
	pub sort: Option<i32>,
	pub user_created: Option<Uuid>,
	pub date_created: Option<OffsetDateTime>,
	pub user_updated: Option<Uuid>,
	pub date_updated: Option<OffsetDateTime>,
	pub featured_image: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
pub struct ArticlesDirectusUsers {
	#[ormlite(primary_key)]
	pub id: i32,
	pub articles_id: Option<i32>,
	pub directus_users_id: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
pub struct ArticlesTags {
	#[ormlite(primary_key)]
	pub id: i32,
	pub articles_id: Option<i32>,
	pub tags_id: Option<i32>,
}

#[derive(Debug, Model)]
pub struct ArticlesTranslations {
	#[ormlite(primary_key)]
	pub id: Uuid,
	#[ormlite(join_column = "articles_id")]
	pub article: Join<Articles>,
	pub articles_id: Option<Uuid>,
	pub languages_code: Option<String>,
	pub content: Option<String>,
	pub title: Option<String>,
	pub summary: Option<String>,
	pub descriptor: Option<String>,
}
