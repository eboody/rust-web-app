use derive_more::derive::Display;
use ormlite::model::{Join, JoinMeta};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, ormlite::Model)]
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
	pub author: Uuid,
	pub date_published: Option<Date>,
	pub content: Option<String>,
	pub title: Option<String>,
	pub sub_title: Option<String>,
	pub summary: Option<String>,
	pub descriptor: Option<String>,
	pub endnotes: Option<String>,
	pub slug: Option<String>,
	pub substack_status: Option<Uuid>,
	//#[ormlite(join_column = "substack_status")]
	//pub substack_statuses: Join<ArticlesSubstackStatus>,
}

#[derive(Debug, ormlite::Model)]
pub struct ArticlesTags {
	#[ormlite(primary_key)]
	pub id: i32,
	pub articles_id: i32,
	pub tags_id: i32,
}

#[derive(Debug, ormlite::Model)]
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
	pub endnotes: Option<String>,
	pub slug: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "articles_substack_status")]
pub struct ArticlesSubstackStatus {
	pub id: Uuid,
	pub articles_id: Uuid,
	pub substack_id: i64,
	pub status: ArticleStatus,
	pub sort: Option<i32>,
	pub date_updated: OffsetDateTime,
	pub message: Option<String>,
}

#[derive(Debug, ormlite::Enum)]
pub enum ArticleStatus {
	Draft,
	Published,
}

#[derive(Debug, Deserialize, Display, Serialize, Clone)]
pub enum Event {
	#[serde(rename = "articles.items.update")]
	Update,
}
