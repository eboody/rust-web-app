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
pub struct CreateDatabaseMariadbRequest {
    /// UUID of the server
    #[serde(rename = "server_uuid")]
    pub server_uuid: String,
    /// UUID of the project
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    /// Name of the environment
    #[serde(rename = "environment_name")]
    pub environment_name: String,
    /// UUID of the destination if the server has multiple destinations
    #[serde(rename = "destination_uuid", skip_serializing_if = "Option::is_none")]
    pub destination_uuid: Option<String>,
    /// MariaDB conf
    #[serde(rename = "mariadb_conf", skip_serializing_if = "Option::is_none")]
    pub mariadb_conf: Option<String>,
    /// MariaDB root password
    #[serde(rename = "mariadb_root_password", skip_serializing_if = "Option::is_none")]
    pub mariadb_root_password: Option<String>,
    /// MariaDB user
    #[serde(rename = "mariadb_user", skip_serializing_if = "Option::is_none")]
    pub mariadb_user: Option<String>,
    /// MariaDB password
    #[serde(rename = "mariadb_password", skip_serializing_if = "Option::is_none")]
    pub mariadb_password: Option<String>,
    /// MariaDB database
    #[serde(rename = "mariadb_database", skip_serializing_if = "Option::is_none")]
    pub mariadb_database: Option<String>,
    /// Name of the database
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the database
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Docker Image of the database
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Is the database public?
    #[serde(rename = "is_public", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// Public port of the database
    #[serde(rename = "public_port", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    /// Memory limit of the database
    #[serde(rename = "limits_memory", skip_serializing_if = "Option::is_none")]
    pub limits_memory: Option<String>,
    /// Memory swap limit of the database
    #[serde(rename = "limits_memory_swap", skip_serializing_if = "Option::is_none")]
    pub limits_memory_swap: Option<String>,
    /// Memory swappiness of the database
    #[serde(rename = "limits_memory_swappiness", skip_serializing_if = "Option::is_none")]
    pub limits_memory_swappiness: Option<i32>,
    /// Memory reservation of the database
    #[serde(rename = "limits_memory_reservation", skip_serializing_if = "Option::is_none")]
    pub limits_memory_reservation: Option<String>,
    /// CPU limit of the database
    #[serde(rename = "limits_cpus", skip_serializing_if = "Option::is_none")]
    pub limits_cpus: Option<String>,
    /// CPU set of the database
    #[serde(rename = "limits_cpuset", skip_serializing_if = "Option::is_none")]
    pub limits_cpuset: Option<String>,
    /// CPU shares of the database
    #[serde(rename = "limits_cpu_shares", skip_serializing_if = "Option::is_none")]
    pub limits_cpu_shares: Option<i32>,
    /// Instant deploy the database
    #[serde(rename = "instant_deploy", skip_serializing_if = "Option::is_none")]
    pub instant_deploy: Option<bool>,
}

impl CreateDatabaseMariadbRequest {
    pub fn new(server_uuid: String, project_uuid: String, environment_name: String) -> CreateDatabaseMariadbRequest {
        CreateDatabaseMariadbRequest {
            server_uuid,
            project_uuid,
            environment_name,
            destination_uuid: None,
            mariadb_conf: None,
            mariadb_root_password: None,
            mariadb_user: None,
            mariadb_password: None,
            mariadb_database: None,
            name: None,
            description: None,
            image: None,
            is_public: None,
            public_port: None,
            limits_memory: None,
            limits_memory_swap: None,
            limits_memory_swappiness: None,
            limits_memory_reservation: None,
            limits_cpus: None,
            limits_cpuset: None,
            limits_cpu_shares: None,
            instant_deploy: None,
        }
    }
}
