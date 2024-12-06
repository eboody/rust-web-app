/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
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
pub trait ServerApi: Send + Sync {
	async fn ping(&self) -> Result<String, Error<PingError>>;
	async fn server_info<'super_admin_token>(
		&self,
		super_admin_token: i32,
	) -> Result<models::ServerInfo200Response, Error<ServerInfoError>>;
}

pub struct ServerApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl ServerApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl ServerApi for ServerApiClient {
	/// Ping, pong. Ping.. pong.
	async fn ping(&self) -> Result<String, Error<PingError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/server/ping", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<PingError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Perform a system status check and return the options.
	async fn server_info<'super_admin_token>(
		&self,
		super_admin_token: i32,
	) -> Result<models::ServerInfo200Response, Error<ServerInfoError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/server/info", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

		local_var_req_builder = local_var_req_builder
			.query(&[("super_admin_token", &super_admin_token.to_string())]);
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<ServerInfoError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}

/// struct for typed errors of method [`ping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PingError {
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`server_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerInfoError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}
