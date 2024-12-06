/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FilesUploadedBy : Who uploaded the file.
/// Who uploaded the file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesUploadedBy {
	String(String),
	Users(Box<models::Users>),
}

impl Default for FilesUploadedBy {
	fn default() -> Self {
		Self::String(Default::default())
	}
}
/// Status of the user.
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Status {
	#[serde(rename = "active")]
	Active,
	#[serde(rename = "invited")]
	Invited,
	#[serde(rename = "draft")]
	Draft,
	#[serde(rename = "suspended")]
	Suspended,
	#[serde(rename = "deleted")]
	Deleted,
}

impl Default for Status {
	fn default() -> Status {
		Self::Active
	}
}
