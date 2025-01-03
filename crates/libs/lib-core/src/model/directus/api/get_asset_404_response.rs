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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAsset404Response {
	#[serde(rename = "error", skip_serializing_if = "Option::is_none")]
	pub error: Option<Box<super::GetAsset404ResponseError>>,
}

impl GetAsset404Response {
	pub fn new() -> GetAsset404Response {
		GetAsset404Response { error: None }
	}
}
