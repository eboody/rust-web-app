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
pub struct CreateRelationRequest {
	/// Collection that has the field that holds the foreign key.
	#[serde(rename = "collection_many", skip_serializing_if = "Option::is_none")]
	pub collection_many: Option<String>,
	/// Collection on the _one_ side of the relationship.
	#[serde(rename = "collection_one", skip_serializing_if = "Option::is_none")]
	pub collection_one: Option<String>,
	/// Foreign key. Field that holds the primary key of the related collection.
	#[serde(rename = "field_many", skip_serializing_if = "Option::is_none")]
	pub field_many: Option<String>,
	/// Alias column that serves as the _one_ side of the relationship.
	#[serde(rename = "field_one", skip_serializing_if = "Option::is_none")]
	pub field_one: Option<String>,
	/// Field on the junction table that holds the primary key of the related collection.
	#[serde(rename = "junction_field", skip_serializing_if = "Option::is_none")]
	pub junction_field: Option<String>,
}

impl CreateRelationRequest {
	pub fn new() -> CreateRelationRequest {
		CreateRelationRequest {
			collection_many: None,
			collection_one: None,
			field_many: None,
			field_one: None,
			junction_field: None,
		}
	}
}
