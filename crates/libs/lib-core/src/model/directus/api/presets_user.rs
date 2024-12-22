/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

/// PresetsUser : The unique identifier of the user to whom this collection preset applies.
/// The unique identifier of the user to whom this collection preset applies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PresetsUser {
	String(String),
	Users(Box<super::Users>),
}

impl Default for PresetsUser {
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