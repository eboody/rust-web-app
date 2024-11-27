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
pub enum CreateItemsEbooksTagsRequest {
    Array(Vec<models::ItemsEbooksTags>),
    ItemsEbooksTags(Box<models::ItemsEbooksTags>),
}

impl Default for CreateItemsEbooksTagsRequest {
    fn default() -> Self {
        Self::Array(Default::default())
    }
}

