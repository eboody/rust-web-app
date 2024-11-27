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
pub struct StartApplicationByUuid200Response {
    /// Message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// UUID of the deployment.
    #[serde(rename = "deployment_uuid", skip_serializing_if = "Option::is_none")]
    pub deployment_uuid: Option<String>,
}

impl StartApplicationByUuid200Response {
    pub fn new() -> StartApplicationByUuid200Response {
        StartApplicationByUuid200Response {
            message: None,
            deployment_uuid: None,
        }
    }
}
