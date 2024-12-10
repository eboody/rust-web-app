use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};
use derive_more::{From, derive::Display};
use lib_core::model;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use std::sync::Arc;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	ReqStampNotInReqExt,

	#[from]
	Model(model::Error),

	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] json::Error),
}

impl AsRef<Error> for Error {
	fn as_ref(&self) -> &Error {
		self
	}
}

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

impl std::error::Error for Error {}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use Error::*;

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
pub enum ClientError {
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
	SERVICE_ERROR,
}

impl AsRef<ClientError> for ClientError {
	fn as_ref(&self) -> &ClientError {
		self
	}
}

impl std::fmt::Display for ClientError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:?}", self)
	}
}
