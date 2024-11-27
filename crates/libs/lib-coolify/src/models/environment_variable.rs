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

/// EnvironmentVariable : Environment Variable model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<i32>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<i32>,
    #[serde(rename = "database_id", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,
    #[serde(rename = "is_build_time", skip_serializing_if = "Option::is_none")]
    pub is_build_time: Option<bool>,
    #[serde(rename = "is_literal", skip_serializing_if = "Option::is_none")]
    pub is_literal: Option<bool>,
    #[serde(rename = "is_multiline", skip_serializing_if = "Option::is_none")]
    pub is_multiline: Option<bool>,
    #[serde(rename = "is_preview", skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[serde(rename = "is_shared", skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(rename = "is_shown_once", skip_serializing_if = "Option::is_none")]
    pub is_shown_once: Option<bool>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "real_value", skip_serializing_if = "Option::is_none")]
    pub real_value: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl EnvironmentVariable {
    /// Environment Variable model
    pub fn new() -> EnvironmentVariable {
        EnvironmentVariable {
            id: None,
            uuid: None,
            application_id: None,
            service_id: None,
            database_id: None,
            is_build_time: None,
            is_literal: None,
            is_multiline: None,
            is_preview: None,
            is_shared: None,
            is_shown_once: None,
            key: None,
            value: None,
            real_value: None,
            version: None,
            created_at: None,
            updated_at: None,
        }
    }
}

