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
pub trait CommentsApi: Send + Sync {
    async fn create_comment<'fields, 'meta, 'create_comment_request>(&self, fields: Option<Vec<String>>, meta: Option<&'meta str>, create_comment_request: Option<models::CreateCommentRequest>) -> Result<models::CreateComment200Response, Error<CreateCommentError>>;
    async fn delete_comment<'id>(&self, id: &'id str) -> Result<(), Error<DeleteCommentError>>;
    async fn delete_comments<>(&self, ) -> Result<(), Error<DeleteCommentsError>>;
    async fn get_comment<'id, 'fields, 'meta>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreateComment200Response, Error<GetCommentError>>;
    async fn get_comments<'fields, 'limit, 'offset, 'page, 'sort, 'filter, 'search, 'meta>(&self, fields: Option<Vec<String>>, limit: Option<i32>, offset: Option<i32>, page: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, meta: Option<&'meta str>) -> Result<models::GetComments200Response, Error<GetCommentsError>>;
    async fn update_comment<'id, 'fields, 'meta, 'update_comment_request>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>, update_comment_request: Option<models::UpdateCommentRequest>) -> Result<models::CreateComment200Response, Error<UpdateCommentError>>;
    async fn update_comments<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'update_comments_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, update_comments_request: Option<models::UpdateCommentsRequest>) -> Result<models::GetComments200Response, Error<UpdateCommentsError>>;
}

pub struct CommentsApiClient {
    configuration: Arc<configuration::Configuration>
}

impl CommentsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl CommentsApi for CommentsApiClient {
    /// Create a new comment.
    async fn create_comment<'fields, 'meta, 'create_comment_request>(&self, fields: Option<Vec<String>>, meta: Option<&'meta str>, create_comment_request: Option<models::CreateCommentRequest>) -> Result<models::CreateComment200Response, Error<CreateCommentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        local_var_req_builder = local_var_req_builder.json(&create_comment_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateCommentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete an existing comment.
    async fn delete_comment<'id>(&self, id: &'id str) -> Result<(), Error<DeleteCommentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
            Ok(())
        } else {
            let local_var_entity: Option<DeleteCommentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete multiple existing comments.
    async fn delete_comments<>(&self, ) -> Result<(), Error<DeleteCommentsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments", local_var_configuration.base_path);
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
            let local_var_entity: Option<DeleteCommentsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve a single comment by unique identifier.
    async fn get_comment<'id, 'fields, 'meta>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreateComment200Response, Error<GetCommentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
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
            let local_var_entity: Option<GetCommentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List the comments.
    async fn get_comments<'fields, 'limit, 'offset, 'page, 'sort, 'filter, 'search, 'meta>(&self, fields: Option<Vec<String>>, limit: Option<i32>, offset: Option<i32>, page: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, meta: Option<&'meta str>) -> Result<models::GetComments200Response, Error<GetCommentsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments", local_var_configuration.base_path);
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
        if let Some(ref local_var_str) = page {
            local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
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
            let local_var_entity: Option<GetCommentsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update an existing comment.
    async fn update_comment<'id, 'fields, 'meta, 'update_comment_request>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>, update_comment_request: Option<models::UpdateCommentRequest>) -> Result<models::CreateComment200Response, Error<UpdateCommentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
        local_var_req_builder = local_var_req_builder.json(&update_comment_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateCommentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update multiple comments at the same time.
    async fn update_comments<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'update_comments_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, update_comments_request: Option<models::UpdateCommentsRequest>) -> Result<models::GetComments200Response, Error<UpdateCommentsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/comments", local_var_configuration.base_path);
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
        local_var_req_builder = local_var_req_builder.json(&update_comments_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateCommentsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`create_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCommentError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCommentError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_comments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCommentsError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommentError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_comments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommentsError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCommentError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_comments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCommentsError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

