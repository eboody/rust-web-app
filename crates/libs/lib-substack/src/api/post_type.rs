use crate::prelude::*;

#[derive(Serialize, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
  #[default]
  Newsletter,
  Podcast,
  Video,
}
