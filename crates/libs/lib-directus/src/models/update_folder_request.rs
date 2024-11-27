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
pub struct UpdateFolderRequest {
    /// Name of the folder. Can't be null or empty.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unique identifier of the parent folder. This allows for nested folders.
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
}

impl UpdateFolderRequest {
    pub fn new() -> UpdateFolderRequest {
        UpdateFolderRequest {
            name: None,
            parent: None,
        }
    }
}
