use crate::prelude::*;

#[derive(Clone, Debug, ormlite::Model)]
pub struct Issues {
  #[ormlite(primary_key)]
  pub id: Uuid,
  pub sort: Option<i32>,
  pub year: String,
  pub season: String,
  pub volume: String,
  pub number: String,
}
