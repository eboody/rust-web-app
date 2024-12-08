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

/// PrivateKey : Private Key model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateKey {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "private_key", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "is_git_related", skip_serializing_if = "Option::is_none")]
    pub is_git_related: Option<bool>,
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl PrivateKey {
    /// Private Key model
    pub fn new() -> PrivateKey {
        PrivateKey {
            id: None,
            uuid: None,
            name: None,
            description: None,
            private_key: None,
            is_git_related: None,
            team_id: None,
            created_at: None,
            updated_at: None,
        }
    }
}

