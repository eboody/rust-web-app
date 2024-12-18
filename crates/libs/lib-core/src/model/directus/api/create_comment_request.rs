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
pub struct CreateCommentRequest {
	/// Which collection this collection comment is for.
	#[serde(rename = "collection")]
	pub collection: String,
	#[serde(rename = "item")]
	pub item: String,
	#[serde(rename = "comment")]
	pub comment: String,
}

impl CreateCommentRequest {
	pub fn new(
		collection: String,
		item: String,
		comment: String,
	) -> CreateCommentRequest {
		CreateCommentRequest {
			collection,
			item,
			comment,
		}
	}
}
