use crate::{prelude::*, view::Toast};
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Reqwest(reqwest::Error),
  HttpStatusCode(reqwest::StatusCode),
  Ormlite(ormlite::CoreError),
}

impl Render for Error {
  fn render(&self) -> Markup {
    html! {
      (match self {
        Error::Reqwest(e) => Toast::Error { text: e.to_string() },
        Error::HttpStatusCode(e) => Toast::Error { text: e.to_string() },
        Error::Ormlite(e) => Toast::Error { text: e.to_string() },
      })
    }
  }
}

impl From<reqwest::Error> for Error {
  fn from(e: reqwest::Error) -> Self {
    Error::Reqwest(e)
  }
}

impl From<reqwest::StatusCode> for Error {
  fn from(e: reqwest::StatusCode) -> Self {
    Error::HttpStatusCode(e)
  }
}
impl From<Error> for axum::Error {
  fn from(e: Error) -> Self {
    axum::Error::new(e)
  }
}

impl From<ormlite::CoreError> for Error {
  fn from(e: ormlite::CoreError) -> Self {
    Error::Ormlite(e)
  }
}

impl core::fmt::Display for Error {
  fn fmt(
    &self,
    fmt: &mut core::fmt::Formatter,
  ) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    match self {
      Error::Reqwest(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
      }
      Error::HttpStatusCode(status) => {
        (status, "HTTP error encountered").into_response()
      }
      Error::Ormlite(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
      }
    }
  }
}
