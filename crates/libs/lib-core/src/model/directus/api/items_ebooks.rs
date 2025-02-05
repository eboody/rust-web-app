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
pub struct ItemsEbooks {
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<uuid::Uuid>,
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(
		rename = "sort",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub sort: Option<Option<i32>>,
	#[serde(
		rename = "user_created",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub user_created: Option<Option<Box<super::FilesModifiedBy>>>,
	#[serde(
		rename = "date_created",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub date_created: Option<Option<String>>,
	#[serde(
		rename = "user_updated",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub user_updated: Option<Option<Box<super::FilesModifiedBy>>>,
	#[serde(
		rename = "date_updated",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub date_updated: Option<Option<String>>,
	#[serde(
		rename = "date_published",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub date_published: Option<Option<String>>,
	#[serde(
		rename = "translations",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub translations: Option<Option<Vec<super::ItemsEbooksTranslationsInner>>>,
	#[serde(
		rename = "authors",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub authors: Option<Option<Vec<super::ItemsEbooksAuthorsInner>>>,
	#[serde(
		rename = "tags",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub tags: Option<Option<Vec<super::ItemsEbooksTagsInner>>>,
}

impl ItemsEbooks {
	pub fn new() -> ItemsEbooks {
		ItemsEbooks {
			id: None,
			status: None,
			sort: None,
			user_created: None,
			date_created: None,
			user_updated: None,
			date_updated: None,
			date_published: None,
			translations: None,
			authors: None,
			tags: None,
		}
	}
}
