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

/// CreateFieldRequestMeta : The meta info.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFieldRequestMeta {
	/// Unique identifier for the field in the `directus_fields` collection.
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<i32>,
	/// Unique name of the collection this field is in.
	#[serde(rename = "collection", skip_serializing_if = "Option::is_none")]
	pub collection: Option<String>,
	/// Unique name of the field. Field name is unique within the collection.
	#[serde(rename = "field", skip_serializing_if = "Option::is_none")]
	pub field: Option<String>,
	/// Transformation flag for field
	#[serde(
		rename = "special",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub special: Option<Option<Vec<String>>>,
	/// What interface is used in the admin app to edit the value for this field.
	#[serde(
		rename = "system-interface",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub system_interface: Option<Option<String>>,
	/// Options for the interface that's used. This format is based on the individual interface.
	#[serde(
		rename = "options",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub options: Option<Option<json::Value>>,
	/// What display is used in the admin app to display the value for this field.
	#[serde(
		rename = "display",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub display: Option<Option<String>>,
	/// Options for the display that's used. This format is based on the individual display.
	#[serde(
		rename = "display_options",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub display_options: Option<Option<json::Value>>,
	/// If the field can be altered by the end user. Directus system fields have this value set to `true`.
	#[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
	pub locked: Option<bool>,
	/// Prevents the user from editing the value in the field.
	#[serde(rename = "readonly", skip_serializing_if = "Option::is_none")]
	pub readonly: Option<bool>,
	/// If this field should be hidden.
	#[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
	pub hidden: Option<bool>,
	/// Sort order of this field on the edit page of the admin app.
	#[serde(
		rename = "sort",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub sort: Option<Option<i32>>,
	/// Width of the field on the edit form.
	#[serde(
		rename = "width",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub width: Option<Option<Width>>,
	/// What field group this field is part of.
	#[serde(
		rename = "group",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub group: Option<Option<i32>>,
	/// Key value pair of `<language>: <translation>` that allows the user to change the displayed name of the field in the admin app.
	#[serde(
		rename = "translation",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub translation: Option<Option<json::Value>>,
	/// A user provided note for the field. Will be rendered alongside the interface on the edit page.
	#[serde(
		rename = "note",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub note: Option<Option<String>>,
}

impl CreateFieldRequestMeta {
	/// The meta info.
	pub fn new() -> CreateFieldRequestMeta {
		CreateFieldRequestMeta {
			id: None,
			collection: None,
			field: None,
			special: None,
			system_interface: None,
			options: None,
			display: None,
			display_options: None,
			locked: None,
			readonly: None,
			hidden: None,
			sort: None,
			width: None,
			group: None,
			translation: None,
			note: None,
		}
	}
}
/// Width of the field on the edit form.
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Width {
	#[serde(rename = "half")]
	Half,
	#[serde(rename = "half-left")]
	HalfLeft,
	#[serde(rename = "half-right")]
	HalfRight,
	#[serde(rename = "full")]
	Full,
	#[serde(rename = "fill")]
	Fill,
}

impl Default for Width {
	fn default() -> Width {
		Self::Half
	}
}