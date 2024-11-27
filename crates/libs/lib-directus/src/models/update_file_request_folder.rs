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

/// UpdateFileRequestFolder : Virtual folder where this file resides in.
/// Virtual folder where this file resides in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFileRequestFolder {
    String(String),
    Folders(Box<models::Folders>),
}

impl Default for UpdateFileRequestFolder {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

