use serde::{Deserialize, Serialize};

#[derive(
  Default, Debug, Deserialize, Serialize, Clone, ormlite::Enum, PartialEq,
)]
#[serde(rename_all = "snake_case")]
pub enum Status {
  Published,
  UnderReview,
  Draft,
  #[default]
  Archived,
}
