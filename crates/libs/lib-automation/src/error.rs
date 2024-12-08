use std::sync::Arc;

use axum::response::{IntoResponse, Response};
use derive_more::From;
use reqwest::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
	MissingEnv(&'static str),
	WrongFormat(String),
	#[from]
	Request(#[serde_as(as = "DisplayFromStr")] reqwest::Error),
	#[from]
	Ormlite(#[serde_as(as = "DisplayFromStr")] ormlite::Error),

	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(Arc::new(self));

		response
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

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Error::SerdeJson(e)
	}
}
