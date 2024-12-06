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
pub struct PasswordResetRequest {
	/// One-time use JWT token that is used to verify the user.
	#[serde(rename = "token")]
	pub token: String,
	/// New password for the user.
	#[serde(rename = "password")]
	pub password: String,
}

impl PasswordResetRequest {
	pub fn new(token: String, password: String) -> PasswordResetRequest {
		PasswordResetRequest { token, password }
	}
}
