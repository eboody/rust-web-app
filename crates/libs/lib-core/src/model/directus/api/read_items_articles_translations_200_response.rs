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
pub struct ReadItemsArticlesTranslations200Response {
	#[serde(rename = "data", skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<super::ItemsArticlesTranslations>>,
	#[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
	pub meta: Option<Box<super::XMetadata>>,
}

impl ReadItemsArticlesTranslations200Response {
	pub fn new() -> ReadItemsArticlesTranslations200Response {
		ReadItemsArticlesTranslations200Response {
			data: None,
			meta: None,
		}
	}
}
