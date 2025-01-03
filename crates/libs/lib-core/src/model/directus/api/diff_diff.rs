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
pub struct DiffDiff {
	#[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
	pub collections: Option<Vec<super::DiffDiffCollectionsInner>>,
	#[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
	pub fields: Option<Vec<super::DiffDiffFieldsInner>>,
	#[serde(rename = "relations", skip_serializing_if = "Option::is_none")]
	pub relations: Option<Vec<super::DiffDiffRelationsInner>>,
}

impl DiffDiff {
	pub fn new() -> DiffDiff {
		DiffDiff {
			collections: None,
			fields: None,
			relations: None,
		}
	}
}
