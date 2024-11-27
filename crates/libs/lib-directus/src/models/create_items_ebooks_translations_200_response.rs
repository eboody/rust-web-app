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
pub struct CreateItemsEbooksTranslations200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::ItemsEbooksTranslations>>,
}

impl CreateItemsEbooksTranslations200Response {
    pub fn new() -> CreateItemsEbooksTranslations200Response {
        CreateItemsEbooksTranslations200Response {
            data: None,
        }
    }
}
