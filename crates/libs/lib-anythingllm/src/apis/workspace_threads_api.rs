/*
 * AnythingLLM Developer API
 *
 * API endpoints that enable programmatic reading, writing, and updating of your AnythingLLM instance. UI supplied by Swagger.io.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait WorkspaceThreadsApi: Send + Sync {
	async fn v1_workspace_slug_thread_new_post<'slug>(
		&self,
		slug: &'slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadNewPostError>>;
	async fn v1_workspace_slug_thread_thread_slug_chat_post<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadThreadSlugChatPostError>>;
	async fn v1_workspace_slug_thread_thread_slug_chats_get<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadThreadSlugChatsGetError>>;
	async fn v1_workspace_slug_thread_thread_slug_delete<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<(), Error<V1WorkspaceSlugThreadThreadSlugDeleteError>>;
	async fn v1_workspace_slug_thread_thread_slug_stream_chat_post<
		'slug,
		'thread_slug,
	>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<Vec<String>, Error<V1WorkspaceSlugThreadThreadSlugStreamChatPostError>>;
	async fn v1_workspace_slug_thread_thread_slug_update_post<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadThreadSlugUpdatePostError>>;
}

pub struct WorkspaceThreadsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl WorkspaceThreadsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl WorkspaceThreadsApi for WorkspaceThreadsApiClient {
	/// Create a new workspace thread
	async fn v1_workspace_slug_thread_new_post<'slug>(
		&self,
		slug: &'slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadNewPostError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/workspace/{slug}/thread/new",
			local_var_configuration.base_path,
			slug = crate::apis::urlencode(slug)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_token) =
			local_var_configuration.bearer_access_token
		{
			local_var_req_builder =
				local_var_req_builder.bearer_auth(local_var_token.to_owned());
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<V1WorkspaceSlugThreadNewPostError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Chat with a workspace thread
	async fn v1_workspace_slug_thread_thread_slug_chat_post<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadThreadSlugChatPostError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/workspace/{slug}/thread/{threadSlug}/chat",
			local_var_configuration.base_path,
			slug = crate::apis::urlencode(slug),
			threadSlug = crate::apis::urlencode(thread_slug)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_token) =
			local_var_configuration.bearer_access_token
		{
			local_var_req_builder =
				local_var_req_builder.bearer_auth(local_var_token.to_owned());
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<
				V1WorkspaceSlugThreadThreadSlugChatPostError,
			> = json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get chats for a workspace thread
	async fn v1_workspace_slug_thread_thread_slug_chats_get<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadThreadSlugChatsGetError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/workspace/{slug}/thread/{threadSlug}/chats",
			local_var_configuration.base_path,
			slug = crate::apis::urlencode(slug),
			threadSlug = crate::apis::urlencode(thread_slug)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_token) =
			local_var_configuration.bearer_access_token
		{
			local_var_req_builder =
				local_var_req_builder.bearer_auth(local_var_token.to_owned());
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<
				V1WorkspaceSlugThreadThreadSlugChatsGetError,
			> = json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete a workspace thread
	async fn v1_workspace_slug_thread_thread_slug_delete<'slug, 'thread_slug>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<(), Error<V1WorkspaceSlugThreadThreadSlugDeleteError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/workspace/{slug}/thread/{threadSlug}",
			local_var_configuration.base_path,
			slug = crate::apis::urlencode(slug),
			threadSlug = crate::apis::urlencode(thread_slug)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_token) =
			local_var_configuration.bearer_access_token
		{
			local_var_req_builder =
				local_var_req_builder.bearer_auth(local_var_token.to_owned());
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			Ok(())
		} else {
			let local_var_entity: Option<
				V1WorkspaceSlugThreadThreadSlugDeleteError,
			> = json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Stream chat with a workspace thread
	async fn v1_workspace_slug_thread_thread_slug_stream_chat_post<
		'slug,
		'thread_slug,
	>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<Vec<String>, Error<V1WorkspaceSlugThreadThreadSlugStreamChatPostError>>
	{
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/workspace/{slug}/thread/{threadSlug}/stream-chat",
			local_var_configuration.base_path,
			slug = crate::apis::urlencode(slug),
			threadSlug = crate::apis::urlencode(thread_slug)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_token) =
			local_var_configuration.bearer_access_token
		{
			local_var_req_builder =
				local_var_req_builder.bearer_auth(local_var_token.to_owned());
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<
				V1WorkspaceSlugThreadThreadSlugStreamChatPostError,
			> = json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update thread name by its unique slug.
	async fn v1_workspace_slug_thread_thread_slug_update_post<
		'slug,
		'thread_slug,
	>(
		&self,
		slug: &'slug str,
		thread_slug: &'thread_slug str,
	) -> Result<json::Value, Error<V1WorkspaceSlugThreadThreadSlugUpdatePostError>>
	{
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/workspace/{slug}/thread/{threadSlug}/update",
			local_var_configuration.base_path,
			slug = crate::apis::urlencode(slug),
			threadSlug = crate::apis::urlencode(thread_slug)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_token) =
			local_var_configuration.bearer_access_token
		{
			local_var_req_builder =
				local_var_req_builder.bearer_auth(local_var_token.to_owned());
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<
				V1WorkspaceSlugThreadThreadSlugUpdatePostError,
			> = json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}

/// struct for typed errors of method [`v1_workspace_slug_thread_new_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1WorkspaceSlugThreadNewPostError {
	Status400(),
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`v1_workspace_slug_thread_thread_slug_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1WorkspaceSlugThreadThreadSlugChatPostError {
	Status400(),
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`v1_workspace_slug_thread_thread_slug_chats_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1WorkspaceSlugThreadThreadSlugChatsGetError {
	Status400(),
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`v1_workspace_slug_thread_thread_slug_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1WorkspaceSlugThreadThreadSlugDeleteError {
	Status400(),
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`v1_workspace_slug_thread_thread_slug_stream_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1WorkspaceSlugThreadThreadSlugStreamChatPostError {
	Status400(),
	Status403(models::InvalidApiKey),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`v1_workspace_slug_thread_thread_slug_update_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1WorkspaceSlugThreadThreadSlugUpdatePostError {
	Status400(),
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(json::Value),
}
