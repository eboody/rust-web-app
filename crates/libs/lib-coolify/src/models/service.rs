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

/// Service : Service model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    /// The unique identifier of the service. Only used for database identification.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The unique identifier of the service.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// The name of the service.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique identifier of the environment where the service is attached to.
    #[serde(rename = "environment_id", skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<i32>,
    /// The unique identifier of the server where the service is running.
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i32>,
    /// The description of the service.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The raw docker-compose.yml file of the service.
    #[serde(rename = "docker_compose_raw", skip_serializing_if = "Option::is_none")]
    pub docker_compose_raw: Option<String>,
    /// The docker-compose.yml file that is parsed and modified by Coolify.
    #[serde(rename = "docker_compose", skip_serializing_if = "Option::is_none")]
    pub docker_compose: Option<String>,
    /// Destination type.
    #[serde(rename = "destination_type", skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    /// The unique identifier of the destination where the service is running.
    #[serde(rename = "destination_id", skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<i32>,
    /// The flag to connect the service to the predefined Docker network.
    #[serde(rename = "connect_to_docker_network", skip_serializing_if = "Option::is_none")]
    pub connect_to_docker_network: Option<bool>,
    /// The flag to enable the container label escape.
    #[serde(rename = "is_container_label_escape_enabled", skip_serializing_if = "Option::is_none")]
    pub is_container_label_escape_enabled: Option<bool>,
    /// The flag to enable the container label readonly.
    #[serde(rename = "is_container_label_readonly_enabled", skip_serializing_if = "Option::is_none")]
    pub is_container_label_readonly_enabled: Option<bool>,
    /// The hash of the service configuration.
    #[serde(rename = "config_hash", skip_serializing_if = "Option::is_none")]
    pub config_hash: Option<String>,
    /// The type of the service.
    #[serde(rename = "service_type", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// The date and time when the service was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The date and time when the service was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The date and time when the service was deleted.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl Service {
    /// Service model
    pub fn new() -> Service {
        Service {
            id: None,
            uuid: None,
            name: None,
            environment_id: None,
            server_id: None,
            description: None,
            docker_compose_raw: None,
            docker_compose: None,
            destination_type: None,
            destination_id: None,
            connect_to_docker_network: None,
            is_container_label_escape_enabled: None,
            is_container_label_readonly_enabled: None,
            config_hash: None,
            service_type: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}
