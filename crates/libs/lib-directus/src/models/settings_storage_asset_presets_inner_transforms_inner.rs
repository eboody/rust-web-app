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
pub struct SettingsStorageAssetPresetsInnerTransformsInner {
    /// The Sharp method name
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// A list of arguments to pass to the Sharp method
    #[serde(rename = "arguments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Option<Vec<models::SettingsStorageAssetPresetsInnerTransformsInnerArgumentsInner>>>,
}

impl SettingsStorageAssetPresetsInnerTransformsInner {
	pub fn new() -> SettingsStorageAssetPresetsInnerTransformsInner {
		SettingsStorageAssetPresetsInnerTransformsInner {
			method: None,
			arguments: None,
		}
	}
}
