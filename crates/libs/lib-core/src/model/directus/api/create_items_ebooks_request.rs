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
pub enum CreateItemsEbooksRequest {
	Array(Vec<super::ItemsEbooks>),
	ItemsEbooks(Box<super::ItemsEbooks>),
}

impl Default for CreateItemsEbooksRequest {
	fn default() -> Self {
		Self::Array(Default::default())
	}
}
