use axum::response::IntoResponse;
use derive_more::From;
use lib_core::model;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Serialize)]
pub enum Error {
  #[from]
  Model(model::Error),

  #[from]
  Automation(lib_automation::Error),
}

// region:    --- Error Boilerplate
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
  fn into_response(self) -> axum::response::Response {
    // Create a placeholder Axum reponse.
    let mut response = axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the Error into the reponse.
    response.extensions_mut().insert(std::sync::Arc::new(self));

    response
  }
}
// endregion: --- Error Boilerplate
