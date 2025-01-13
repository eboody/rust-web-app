use crate::web;
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use derive_more::From;
use lib_auth::{pwd, token};
use lib_core::model;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use std::sync::Arc;
use tracing::debug;

#[allow(unused)]
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
#[serde(tag = "type", content = "data")]
pub enum Error {
  // -- Modules
  #[from]
  Model(model::Error),
  #[from]
  Pwd(pwd::Error),
  #[from]
  Token(token::Error),

  // -- External Modules
  #[from]
  SerdeJson(#[serde_as(as = "DisplayFromStr")] json::Error),
}

// region:    --- From rpc-router::Error

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
  fn into_response(self) -> Response {
    debug!("{:<12} - model::Error {self:?}", "INTO_RES");

    // Create a placeholder Axum reponse.
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the Error into the reponse.
    response.extensions_mut().insert(Arc::new(self));

    response
  }
}
// endregion: --- Axum IntoResponse

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
// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
#[allow(unused)]
impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    use web::Error::*; // TODO: should change to `use web::Error as E`

    match self {
      // -- Model
      Model(model::Error::EntityNotFound { entity, id }) => {
        (StatusCode::BAD_REQUEST, ClientError::ENTITY_NOT_FOUND {
          entity,
          id: *id,
        })
      }

      // -- Fallback.
      _ => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ClientError::SERVICE_ERROR,
      ),
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum ClientError {
  LOGIN_FAIL,
  NO_AUTH,
  ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

  RPC_REQUEST_INVALID(String),
  RPC_REQUEST_METHOD_UNKNOWN(String),
  RPC_PARAMS_INVALID(String),

  SERVICE_ERROR,
}
// endregion: --- Client Error
