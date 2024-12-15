use crate::prelude::*;
use directus::{Status, admin::Users};

#[derive(Debug, ormlite::Model)]
pub struct Articles {
  #[ormlite(primary_key)]
  pub id: Uuid,
  pub status: Status,
  pub sort: Option<i32>,
  pub user_created: Option<Uuid>,
  pub date_created: Option<OffsetDateTime>,
  pub user_updated: Option<Uuid>,
  pub date_updated: Option<OffsetDateTime>,
  pub featured_image: Option<Uuid>,
  pub tags: Option<json::Value>,
  pub author_id: Uuid,
  #[ormlite(join_column = "author_id")]
  pub author: Join<Users>,
  pub date_published: Option<Date>,
  pub issue: Option<Uuid>,
  pub is_evergreen: bool,
  pub slug: Option<String>,
  pub body: Option<String>,
  pub endnotes: Option<String>,
  pub descriptor: Option<String>,
  pub summary: Option<String>,
  pub title: Option<String>,
  pub subtitle: Option<String>,
  pub substack_status: Option<Uuid>,
  pub section: Option<Uuid>,
  pub audience: substack::Audience,
}

impl AsRef<Articles> for Articles {
  fn as_ref(&self) -> &Articles {
    self
  }
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
pub struct ArticlesFiles {
  #[ormlite(primary_key)]
  pub id: i32,
  pub articles_id: Uuid,
  pub directus_files_id: Uuid,
  pub caption: Option<String>,
  pub figure: Option<String>,
  pub url: Option<String>,
  pub mimetype: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "articles_substack_status")]
pub struct ArticlesSubstackStatus {
  pub id: Uuid,
  pub articles_id: Uuid,
  pub substack_id: i64,
  pub status: SubsctackArticleStatus,
  pub sort: Option<i32>,
  pub date_updated: OffsetDateTime,
  pub message: Option<String>,
}

#[derive(Debug, ormlite::Enum)]
pub enum SubsctackArticleStatus {
  Draft,
  Published,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticlesUpdate {
  pub status: Option<Status>,
  pub sort: Option<i32>,
  pub user_created: Option<Uuid>,
  pub date_created: Option<OffsetDateTime>,
  pub user_updated: Option<Uuid>,
  pub date_updated: Option<OffsetDateTime>,
  pub featured_image: Option<Uuid>,
  pub author: Option<Uuid>,
  pub date_published: Option<Date>,
  pub content: Option<String>,
  pub title: Option<String>,
  pub subtitle: Option<String>,
  pub summary: Option<String>,
  pub descriptor: Option<String>,
  pub endnotes: Option<String>,
  pub slug: Option<String>,
  pub substack_status: Option<Uuid>,
}

#[derive(Debug, Deserialize, Display, Serialize, Clone)]
pub enum Event {
  #[serde(rename = "articles.items.update")]
  Update,
}

#[derive(Debug, Deserialize, Serialize, ormlite::Model)]
pub struct ArticlesArticles {
  id: Uuid,
  articles_id: Uuid,
  related_articles_id: Uuid,
}
