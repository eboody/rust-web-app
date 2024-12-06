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
pub struct DiffDiffCollectionsInner {
	#[serde(rename = "collection", skip_serializing_if = "Option::is_none")]
	pub collection: Option<String>,
	#[serde(rename = "diff", skip_serializing_if = "Option::is_none")]
	pub diff: Option<Vec<json::Value>>,
}

impl DiffDiffCollectionsInner {
	pub fn new() -> DiffDiffCollectionsInner {
		DiffDiffCollectionsInner {
			collection: None,
			diff: None,
		}
	}
}
