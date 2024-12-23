use ormlite::Model;
use time::PrimitiveDateTime;

#[derive(Debug, Model, Clone)]
#[ormlite(table = "wp_posts")]
pub struct WpPosts {
  #[ormlite(primary_key)]
  pub wp_id: i64,
  pub title: String,
  pub date: PrimitiveDateTime,
  pub slug: String,
  pub last_modified: PrimitiveDateTime,
  pub content: String,
  pub endnotes: Option<String>,
  pub descriptor: Option<String>,
  pub summary: Option<String>,
  pub author: String,
}
