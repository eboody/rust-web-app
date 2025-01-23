use crate::prelude::*;

#[derive(Debug, ormlite::Model)]
pub struct SubstackCoverImage {
    pub id: String,
    pub substack_draft: Uuid,
    pub url: String,
    pub content_type: String,
    pub bytes: i64,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
}
