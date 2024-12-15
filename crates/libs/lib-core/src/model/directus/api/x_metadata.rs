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
pub struct XMetadata {
	/// Returns the total item count of the collection you're querying.
	#[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
	pub total_count: Option<i32>,
	/// Returns the item count of the collection you're querying, taking the current filter/search parameters into account.
	#[serde(rename = "filter_count", skip_serializing_if = "Option::is_none")]
	pub filter_count: Option<i32>,
}

impl XMetadata {
	pub fn new() -> XMetadata {
		XMetadata {
			total_count: None,
			filter_count: None,
		}
	}
}
