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
pub enum CreateItemsLanguagesRequest {
    Array(Vec<models::ItemsLanguages>),
    ItemsLanguages(Box<models::ItemsLanguages>),
}

impl Default for CreateItemsLanguagesRequest {
    fn default() -> Self {
        Self::Array(Default::default())
    }
}

