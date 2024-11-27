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

/// ActivityCollection : Collection identifier in which the item resides.
/// Collection identifier in which the item resides.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityCollection {
    String(String),
    Collections(Box<models::Collections>),
}

impl Default for ActivityCollection {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
