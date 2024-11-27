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
pub struct CreatePublicApplicationRequest {
    /// The project UUID.
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    /// The server UUID.
    #[serde(rename = "server_uuid")]
    pub server_uuid: String,
    /// The environment name.
    #[serde(rename = "environment_name")]
    pub environment_name: String,
    /// The git repository URL.
    #[serde(rename = "git_repository")]
    pub git_repository: String,
    /// The git branch.
    #[serde(rename = "git_branch")]
    pub git_branch: String,
    /// The build pack type.
    #[serde(rename = "build_pack")]
    pub build_pack: BuildPack,
    /// The ports to expose.
    #[serde(rename = "ports_exposes")]
    pub ports_exposes: String,
    /// The destination UUID.
    #[serde(rename = "destination_uuid", skip_serializing_if = "Option::is_none")]
    pub destination_uuid: Option<String>,
    /// The application name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The application description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The application domains.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// The git commit SHA.
    #[serde(rename = "git_commit_sha", skip_serializing_if = "Option::is_none")]
    pub git_commit_sha: Option<String>,
    /// The docker registry image name.
    #[serde(rename = "docker_registry_image_name", skip_serializing_if = "Option::is_none")]
    pub docker_registry_image_name: Option<String>,
    /// The docker registry image tag.
    #[serde(rename = "docker_registry_image_tag", skip_serializing_if = "Option::is_none")]
    pub docker_registry_image_tag: Option<String>,
    /// The flag to indicate if the application is static.
    #[serde(rename = "is_static", skip_serializing_if = "Option::is_none")]
    pub is_static: Option<bool>,
    /// The static image.
    #[serde(rename = "static_image", skip_serializing_if = "Option::is_none")]
    pub static_image: Option<StaticImage>,
    /// The install command.
    #[serde(rename = "install_command", skip_serializing_if = "Option::is_none")]
    pub install_command: Option<String>,
    /// The build command.
    #[serde(rename = "build_command", skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    /// The start command.
    #[serde(rename = "start_command", skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
    /// The ports mappings.
    #[serde(rename = "ports_mappings", skip_serializing_if = "Option::is_none")]
    pub ports_mappings: Option<String>,
    /// The base directory for all commands.
    #[serde(rename = "base_directory", skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<String>,
    /// The publish directory.
    #[serde(rename = "publish_directory", skip_serializing_if = "Option::is_none")]
    pub publish_directory: Option<String>,
    /// Health check enabled.
    #[serde(rename = "health_check_enabled", skip_serializing_if = "Option::is_none")]
    pub health_check_enabled: Option<bool>,
    /// Health check path.
    #[serde(rename = "health_check_path", skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// Health check port.
    #[serde(rename = "health_check_port", skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,
    /// Health check host.
    #[serde(rename = "health_check_host", skip_serializing_if = "Option::is_none")]
    pub health_check_host: Option<String>,
    /// Health check method.
    #[serde(rename = "health_check_method", skip_serializing_if = "Option::is_none")]
    pub health_check_method: Option<String>,
    /// Health check return code.
    #[serde(rename = "health_check_return_code", skip_serializing_if = "Option::is_none")]
    pub health_check_return_code: Option<i32>,
    /// Health check scheme.
    #[serde(rename = "health_check_scheme", skip_serializing_if = "Option::is_none")]
    pub health_check_scheme: Option<String>,
    /// Health check response text.
    #[serde(rename = "health_check_response_text", skip_serializing_if = "Option::is_none")]
    pub health_check_response_text: Option<String>,
    /// Health check interval in seconds.
    #[serde(rename = "health_check_interval", skip_serializing_if = "Option::is_none")]
    pub health_check_interval: Option<i32>,
    /// Health check timeout in seconds.
    #[serde(rename = "health_check_timeout", skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    /// Health check retries count.
    #[serde(rename = "health_check_retries", skip_serializing_if = "Option::is_none")]
    pub health_check_retries: Option<i32>,
    /// Health check start period in seconds.
    #[serde(rename = "health_check_start_period", skip_serializing_if = "Option::is_none")]
    pub health_check_start_period: Option<i32>,
    /// Memory limit.
    #[serde(rename = "limits_memory", skip_serializing_if = "Option::is_none")]
    pub limits_memory: Option<String>,
    /// Memory swap limit.
    #[serde(rename = "limits_memory_swap", skip_serializing_if = "Option::is_none")]
    pub limits_memory_swap: Option<String>,
    /// Memory swappiness.
    #[serde(rename = "limits_memory_swappiness", skip_serializing_if = "Option::is_none")]
    pub limits_memory_swappiness: Option<i32>,
    /// Memory reservation.
    #[serde(rename = "limits_memory_reservation", skip_serializing_if = "Option::is_none")]
    pub limits_memory_reservation: Option<String>,
    /// CPU limit.
    #[serde(rename = "limits_cpus", skip_serializing_if = "Option::is_none")]
    pub limits_cpus: Option<String>,
    /// CPU set.
    #[serde(rename = "limits_cpuset", skip_serializing_if = "Option::is_none")]
    pub limits_cpuset: Option<String>,
    /// CPU shares.
    #[serde(rename = "limits_cpu_shares", skip_serializing_if = "Option::is_none")]
    pub limits_cpu_shares: Option<i32>,
    /// Custom labels.
    #[serde(rename = "custom_labels", skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<String>,
    /// Custom docker run options.
    #[serde(rename = "custom_docker_run_options", skip_serializing_if = "Option::is_none")]
    pub custom_docker_run_options: Option<String>,
    /// Post deployment command.
    #[serde(rename = "post_deployment_command", skip_serializing_if = "Option::is_none")]
    pub post_deployment_command: Option<String>,
    /// Post deployment command container.
    #[serde(rename = "post_deployment_command_container", skip_serializing_if = "Option::is_none")]
    pub post_deployment_command_container: Option<String>,
    /// Pre deployment command.
    #[serde(rename = "pre_deployment_command", skip_serializing_if = "Option::is_none")]
    pub pre_deployment_command: Option<String>,
    /// Pre deployment command container.
    #[serde(rename = "pre_deployment_command_container", skip_serializing_if = "Option::is_none")]
    pub pre_deployment_command_container: Option<String>,
    /// Manual webhook secret for Github.
    #[serde(rename = "manual_webhook_secret_github", skip_serializing_if = "Option::is_none")]
    pub manual_webhook_secret_github: Option<String>,
    /// Manual webhook secret for Gitlab.
    #[serde(rename = "manual_webhook_secret_gitlab", skip_serializing_if = "Option::is_none")]
    pub manual_webhook_secret_gitlab: Option<String>,
    /// Manual webhook secret for Bitbucket.
    #[serde(rename = "manual_webhook_secret_bitbucket", skip_serializing_if = "Option::is_none")]
    pub manual_webhook_secret_bitbucket: Option<String>,
    /// Manual webhook secret for Gitea.
    #[serde(rename = "manual_webhook_secret_gitea", skip_serializing_if = "Option::is_none")]
    pub manual_webhook_secret_gitea: Option<String>,
    /// How to set redirect with Traefik / Caddy. www<->non-www.
    #[serde(rename = "redirect", skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Redirect>,
    /// The flag to indicate if the application should be deployed instantly.
    #[serde(rename = "instant_deploy", skip_serializing_if = "Option::is_none")]
    pub instant_deploy: Option<bool>,
    /// The Dockerfile content.
    #[serde(rename = "dockerfile", skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    /// The Docker Compose location.
    #[serde(rename = "docker_compose_location", skip_serializing_if = "Option::is_none")]
    pub docker_compose_location: Option<String>,
    /// The Docker Compose raw content.
    #[serde(rename = "docker_compose_raw", skip_serializing_if = "Option::is_none")]
    pub docker_compose_raw: Option<String>,
    /// The Docker Compose custom start command.
    #[serde(rename = "docker_compose_custom_start_command", skip_serializing_if = "Option::is_none")]
    pub docker_compose_custom_start_command: Option<String>,
    /// The Docker Compose custom build command.
    #[serde(rename = "docker_compose_custom_build_command", skip_serializing_if = "Option::is_none")]
    pub docker_compose_custom_build_command: Option<String>,
    /// The Docker Compose domains.
    #[serde(rename = "docker_compose_domains", skip_serializing_if = "Option::is_none")]
    pub docker_compose_domains: Option<Vec<serde_json::Value>>,
    /// The watch paths.
    #[serde(rename = "watch_paths", skip_serializing_if = "Option::is_none")]
    pub watch_paths: Option<String>,
    /// Use build server.
    #[serde(rename = "use_build_server", skip_serializing_if = "Option::is_none")]
    pub use_build_server: Option<bool>,
}

