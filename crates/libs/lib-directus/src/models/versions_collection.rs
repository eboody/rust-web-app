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

/// VersionsCollection : Name of the collection the Content Version is created on.
/// Name of the collection the Content Version is created on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionsCollection {
    String(String),
    Collections(Box<models::Collections>),
}

impl Default for VersionsCollection {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
