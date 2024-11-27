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
pub struct CreateEnvByApplicationUuid201Response {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl CreateEnvByApplicationUuid201Response {
    pub fn new() -> CreateEnvByApplicationUuid201Response {
        CreateEnvByApplicationUuid201Response {
            uuid: None,
        }
    }
}
