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
pub struct SortRequest {
    /// Primary key of item to move
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<f64>,
    /// Primary key of item where to move the current item to
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

impl SortRequest {
    pub fn new() -> SortRequest {
        SortRequest {
            item: None,
            to: None,
        }
    }
}

