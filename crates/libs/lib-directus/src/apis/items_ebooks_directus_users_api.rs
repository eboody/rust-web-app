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
pub trait ItemsEbooksDirectusUsersApi: Send + Sync {
    async fn create_items_ebooks_directus_users<'meta, 'create_items_ebooks_directus_users_request>(&self, meta: Option<&'meta str>, create_items_ebooks_directus_users_request: Option<models::CreateItemsEbooksDirectusUsersRequest>) -> Result<models::CreateItemsEbooksDirectusUsers200Response, Error<CreateItemsEbooksDirectusUsersError>>;
    async fn delete_items_ebooks_directus_users<>(&self, ) -> Result<(), Error<DeleteItemsEbooksDirectusUsersError>>;
    async fn delete_single_items_ebooks_directus_users<'id>(&self, id: models::ReadSingleItemsLanguagesIdParameter) -> Result<(), Error<DeleteSingleItemsEbooksDirectusUsersError>>;
    async fn read_items_ebooks_directus_users<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>) -> Result<models::ReadItemsEbooksDirectusUsers200Response, Error<ReadItemsEbooksDirectusUsersError>>;
    async fn read_single_items_ebooks_directus_users<'id, 'fields, 'meta, 'version>(&self, id: models::ReadSingleItemsLanguagesIdParameter, fields: Option<Vec<String>>, meta: Option<&'meta str>, version: Option<&'version str>) -> Result<models::ReadSingleItemsEbooksDirectusUsers200Response, Error<ReadSingleItemsEbooksDirectusUsersError>>;
    async fn update_items_ebooks_directus_users<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'create_items_ebooks_directus_users_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, create_items_ebooks_directus_users_request: Option<models::CreateItemsEbooksDirectusUsersRequest>) -> Result<models::UpdateItemsEbooksDirectusUsers200Response, Error<UpdateItemsEbooksDirectusUsersError>>;
    async fn update_single_items_ebooks_directus_users<'id, 'fields, 'meta, 'items_ebooks_directus_users>(&self, id: models::ReadSingleItemsLanguagesIdParameter, fields: Option<Vec<String>>, meta: Option<&'meta str>, items_ebooks_directus_users: Option<models::ItemsEbooksDirectusUsers>) -> Result<models::ReadSingleItemsEbooksDirectusUsers200Response, Error<UpdateSingleItemsEbooksDirectusUsersError>>;
}

pub struct ItemsEbooksDirectusUsersApiClient {
    configuration: Arc<configuration::Configuration>
}

impl ItemsEbooksDirectusUsersApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl ItemsEbooksDirectusUsersApi for ItemsEbooksDirectusUsersApiClient {
    /// Create a new ebooks_directus_users item.
    async fn create_items_ebooks_directus_users<'meta, 'create_items_ebooks_directus_users_request>(&self, meta: Option<&'meta str>, create_items_ebooks_directus_users_request: Option<models::CreateItemsEbooksDirectusUsersRequest>) -> Result<models::CreateItemsEbooksDirectusUsers200Response, Error<CreateItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&create_items_ebooks_directus_users_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete multiple existing ebooks_directus_users items.
    async fn delete_items_ebooks_directus_users<>(&self, ) -> Result<(), Error<DeleteItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users", local_var_configuration.base_path);
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
            let local_var_entity: Option<DeleteItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete an existing ebooks_directus_users item.
    async fn delete_single_items_ebooks_directus_users<'id>(&self, id: models::ReadSingleItemsLanguagesIdParameter) -> Result<(), Error<DeleteSingleItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users/{id}", local_var_configuration.base_path, id=id.to_string());
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
            let local_var_entity: Option<DeleteSingleItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List the ebooks_directus_users items.
    async fn read_items_ebooks_directus_users<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>) -> Result<models::ReadItemsEbooksDirectusUsers200Response, Error<ReadItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
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
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("sort".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("sort", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
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
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ReadItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve a single ebooks_directus_users item by unique identifier.
    async fn read_single_items_ebooks_directus_users<'id, 'fields, 'meta, 'version>(&self, id: models::ReadSingleItemsLanguagesIdParameter, fields: Option<Vec<String>>, meta: Option<&'meta str>, version: Option<&'version str>) -> Result<models::ReadSingleItemsEbooksDirectusUsers200Response, Error<ReadSingleItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users/{id}", local_var_configuration.base_path, id=id.to_string());
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = version {
            local_var_req_builder = local_var_req_builder.query(&[("version", &local_var_str.to_string())]);
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
            let local_var_entity: Option<ReadSingleItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update multiple ebooks_directus_users items at the same time.
    async fn update_items_ebooks_directus_users<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'create_items_ebooks_directus_users_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, create_items_ebooks_directus_users_request: Option<models::CreateItemsEbooksDirectusUsersRequest>) -> Result<models::UpdateItemsEbooksDirectusUsers200Response, Error<UpdateItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
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
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("sort".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("sort", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
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
        local_var_req_builder = local_var_req_builder.json(&create_items_ebooks_directus_users_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update an existing ebooks_directus_users item.
    async fn update_single_items_ebooks_directus_users<'id, 'fields, 'meta, 'items_ebooks_directus_users>(&self, id: models::ReadSingleItemsLanguagesIdParameter, fields: Option<Vec<String>>, meta: Option<&'meta str>, items_ebooks_directus_users: Option<models::ItemsEbooksDirectusUsers>) -> Result<models::ReadSingleItemsEbooksDirectusUsers200Response, Error<UpdateSingleItemsEbooksDirectusUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/items/ebooks_directus_users/{id}", local_var_configuration.base_path, id=id.to_string());
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&items_ebooks_directus_users);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateSingleItemsEbooksDirectusUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`create_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateItemsEbooksDirectusUsersError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteItemsEbooksDirectusUsersError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_single_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSingleItemsEbooksDirectusUsersError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadItemsEbooksDirectusUsersError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_single_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadSingleItemsEbooksDirectusUsersError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateItemsEbooksDirectusUsersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_single_items_ebooks_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSingleItemsEbooksDirectusUsersError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}
