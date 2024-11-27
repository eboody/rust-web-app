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

/// RevisionsVersion : Associated version of this revision.
/// Associated version of this revision.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevisionsVersion {
    String(String),
    Versions(Box<models::Versions>),
}

impl Default for RevisionsVersion {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
