use crate::prelude::*;

#[derive(Serialize, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
  #[default]
  Newsletter,
  Podcast,
  Video,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
  #[default]
  Everyone,
  Founding,
  OnlyPaid,
  OnlyFree,
}

#[derive(Serialize, Debug)]
pub struct ByLine {
  pub id: u64,
  pub is_guest: bool,
}
