/*
 * Facebook Marketing API
 *
 * This is a generated connector for [Facebook Marketing API v12.0](https://developers.facebook.com/docs/marketing-apis) OpenAPI specification.  Facebook is an American online social media and social networking service owned by Facebook, Inc.Facebook Marketing  APIs are a collection of Graph API endpoints that can be used to help you advertise on Facebook.
 *
 * The version of the OpenAPI document: v12.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AdUpdateDraftAdgroupId : The ID of the draft ad
/// The ID of the draft ad
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdUpdateDraftAdgroupId {
	String(String),
	Integer(i32),
}

impl Default for AdUpdateDraftAdgroupId {
	fn default() -> Self {
		Self::String(Default::default())
	}
}
