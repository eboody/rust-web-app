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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemsTagsTranslationsTagsId {
	Integer(i32),
	ItemsTags(Box<super::ItemsTags>),
}

impl Default for ItemsTagsTranslationsTagsId {
	fn default() -> Self {
		Self::Integer(Default::default())
	}
}