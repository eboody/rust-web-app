#![allow(unused)]
use super::response::ImageResponse;
use serde_with::Bytes;

use crate::prelude::*;

pub async fn upload_image(
  mm: &ModelManager,
  post_id: u64,
  directus_image_id: Uuid,
) -> Result<ImageResponse> {
  todo!()
}
