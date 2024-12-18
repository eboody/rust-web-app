use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, Default, Deserialize, ormlite::Enum)]
#[serde(rename_all = "snake_case")]
pub enum Type {
  #[default]
  Newsletter,
  Podcast,
  Video,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, ormlite::Enum)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
  #[default]
  Everyone,
  Founding,
  OnlyPaid,
  OnlyFree,
}

#[derive(Clone, Serialize, Debug)]
pub struct ByLine {
  pub id: i64,
  pub is_guest: bool,
}
