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
pub struct RestartServiceByUuid200Response {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RestartServiceByUuid200Response {
    pub fn new() -> RestartServiceByUuid200Response {
        RestartServiceByUuid200Response {
            message: None,
        }
    }
}

