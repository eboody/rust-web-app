/*
 * AnythingLLM Developer API
 *
 * API endpoints that enable programmatic reading, writing, and updating of your AnythingLLM instance. UI supplied by Swagger.io.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1SystemRemoveDocumentsDeleteRequest {
	#[serde(rename = "names", skip_serializing_if = "Option::is_none")]
	pub names: Option<Vec<String>>,
}

impl V1SystemRemoveDocumentsDeleteRequest {
	pub fn new() -> V1SystemRemoveDocumentsDeleteRequest {
		V1SystemRemoveDocumentsDeleteRequest { names: None }
	}
}
