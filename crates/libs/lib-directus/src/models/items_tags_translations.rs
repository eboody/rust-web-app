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
pub struct ItemsTagsTranslations {
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<i32>,
	#[serde(
		rename = "tags_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub tags_id: Option<Option<Box<models::ItemsTagsTranslationsTagsId>>>,
	#[serde(
		rename = "languages_code",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub languages_code:
		Option<Option<Box<models::ItemsEbooksTranslationsLanguagesCode>>>,
}

impl ItemsTagsTranslations {
	pub fn new() -> ItemsTagsTranslations {
		ItemsTagsTranslations {
			id: None,
			tags_id: None,
			languages_code: None,
		}
	}
}
