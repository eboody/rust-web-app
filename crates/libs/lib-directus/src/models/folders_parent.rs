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

/// FoldersParent : Unique identifier of the parent folder. This allows for nested folders.
/// Unique identifier of the parent folder. This allows for nested folders.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldersParent {
	String(String),
	Folders(Box<models::Folders>),
}

impl Default for FoldersParent {
	fn default() -> Self {
		Self::String(Default::default())
	}
}
