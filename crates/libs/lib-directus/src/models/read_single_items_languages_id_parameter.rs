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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadSingleItemsLanguagesIdParameter {
    /// Incremental index of the item.
    Integer(i32),
    /// Unique identifier of the item.
    String(String),
}

impl Default for ReadSingleItemsLanguagesIdParameter {
    fn default() -> Self {
        Self::Integer(Default::default())
    }
}

