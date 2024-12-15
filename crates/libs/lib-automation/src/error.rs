use std::sync::Arc;
use url::Url;
use uuid::Uuid;

use axum::response::{IntoResponse, Response};
use derive_more::From;
use lib_core::model::directus::UploadFilePayload;
use reqwest::StatusCode;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use tracing::error;

use crate::directus;

pub type Result<T> = core::result::Result<T, Error>;
pub type BoxResult<T> = core::result::Result<T, Box<Error>>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
  #[from]
  Io(#[serde_as(as = "DisplayFromStr")] std::io::Error),
  MissingEnv(&'static str),
  WrongFormat(String),
  #[from]
  Request(#[serde_as(as = "DisplayFromStr")] reqwest::Error),

  #[from]
  Ormlite(#[serde_as(as = "DisplayFromStr")] ormlite::Error),

  #[from]
  Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

  SerdeJson(#[serde_as(as = "DisplayFromStr")] json::Error),

  #[from]
  Substack(lib_substack::Error),

  NoKeyInTrigger(directus::trigger::Body),

  NoTitleInArticle(Uuid),
  NoContentInArticle(Uuid),
  NoSlugInArticle(Uuid),

  #[from]
  Uuid(#[serde_as(as = "DisplayFromStr")] uuid::Error),

  UnknownMimeType(UploadFilePayload),
  UnknownContentType(UploadFilePayload),
  UnknownFolderContentType(UploadFilePayload),

  #[from]
  Url(#[serde_as(as = "DisplayFromStr")] url::ParseError),

  NoLastPathSegment(String),
  NoPathSegments(String),
  NoFileExtension(String),

  FailedToUploadImage(Url),

  #[from]
  Regex(#[serde_as(as = "DisplayFromStr")] regex::Error),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
    error!("->> {:<12} - self: {:#?}", file!(), self);

    response.extensions_mut().insert(Arc::new(self));

    response
  }
}

impl core::fmt::Display for Error {
  fn fmt(
    &self,
    fmt: &mut core::fmt::Formatter,
  ) -> core::result::Result<(), core::fmt::Error> {
    error!("->> {:<12} - self: {:#?}", file!(), self);
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl From<json::Error> for Error {
  fn from(e: json::Error) -> Self {
    Error::SerdeJson(e)
  }
}

impl From<Box<Error>> for Error {
  fn from(e: Box<Error>) -> Self {
    *e
  }
}
