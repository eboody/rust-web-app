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
pub trait SystemSettingsApi: Send + Sync {
	async fn v1_system_env_dump_get(
		&self,
	) -> Result<(), Error<V1SystemEnvDumpGetError>>;
	async fn v1_system_export_chats_get<'type_>(
		&self,
		r#type: Option<&'type_ str>,
	) -> Result<serde_json::Value, Error<V1SystemExportChatsGetError>>;
	async fn v1_system_get(
		&self,
	) -> Result<serde_json::Value, Error<V1SystemGetError>>;
	async fn v1_system_remove_documents_delete<
		'v1_system_remove_documents_delete_request,
	>(
		&self,
		v1_system_remove_documents_delete_request: models::V1SystemRemoveDocumentsDeleteRequest,
	) -> Result<serde_json::Value, Error<V1SystemRemoveDocumentsDeleteError>>;
	async fn v1_system_update_env_post(
		&self,
	) -> Result<serde_json::Value, Error<V1SystemUpdateEnvPostError>>;
	async fn v1_system_vector_count_get(
		&self,
	) -> Result<serde_json::Value, Error<V1SystemVectorCountGetError>>;
}

pub struct SystemSettingsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl SystemSettingsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl SystemSettingsApi for SystemSettingsApiClient {
	/// Dump all settings to file storage
	async fn v1_system_env_dump_get(
		&self,
	) -> Result<(), Error<V1SystemEnvDumpGetError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/v1/system/env-dump", local_var_configuration.base_path);
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
			Ok(())
		} else {
			let local_var_entity: Option<V1SystemEnvDumpGetError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Export all of the chats from the system in a known format. Output depends on the type sent. Will be send with the correct header for the output.
	async fn v1_system_export_chats_get<'type_>(
		&self,
		r#type: Option<&'type_ str>,
	) -> Result<serde_json::Value, Error<V1SystemExportChatsGetError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/system/export-chats",
			local_var_configuration.base_path
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = r#type {
			local_var_req_builder =
				local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
		}
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<V1SystemExportChatsGetError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get all current system settings that are defined.
	async fn v1_system_get(
		&self,
	) -> Result<serde_json::Value, Error<V1SystemGetError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/v1/system", local_var_configuration.base_path);
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<V1SystemGetError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Permanently remove documents from the system.
	async fn v1_system_remove_documents_delete<
		'v1_system_remove_documents_delete_request,
	>(
		&self,
		v1_system_remove_documents_delete_request: models::V1SystemRemoveDocumentsDeleteRequest,
	) -> Result<serde_json::Value, Error<V1SystemRemoveDocumentsDeleteError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/system/remove-documents",
			local_var_configuration.base_path
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
		local_var_req_builder =
			local_var_req_builder.json(&v1_system_remove_documents_delete_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<V1SystemRemoveDocumentsDeleteError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update a system setting or preference.
	async fn v1_system_update_env_post(
		&self,
	) -> Result<serde_json::Value, Error<V1SystemUpdateEnvPostError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/v1/system/update-env", local_var_configuration.base_path);
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<V1SystemUpdateEnvPostError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Number of all vectors in connected vector database
	async fn v1_system_vector_count_get(
		&self,
	) -> Result<serde_json::Value, Error<V1SystemVectorCountGetError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/v1/system/vector-count",
			local_var_configuration.base_path
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<V1SystemVectorCountGetError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}

/// struct for typed errors of method [`v1_system_env_dump_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1SystemEnvDumpGetError {
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_system_export_chats_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1SystemExportChatsGetError {
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_system_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1SystemGetError {
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_system_remove_documents_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1SystemRemoveDocumentsDeleteError {
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_system_update_env_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1SystemUpdateEnvPostError {
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_system_vector_count_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1SystemVectorCountGetError {
	Status403(models::InvalidApiKey),
	Status500(),
	UnknownValue(serde_json::Value),
}