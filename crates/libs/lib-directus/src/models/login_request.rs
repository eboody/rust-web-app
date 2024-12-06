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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
	/// Email address of the user you're retrieving the access token for.
	#[serde(rename = "email")]
	pub email: String,
	/// Password of the user.
	#[serde(rename = "password")]
	pub password: String,
	/// Whether to retrieve the refresh token in the JSON response, or in a httpOnly cookie.
	#[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
	pub mode: Option<Mode>,
	/// The user's one-time-password (if MFA is enabled).
	#[serde(rename = "otp", skip_serializing_if = "Option::is_none")]
	pub otp: Option<String>,
}

impl LoginRequest {
	pub fn new(email: String, password: String) -> LoginRequest {
		LoginRequest {
			email,
			password,
			mode: None,
			otp: None,
		}
	}
}
/// Whether to retrieve the refresh token in the JSON response, or in a httpOnly cookie.
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Mode {
	#[serde(rename = "json")]
	Json,
	#[serde(rename = "cookie")]
	Cookie,
	#[serde(rename = "session")]
	Session,
}

impl Default for Mode {
	fn default() -> Mode {
		Self::Json
	}
}
