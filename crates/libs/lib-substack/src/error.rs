use derive_more::derive::{Display, From};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Display, Serialize)]
pub enum Error {
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),

	#[from]
	Io(#[serde_as(as = "DisplayFromStr")] std::io::Error),

	MammothFailed(String),
}

impl std::error::Error for Error {}
