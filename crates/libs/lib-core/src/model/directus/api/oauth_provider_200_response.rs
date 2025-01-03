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
pub struct OauthProvider200Response {
	#[serde(rename = "public", skip_serializing_if = "Option::is_none")]
	pub public: Option<bool>,
	#[serde(rename = "data", skip_serializing_if = "Option::is_none")]
	pub data: Option<Box<super::OauthProvider200ResponseData>>,
}

impl OauthProvider200Response {
	pub fn new() -> OauthProvider200Response {
		OauthProvider200Response {
			public: None,
			data: None,
		}
	}
}
