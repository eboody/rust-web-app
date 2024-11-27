/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

#[async_trait]
pub trait PermissionsApi: Send + Sync {
    async fn create_permission<'meta, 'create_permission_request>(&self, meta: Option<&'meta str>, create_permission_request: Option<models::CreatePermissionRequest>) -> Result<models::CreatePermission200Response, Error<CreatePermissionError>>;
    async fn delete_permission<'id>(&self, id: i32) -> Result<(), Error<DeletePermissionError>>;
    async fn delete_permissions<>(&self, ) -> Result<(), Error<DeletePermissionsError>>;
    async fn get_my_permissions<>(&self, ) -> Result<models::GetMyPermissions200Response, Error<GetMyPermissionsError>>;
    async fn get_permission<'id, 'fields, 'meta>(&self, id: i32, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreatePermission200Response, Error<GetPermissionError>>;
    async fn get_permissions<'fields, 'limit, 'offset, 'meta, 'sort, 'filter, 'search, 'page>(&self, fields: Option<Vec<String>>, limit: Option<i32>, offset: Option<i32>, meta: Option<&'meta str>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, page: Option<i32>) -> Result<models::GetPermissions200Response, Error<GetPermissionsError>>;
    async fn update_permission<'id, 'meta, 'update_permission_request>(&self, id: i32, meta: Option<&'meta str>, update_permission_request: Option<models::UpdatePermissionRequest>) -> Result<models::CreatePermission200Response, Error<UpdatePermissionError>>;
    async fn update_permissions<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'update_permissions_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, update_permissions_request: Option<models::UpdatePermissionsRequest>) -> Result<models::GetPermissions200Response, Error<UpdatePermissionsError>>;
}

pub struct PermissionsApiClient {
    configuration: Arc<configuration::Configuration>
}

impl PermissionsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl PermissionsApi for PermissionsApiClient {
    /// Create a new permission.
    async fn create_permission<'meta, 'create_permission_request>(&self, meta: Option<&'meta str>, create_permission_request: Option<models::CreatePermissionRequest>) -> Result<models::CreatePermission200Response, Error<CreatePermissionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&create_permission_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreatePermissionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete an existing permission
    async fn delete_permission<'id>(&self, id: i32) -> Result<(), Error<DeletePermissionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions/{id}", local_var_configuration.base_path, id=id);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeletePermissionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete multiple existing permissions.
    async fn delete_permissions<>(&self, ) -> Result<(), Error<DeletePermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeletePermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List the permissions that apply to the current user.
    async fn get_my_permissions<>(&self, ) -> Result<models::GetMyPermissions200Response, Error<GetMyPermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions/me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetMyPermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve a single permissions object by unique identifier.
    async fn get_permission<'id, 'fields, 'meta>(&self, id: i32, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreatePermission200Response, Error<GetPermissionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions/{id}", local_var_configuration.base_path, id=id);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetPermissionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List all permissions.
    async fn get_permissions<'fields, 'limit, 'offset, 'meta, 'sort, 'filter, 'search, 'page>(&self, fields: Option<Vec<String>>, limit: Option<i32>, offset: Option<i32>, meta: Option<&'meta str>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, page: Option<i32>) -> Result<models::GetPermissions200Response, Error<GetPermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = offset {
            local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("sort".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("sort", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = filter {
            local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page {
            local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetPermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update an existing permission
    async fn update_permission<'id, 'meta, 'update_permission_request>(&self, id: i32, meta: Option<&'meta str>, update_permission_request: Option<models::UpdatePermissionRequest>) -> Result<models::CreatePermission200Response, Error<UpdatePermissionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions/{id}", local_var_configuration.base_path, id=id);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&update_permission_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdatePermissionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update multiple permissions at the same time.
    async fn update_permissions<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'update_permissions_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, update_permissions_request: Option<models::UpdatePermissionsRequest>) -> Result<models::GetPermissions200Response, Error<UpdatePermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/permissions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = offset {
            local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("sort".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("sort", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = filter {
            local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&update_permissions_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdatePermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`create_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePermissionError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePermissionError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePermissionsError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyPermissionsError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionsError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePermissionError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePermissionsError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

