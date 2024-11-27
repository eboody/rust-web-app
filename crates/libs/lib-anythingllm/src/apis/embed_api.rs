/*
 * AnythingLLM Developer API
 *
 * API endpoints that enable programmatic reading, writing, and updating of your AnythingLLM instance. UI supplied by Swagger.io.
 *
 * The version of the OpenAPI document: 1.0.0
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
pub trait EmbedApi: Send + Sync {
    async fn v1_embed_embed_uuid_chats_get<'embed_uuid>(&self, embed_uuid: &'embed_uuid str) -> Result<serde_json::Value, Error<V1EmbedEmbedUuidChatsGetError>>;
    async fn v1_embed_embed_uuid_chats_session_uuid_get<'embed_uuid, 'session_uuid>(&self, embed_uuid: &'embed_uuid str, session_uuid: &'session_uuid str) -> Result<serde_json::Value, Error<V1EmbedEmbedUuidChatsSessionUuidGetError>>;
    async fn v1_embed_get<>(&self, ) -> Result<serde_json::Value, Error<V1EmbedGetError>>;
}

pub struct EmbedApiClient {
    configuration: Arc<configuration::Configuration>
}

impl EmbedApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl EmbedApi for EmbedApiClient {
    /// Get all chats for a specific embed
    async fn v1_embed_embed_uuid_chats_get<'embed_uuid>(&self, embed_uuid: &'embed_uuid str) -> Result<serde_json::Value, Error<V1EmbedEmbedUuidChatsGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/embed/{embedUuid}/chats", local_var_configuration.base_path, embedUuid=crate::apis::urlencode(embed_uuid));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1EmbedEmbedUuidChatsGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get chats for a specific embed and session
    async fn v1_embed_embed_uuid_chats_session_uuid_get<'embed_uuid, 'session_uuid>(&self, embed_uuid: &'embed_uuid str, session_uuid: &'session_uuid str) -> Result<serde_json::Value, Error<V1EmbedEmbedUuidChatsSessionUuidGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/embed/{embedUuid}/chats/{sessionUuid}", local_var_configuration.base_path, embedUuid=crate::apis::urlencode(embed_uuid), sessionUuid=crate::apis::urlencode(session_uuid));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1EmbedEmbedUuidChatsSessionUuidGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List all active embeds
    async fn v1_embed_get<>(&self, ) -> Result<serde_json::Value, Error<V1EmbedGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/embed", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1EmbedGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`v1_embed_embed_uuid_chats_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1EmbedEmbedUuidChatsGetError {
    Status403(models::InvalidApiKey),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_embed_embed_uuid_chats_session_uuid_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1EmbedEmbedUuidChatsSessionUuidGetError {
    Status403(models::InvalidApiKey),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_embed_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1EmbedGetError {
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}
