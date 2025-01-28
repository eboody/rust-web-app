use crate::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use bytes::Bytes;
use lib_core::model::{self, SubstackDraft};
use lib_utils::retry::RetryableRequest;
use url::Url;

use crate::config::config;

pub struct Image {
    pub image: Bytes,
    pub post_id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub id: String,
    pub url: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub bytes: i64,
    #[serde(rename = "imageWidth")]
    pub image_width: Option<i32>,
    #[serde(rename = "imageHeight")]
    pub image_height: Option<i32>,
}

impl Response {
    fn to_ss_cover_image(&self, ss_draft_id: Uuid) -> model::SubstackCoverImage {
        model::SubstackCoverImage {
            id: self.id.clone(),
            substack_draft: ss_draft_id,
            url: self.url.clone(),
            content_type: self.content_type.clone(),
            bytes: self.bytes,
            image_width: self.image_width,
            image_height: self.image_height,
        }
    }
}

impl Image {
    pub async fn upload_to_substack(
        mm: &ModelManager,
        image_file: model::Files,
        draft_ssid: i64,
    ) -> Result<model::SubstackCoverImage> {
        let image_file_url: Url = image_file.try_into()?;

        let download_image_res = mm.reqwest().get(image_file_url).send().await?;

        let image_bytes = download_image_res.bytes().await?;

        let b64 = format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(&image_bytes)
        );

        let image_res_body = mm
            .reqwest()
            .post(format!("{}/image", &config().API_URL))
            .headers(config().HEADERS.clone())
            .json(&json!({
                "post_id": draft_ssid,
                "image": b64,
            }))
            .retry()
            .send::<Response>()
            .await?;

        let ss_draft = SubstackDraft::select()
            .where_("substack_draft_id = ?")
            .bind(draft_ssid)
            .fetch_one(mm.orm())
            .await?;

        let ss_cover_image = image_res_body.to_ss_cover_image(ss_draft.id);

        let ss_cover_image = ss_cover_image
            .insert(mm.orm())
            .on_conflict(OnConflict::do_update_on_pkey("id"))
            .await?;

        Ok(ss_cover_image)
    }
}
