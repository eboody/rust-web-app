/*
 * Coolify
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateApplicationByUuid200Response {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl UpdateApplicationByUuid200Response {
    pub fn new() -> UpdateApplicationByUuid200Response {
        UpdateApplicationByUuid200Response {
            uuid: None,
        }
    }
}
