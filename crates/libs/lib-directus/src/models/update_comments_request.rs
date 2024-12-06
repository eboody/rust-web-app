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
pub struct UpdateCommentsRequest {
	#[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
	pub keys: Option<Vec<String>>,
	#[serde(rename = "data", skip_serializing_if = "Option::is_none")]
	pub data: Option<Box<models::UpdateCommentsRequestData>>,
}

impl UpdateCommentsRequest {
	pub fn new() -> UpdateCommentsRequest {
		UpdateCommentsRequest {
			keys: None,
			data: None,
		}
	}
}
