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
pub struct PromoteContentVersionRequest {
	/// Hash of the main version of the item to be promoted.
	#[serde(rename = "mainHash", skip_serializing_if = "Option::is_none")]
	pub main_hash: Option<String>,
	/// Optional array of field names of which the values are to be promoted.
	#[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
	pub fields: Option<String>,
}

impl PromoteContentVersionRequest {
	pub fn new() -> PromoteContentVersionRequest {
		PromoteContentVersionRequest {
			main_hash: None,
			fields: None,
		}
	}
}