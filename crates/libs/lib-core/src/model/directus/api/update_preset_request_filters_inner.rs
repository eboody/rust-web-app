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
pub struct UpdatePresetRequestFiltersInner {
	#[serde(rename = "field", skip_serializing_if = "Option::is_none")]
	pub field: Option<String>,
	#[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
	pub operator: Option<String>,
	#[serde(rename = "value", skip_serializing_if = "Option::is_none")]
	pub value: Option<i32>,
}

impl UpdatePresetRequestFiltersInner {
	pub fn new() -> UpdatePresetRequestFiltersInner {
		UpdatePresetRequestFiltersInner {
			field: None,
			operator: None,
			value: None,
		}
	}
}
