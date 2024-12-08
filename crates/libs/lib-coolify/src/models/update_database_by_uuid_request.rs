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
pub struct UpdateDatabaseByUuidRequest {
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
    /// PostgreSQL user
    #[serde(rename = "postgres_user", skip_serializing_if = "Option::is_none")]
    pub postgres_user: Option<String>,
    /// PostgreSQL password
    #[serde(rename = "postgres_password", skip_serializing_if = "Option::is_none")]
    pub postgres_password: Option<String>,
    /// PostgreSQL database
    #[serde(rename = "postgres_db", skip_serializing_if = "Option::is_none")]
    pub postgres_db: Option<String>,
    /// PostgreSQL initdb args
    #[serde(rename = "postgres_initdb_args", skip_serializing_if = "Option::is_none")]
    pub postgres_initdb_args: Option<String>,
    /// PostgreSQL host auth method
    #[serde(rename = "postgres_host_auth_method", skip_serializing_if = "Option::is_none")]
    pub postgres_host_auth_method: Option<String>,
    /// PostgreSQL conf
    #[serde(rename = "postgres_conf", skip_serializing_if = "Option::is_none")]
    pub postgres_conf: Option<String>,
    /// Clickhouse admin user
    #[serde(rename = "clickhouse_admin_user", skip_serializing_if = "Option::is_none")]
    pub clickhouse_admin_user: Option<String>,
    /// Clickhouse admin password
    #[serde(rename = "clickhouse_admin_password", skip_serializing_if = "Option::is_none")]
    pub clickhouse_admin_password: Option<String>,
    /// DragonFly password
    #[serde(rename = "dragonfly_password", skip_serializing_if = "Option::is_none")]
    pub dragonfly_password: Option<String>,
    /// Redis password
    #[serde(rename = "redis_password", skip_serializing_if = "Option::is_none")]
    pub redis_password: Option<String>,
    /// Redis conf
    #[serde(rename = "redis_conf", skip_serializing_if = "Option::is_none")]
    pub redis_conf: Option<String>,
    /// KeyDB password
    #[serde(rename = "keydb_password", skip_serializing_if = "Option::is_none")]
    pub keydb_password: Option<String>,
    /// KeyDB conf
    #[serde(rename = "keydb_conf", skip_serializing_if = "Option::is_none")]
    pub keydb_conf: Option<String>,
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
    /// Mongo conf
    #[serde(rename = "mongo_conf", skip_serializing_if = "Option::is_none")]
    pub mongo_conf: Option<String>,
    /// Mongo initdb root username
    #[serde(rename = "mongo_initdb_root_username", skip_serializing_if = "Option::is_none")]
    pub mongo_initdb_root_username: Option<String>,
    /// Mongo initdb root password
    #[serde(rename = "mongo_initdb_root_password", skip_serializing_if = "Option::is_none")]
    pub mongo_initdb_root_password: Option<String>,
    /// Mongo initdb init database
    #[serde(rename = "mongo_initdb_init_database", skip_serializing_if = "Option::is_none")]
    pub mongo_initdb_init_database: Option<String>,
    /// MySQL root password
    #[serde(rename = "mysql_root_password", skip_serializing_if = "Option::is_none")]
    pub mysql_root_password: Option<String>,
    /// MySQL user
    #[serde(rename = "mysql_user", skip_serializing_if = "Option::is_none")]
    pub mysql_user: Option<String>,
    /// MySQL database
    #[serde(rename = "mysql_database", skip_serializing_if = "Option::is_none")]
    pub mysql_database: Option<String>,
    /// MySQL conf
    #[serde(rename = "mysql_conf", skip_serializing_if = "Option::is_none")]
    pub mysql_conf: Option<String>,
}

impl UpdateDatabaseByUuidRequest {
    pub fn new() -> UpdateDatabaseByUuidRequest {
        UpdateDatabaseByUuidRequest {
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
            postgres_user: None,
            postgres_password: None,
            postgres_db: None,
            postgres_initdb_args: None,
            postgres_host_auth_method: None,
            postgres_conf: None,
            clickhouse_admin_user: None,
            clickhouse_admin_password: None,
            dragonfly_password: None,
            redis_password: None,
            redis_conf: None,
            keydb_password: None,
            keydb_conf: None,
            mariadb_conf: None,
            mariadb_root_password: None,
            mariadb_user: None,
            mariadb_password: None,
            mariadb_database: None,
            mongo_conf: None,
            mongo_initdb_root_username: None,
            mongo_initdb_root_password: None,
            mongo_initdb_init_database: None,
            mysql_root_password: None,
            mysql_user: None,
            mysql_database: None,
            mysql_conf: None,
        }
    }
}

