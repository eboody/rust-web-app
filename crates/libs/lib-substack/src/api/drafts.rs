use crate::{
	md_to_prosemirror, prelude::*, prose_mirror, transform_endnotes_for_substack,
	transform_to_substack_format,
};
use lib_core::model::directus;
use ormlite::types::Uuid;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::audiences::Audience;

#[derive(Serialize, Debug, Default)]
pub struct DraftRequest {
	pub audience: Audience,
	pub draft_body: DraftBody,
	pub draft_title: String,
	pub draft_subtitle: String,
	pub r#type: DraftType,
	pub draft_section_id: Option<String>,
	pub draft_bylines: Vec<DraftByline>,
	pub section_chosen: bool,
	pub draft_podcast_duration: Option<String>,
	pub draft_podcast_preview_upload_id: Option<String>,
	pub draft_podcast_upload_id: Option<String>,
	pub draft_podcast_url: String,
	pub draft_video_upload_id: Option<String>,
	pub draft_voiceover_upload_id: Option<String>,
}

#[derive(Serialize, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftType {
	#[default]
	Newsletter,
	Podcast,
	Video,
}

impl DraftRequest {
	pub async fn post(&self, client: &reqwest::Client) -> Result<DraftResponse> {
		let url = Url::parse(&format!("{}/drafts", &config().API_URL))?;

		Ok(client
			.post(url)
			.headers(config().HEADERS.clone())
			.json(self)
			.send()
			.await?
			.json::<DraftResponse>()
			.await?)
	}
}

impl TryFrom<directus::Articles> for DraftRequest {
	type Error = crate::Error;

	fn try_from(
		article: directus::Articles,
	) -> std::result::Result<Self, Self::Error> {
		Ok(Self {
			audience: Audience::Everyone,
			r#type: DraftType::Newsletter,
			draft_body: article.as_ref().try_into()?,
			draft_title: article.title.clone().unwrap_or_default(),
			draft_subtitle: article.subtitle.clone().unwrap_or_default(),
			..Default::default()
		})
	}
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DraftBody(pub String);

impl TryFrom<&directus::Articles> for DraftBody {
	type Error = crate::Error;

	fn try_from(article: &directus::Articles) -> Result<Self> {
		if let Some(content) = &article.body {
			let doc = md_to_prosemirror(content)?;
			let mut doc: prose_mirror::Node = doc.into();

			transform_to_substack_format(&mut doc);

			if let Some(endnotes) = &article.endnotes {
				debug!("->> {:<12} - endnotes: {:#?}", file!(), endnotes);

				let endnotes = md_to_prosemirror(endnotes)?;
				let mut endnotes = transform_endnotes_for_substack(&endnotes.into());
				doc.content.as_mut().unwrap().append(&mut endnotes);
			}

			Ok(DraftBody(json::to_string(&doc)?))
		} else {
			Err(Error::NoArticleContent)
		}
	}
}

#[derive(Serialize, Debug)]
pub struct DraftByline {
	pub id: u64,
	pub is_guest: bool,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Serialize)]
pub struct DraftResponse {
	pub id: i64,
	#[serde(rename = "type")]
	pub type_: DraftType,
	pub draft_title: String,
	pub draft_subtitle: String,
	pub audience: Audience,
	pub section_chosen: bool,
	pub publication_id: i64,
	pub word_count: i64,
	pub draft_body: String,
	#[serde(with = "time::serde::rfc3339")]
	pub draft_created_at: OffsetDateTime,
	#[serde(with = "time::serde::rfc3339")]
	pub draft_updated_at: OffsetDateTime,
	pub uuid: Uuid,
	// i might need to add more fields here
}
