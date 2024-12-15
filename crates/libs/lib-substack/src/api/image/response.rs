#![allow(unused)]
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};
use url::Url;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct ImageResponse {
  #[serde_as(as = "DisplayFromStr")]
  pub id: u64,
  pub url: Url,
  #[serde_as(as = "DisplayFromStr")]
  pub content_type: mime::Mime,
  pub bytes: usize,
  pub image_width: u32,
  pub image_height: u32,
}
