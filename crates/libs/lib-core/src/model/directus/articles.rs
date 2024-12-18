use crate::prelude::*;
use directus::{Status, admin::Users};

#[derive(Debug, ormlite::Model, Serialize, Deserialize, Partial)]
#[partially(derive(Debug, Deserialize, Serialize))]
pub struct Articles {
  #[partially(omit)]
  #[ormlite(primary_key)]
  pub id: Uuid,

  #[ormlite(join_column = "author_id")]
  #[serde(skip)]
  #[partially(omit)]
  pub author: Join<Users>,
  pub author_id: Uuid,

  pub slug: Option<String>,
  pub body: Option<String>,
  pub endnotes: Option<String>,
  pub descriptor: Option<String>,
  pub title: Option<String>,
  pub subtitle: Option<String>,
  pub summary: Option<String>,

  pub status: Status,
  pub is_evergreen: bool,
  pub audience: substack::Audience,
  pub sort: Option<i32>,
  pub featured_image: Option<Uuid>,
  pub tags: Option<json::Value>,
  pub date_published: Option<Date>,
  pub issue: Option<Uuid>,
  pub section: Option<Uuid>,

  pub user_created: Option<Uuid>,
  pub date_created: Option<OffsetDateTime>,
  pub user_updated: Option<Uuid>,
  pub date_updated: Option<OffsetDateTime>,
}

impl AsRef<Articles> for Articles {
  fn as_ref(&self) -> &Articles {
    self
  }
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

#[derive(Debug, Deserialize, Display, Serialize, Clone)]
pub enum Event {
  #[serde(rename = "articles.items.update")]
  Update,
}

#[derive(Debug, Deserialize, Serialize, ormlite::Model)]
#[ormlite(table = "articles_articles")]
pub struct RelatedArticles {
  pub id: i32,
  pub articles_id: Uuid,
  pub related_articles_id: Uuid,
}
