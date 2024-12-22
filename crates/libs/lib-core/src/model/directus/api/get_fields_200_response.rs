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
pub struct GetFields200Response {
	#[serde(rename = "data", skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<super::Fields>>,
}

impl GetFields200Response {
	pub fn new() -> GetFields200Response {
		GetFields200Response { data: None }
	}
}