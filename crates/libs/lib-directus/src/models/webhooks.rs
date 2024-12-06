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
pub struct Webhooks {
	/// The index of the webhook.
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<i32>,
	/// The name of the webhook.
	#[serde(rename = "name", skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Method used in the webhook.
	#[serde(rename = "method", skip_serializing_if = "Option::is_none")]
	pub method: Option<String>,
	/// The url of the webhook.
	#[serde(
		rename = "url",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub url: Option<Option<String>>,
	/// The status of the webhook.
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	/// If yes, send the content of what was done
	#[serde(rename = "data", skip_serializing_if = "Option::is_none")]
	pub data: Option<bool>,
	/// The actions that triggers this webhook.
	#[serde(
		rename = "actions",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub actions: Option<Option<Vec<String>>>,
	#[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
	pub collections: Option<Vec<String>>,
	#[serde(
		rename = "headers",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub headers: Option<Option<serde_json::Value>>,
	#[serde(
		rename = "was_active_before_deprecation",
		skip_serializing_if = "Option::is_none"
	)]
	pub was_active_before_deprecation: Option<bool>,
	#[serde(
		rename = "migrated_flow",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub migrated_flow: Option<Option<Box<models::WebhooksMigratedFlow>>>,
}

impl Webhooks {
	pub fn new() -> Webhooks {
		Webhooks {
			id: None,
			name: None,
			method: None,
			url: None,
			status: None,
			data: None,
			actions: None,
			collections: None,
			headers: None,
			was_active_before_deprecation: None,
			migrated_flow: None,
		}
	}
}
