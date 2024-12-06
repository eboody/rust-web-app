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

/// ContextualBundlingSpec : Settings of Contextual Bundle to support ads serving in Facebook contextual surfaces
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextualBundlingSpec {
	/// Status
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status: Option<Status>,
}

impl ContextualBundlingSpec {
	/// Settings of Contextual Bundle to support ads serving in Facebook contextual surfaces
	pub fn new() -> ContextualBundlingSpec {
		ContextualBundlingSpec { status: None }
	}
}
/// Status
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Status {
	#[serde(rename = "OPT_OUT")]
	Out,
	#[serde(rename = "OPT_IN")]
	In,
}

impl Default for Status {
	fn default() -> Status {
		Self::Out
	}
}
