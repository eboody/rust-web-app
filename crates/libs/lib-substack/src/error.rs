use derive_more::derive::{Display, From};
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Display, Serialize)]
pub enum Error {
  #[from]
  SerdeJson(#[serde_as(as = "DisplayFromStr")] json::Error),

  #[from]
  Io(#[serde_as(as = "DisplayFromStr")] std::io::Error),

  MammothFailed(String),

  #[from]
  Ormlite(#[serde_as(as = "DisplayFromStr")] ormlite::CoreError),

  ProseMirrorFailed,

  NoArticleContent,
  NoArticleTitle,
  FailedToFetchArticle,

  #[from]
  Request(#[serde_as(as = "DisplayFromStr")] reqwest::Error),

  #[from]
  UrlParse(#[serde_as(as = "DisplayFromStr")] url::ParseError),

  MissingField(String),
}

impl std::error::Error for Error {}
