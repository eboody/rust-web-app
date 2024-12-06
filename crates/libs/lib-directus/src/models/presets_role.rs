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

/// PresetsRole : The unique identifier of a role in the platform. If `user` is null, this will be used to apply the collection preset or bookmark for all users in the role.
/// The unique identifier of a role in the platform. If `user` is null, this will be used to apply the collection preset or bookmark for all users in the role.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PresetsRole {
	String(String),
	Roles(Box<models::Roles>),
}

impl Default for PresetsRole {
	fn default() -> Self {
		Self::String(Default::default())
	}
}
