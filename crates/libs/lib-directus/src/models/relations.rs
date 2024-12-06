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
pub struct Relations {
	/// Unique identifier for the relation.
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<i32>,
	/// Collection that has the field that holds the foreign key.
	#[serde(rename = "many_collection", skip_serializing_if = "Option::is_none")]
	pub many_collection: Option<String>,
	/// Foreign key. Field that holds the primary key of the related collection.
	#[serde(rename = "many_field", skip_serializing_if = "Option::is_none")]
	pub many_field: Option<String>,
	/// Collection on the _one_ side of the relationship.
	#[serde(rename = "one_collection", skip_serializing_if = "Option::is_none")]
	pub one_collection: Option<String>,
	/// Alias column that serves as the _one_ side of the relationship.
	#[serde(
		rename = "one_field",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub one_field: Option<Option<String>>,
	#[serde(
		rename = "one_collection_field",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub one_collection_field: Option<Option<String>>,
	#[serde(
		rename = "one_allowed_collections",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub one_allowed_collections: Option<Option<Vec<String>>>,
	/// Field on the junction table that holds the many field of the related relation.
	#[serde(
		rename = "junction_field",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub junction_field: Option<Option<String>>,
	#[serde(
		rename = "sort_field",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub sort_field: Option<Option<String>>,
	#[serde(
		rename = "one_deselect_action",
		skip_serializing_if = "Option::is_none"
	)]
	pub one_deselect_action: Option<String>,
}

impl Relations {
	pub fn new() -> Relations {
		Relations {
			id: None,
			many_collection: None,
			many_field: None,
			one_collection: None,
			one_field: None,
			one_collection_field: None,
			one_allowed_collections: None,
			junction_field: None,
			sort_field: None,
			one_deselect_action: None,
		}
	}
}
