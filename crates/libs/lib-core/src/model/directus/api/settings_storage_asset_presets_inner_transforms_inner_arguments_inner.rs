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
pub struct SettingsStorageAssetPresetsInnerTransformsInnerArgumentsInner {
	/// A JSON representation of the argument value
	#[serde(rename = "argument", skip_serializing_if = "Option::is_none")]
	pub argument: Option<String>,
}

impl SettingsStorageAssetPresetsInnerTransformsInnerArgumentsInner {
	pub fn new() -> SettingsStorageAssetPresetsInnerTransformsInnerArgumentsInner {
		SettingsStorageAssetPresetsInnerTransformsInnerArgumentsInner {
			argument: None,
		}
	}
}