impl CreatePublicApplicationRequest {
    pub fn new(project_uuid: String, server_uuid: String, environment_name: String, git_repository: String, git_branch: String, build_pack: BuildPack, ports_exposes: String) -> CreatePublicApplicationRequest {
        CreatePublicApplicationRequest {
            project_uuid,
            server_uuid,
            environment_name,
            git_repository,
            git_branch,
            build_pack,
            ports_exposes,
            destination_uuid: None,
            name: None,
            description: None,
            domains: None,
            git_commit_sha: None,
            docker_registry_image_name: None,
            docker_registry_image_tag: None,
            is_static: None,
            static_image: None,
            install_command: None,
            build_command: None,
            start_command: None,
            ports_mappings: None,
            base_directory: None,
            publish_directory: None,
            health_check_enabled: None,
            health_check_path: None,
            health_check_port: None,
            health_check_host: None,
            health_check_method: None,
            health_check_return_code: None,
            health_check_scheme: None,
            health_check_response_text: None,
            health_check_interval: None,
            health_check_timeout: None,
            health_check_retries: None,
            health_check_start_period: None,
            limits_memory: None,
            limits_memory_swap: None,
            limits_memory_swappiness: None,
            limits_memory_reservation: None,
            limits_cpus: None,
            limits_cpuset: None,
            limits_cpu_shares: None,
            custom_labels: None,
            custom_docker_run_options: None,
            post_deployment_command: None,
            post_deployment_command_container: None,
            pre_deployment_command: None,
            pre_deployment_command_container: None,
            manual_webhook_secret_github: None,
            manual_webhook_secret_gitlab: None,
            manual_webhook_secret_bitbucket: None,
            manual_webhook_secret_gitea: None,
            redirect: None,
            instant_deploy: None,
            dockerfile: None,
            docker_compose_location: None,
            docker_compose_raw: None,
            docker_compose_custom_start_command: None,
            docker_compose_custom_build_command: None,
            docker_compose_domains: None,
            watch_paths: None,
            use_build_server: None,
        }
    }
}
/// The build pack type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BuildPack {
    #[serde(rename = "nixpacks")]
    Nixpacks,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "dockerfile")]
    Dockerfile,
    #[serde(rename = "dockercompose")]
    Dockercompose,
}

impl Default for BuildPack {
    fn default() -> BuildPack {
        Self::Nixpacks
    }
}
/// The static image.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StaticImage {
    #[serde(rename = "nginx:alpine")]
    NginxColonAlpine,
}

impl Default for StaticImage {
    fn default() -> StaticImage {
        Self::NginxColonAlpine
    }
}
/// How to set redirect with Traefik / Caddy. www<->non-www.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Redirect {
    #[serde(rename = "www")]
    Www,
    #[serde(rename = "non-www")]
    NonWww,
    #[serde(rename = "both")]
    Both,
}

impl Default for Redirect {
    fn default() -> Redirect {
        Self::Www
    }
}

