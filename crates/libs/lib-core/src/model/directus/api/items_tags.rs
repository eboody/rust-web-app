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
pub struct ItemsTags {
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<i32>,
	#[serde(
		rename = "value",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub value: Option<Option<String>>,
	#[serde(
		rename = "translations",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub translations: Option<Option<Vec<super::ItemsTagsTranslationsInner>>>,
}

impl ItemsTags {
	pub fn new() -> ItemsTags {
		ItemsTags {
			id: None,
			value: None,
			translations: None,
		}
	}
}
