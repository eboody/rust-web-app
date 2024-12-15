pub use crate::post::{Audience, ByLine, Type};
use crate::{
  md_to_prosemirror, prelude::*, prose_mirror, transform_endnotes_for_substack,
  transform_to_substack_format,
};
use lib_core::model::directus;
use ormlite::types::Uuid;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Serialize, Debug, Default)]
pub struct Request {
  pub audience: Audience,
  pub draft_body: Body,
  pub draft_title: String,
  pub draft_subtitle: String,
  pub r#type: Type,
  pub draft_section_id: Option<String>,
  pub draft_bylines: Vec<ByLine>,
  pub section_chosen: bool,
  pub draft_podcast_duration: Option<String>,
  pub draft_podcast_preview_upload_id: Option<String>,
  pub draft_podcast_upload_id: Option<String>,
  pub draft_podcast_url: String,
  pub draft_video_upload_id: Option<String>,
  pub draft_voiceover_upload_id: Option<String>,
}

impl Request {
  pub async fn post(&self, client: &reqwest::Client) -> Result<Response> {
    let url = Url::parse(&format!("{}/drafts", &config().API_URL))?;

    Ok(
      client
        .post(url)
        .headers(config().HEADERS.clone())
        .json(self)
        .send()
        .await?
        .json::<Response>()
        .await?,
    )
  }
}

impl TryFrom<directus::Articles> for Request {
  type Error = crate::Error;

  fn try_from(
    article: directus::Articles,
  ) -> std::result::Result<Self, Self::Error> {
    Ok(Self {
      audience: Audience::Everyone,
      r#type: Type::Newsletter,
      draft_body: article.as_ref().try_into()?,
      draft_title: article.title.clone().unwrap_or_default(),
      draft_subtitle: article.subtitle.clone().unwrap_or_default(),
      ..Default::default()
    })
  }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Body(pub String);

impl TryFrom<&directus::Articles> for Body {
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

      Ok(Body(json::to_string(&doc)?))
    } else {
      Err(Error::NoArticleContent)
    }
  }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Serialize)]
pub struct Response {
  pub id: i64,
  #[serde(rename = "type")]
  pub type_: Type,
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
}
