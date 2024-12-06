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
pub trait WebhooksApi: Send + Sync {
	async fn create_webhook<'fields, 'meta, 'create_webhook_request>(
		&self,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		create_webhook_request: Option<models::CreateWebhookRequest>,
	) -> Result<models::CreateRole200Response, Error<CreateWebhookError>>;
	async fn delete_webhook<'id>(
		&self,
		id: &'id str,
	) -> Result<(), Error<DeleteWebhookError>>;
	async fn delete_webhooks(&self) -> Result<(), Error<DeleteWebhooksError>>;
	async fn get_webhook<'id>(
		&self,
		id: &'id str,
	) -> Result<models::GetWebhooks200Response, Error<GetWebhookError>>;
	async fn get_webhooks(
		&self,
	) -> Result<models::GetWebhooks200Response, Error<GetWebhooksError>>;
	async fn update_webhook<'id, 'fields, 'meta, 'create_webhook_request>(
		&self,
		id: &'id str,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		create_webhook_request: Option<models::CreateWebhookRequest>,
	) -> Result<models::CreateRole200Response, Error<UpdateWebhookError>>;
	async fn update_webhooks<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'update_webhooks_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::serde_json::Value>,
		search: Option<&'search str>,
		update_webhooks_request: Option<models::UpdateWebhooksRequest>,
	) -> Result<models::UpdateWebhooks200Response, Error<UpdateWebhooksError>>;
}

pub struct WebhooksApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl WebhooksApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl WebhooksApi for WebhooksApiClient {
	/// Create a new webhook.
	async fn create_webhook<'fields, 'meta, 'create_webhook_request>(
		&self,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		create_webhook_request: Option<models::CreateWebhookRequest>,
	) -> Result<models::CreateRole200Response, Error<CreateWebhookError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/webhooks", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = fields {
			local_var_req_builder = match "csv" {
				"multi" => local_var_req_builder.query(
					&local_var_str
						.iter()
						.map(|p| ("fields".to_owned(), p.to_string()))
						.collect::<Vec<(std::string::String, std::string::String)>>(
						),
				),
				_ => local_var_req_builder.query(&[(
					"fields",
					&local_var_str
						.iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(ref local_var_str) = meta {
			local_var_req_builder =
				local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		local_var_req_builder = local_var_req_builder.json(&create_webhook_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateWebhookError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete an existing webhook
	async fn delete_webhook<'id>(
		&self,
		id: &'id str,
	) -> Result<(), Error<DeleteWebhookError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/webhooks/{id}",
			local_var_configuration.base_path,
			id = crate::apis::urlencode(id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
			Ok(())
		} else {
			let local_var_entity: Option<DeleteWebhookError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete multiple existing webhooks.
	async fn delete_webhooks(&self) -> Result<(), Error<DeleteWebhooksError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/webhooks", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
			Ok(())
		} else {
			let local_var_entity: Option<DeleteWebhooksError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve a single webhook by unique identifier.
	async fn get_webhook<'id>(
		&self,
		id: &'id str,
	) -> Result<models::GetWebhooks200Response, Error<GetWebhookError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/webhooks/{id}",
			local_var_configuration.base_path,
			id = crate::apis::urlencode(id)
		);
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetWebhookError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get all webhooks.
	async fn get_webhooks(
		&self,
	) -> Result<models::GetWebhooks200Response, Error<GetWebhooksError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/webhooks", local_var_configuration.base_path);
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetWebhooksError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update an existing webhook
	async fn update_webhook<'id, 'fields, 'meta, 'create_webhook_request>(
		&self,
		id: &'id str,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		create_webhook_request: Option<models::CreateWebhookRequest>,
	) -> Result<models::CreateRole200Response, Error<UpdateWebhookError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/webhooks/{id}",
			local_var_configuration.base_path,
			id = crate::apis::urlencode(id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = fields {
			local_var_req_builder = match "csv" {
				"multi" => local_var_req_builder.query(
					&local_var_str
						.iter()
						.map(|p| ("fields".to_owned(), p.to_string()))
						.collect::<Vec<(std::string::String, std::string::String)>>(
						),
				),
				_ => local_var_req_builder.query(&[(
					"fields",
					&local_var_str
						.iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(ref local_var_str) = meta {
			local_var_req_builder =
				local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		local_var_req_builder = local_var_req_builder.json(&create_webhook_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateWebhookError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update multiple webhooks at the same time.
	async fn update_webhooks<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'update_webhooks_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::serde_json::Value>,
		search: Option<&'search str>,
		update_webhooks_request: Option<models::UpdateWebhooksRequest>,
	) -> Result<models::UpdateWebhooks200Response, Error<UpdateWebhooksError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/webhooks", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = fields {
			local_var_req_builder = match "csv" {
				"multi" => local_var_req_builder.query(
					&local_var_str
						.iter()
						.map(|p| ("fields".to_owned(), p.to_string()))
						.collect::<Vec<(std::string::String, std::string::String)>>(
						),
				),
				_ => local_var_req_builder.query(&[(
					"fields",
					&local_var_str
						.iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(ref local_var_str) = limit {
			local_var_req_builder = local_var_req_builder
				.query(&[("limit", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = meta {
			local_var_req_builder =
				local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = offset {
			local_var_req_builder = local_var_req_builder
				.query(&[("offset", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = sort {
			local_var_req_builder = match "csv" {
				"multi" => local_var_req_builder.query(
					&local_var_str
						.iter()
						.map(|p| ("sort".to_owned(), p.to_string()))
						.collect::<Vec<(std::string::String, std::string::String)>>(
						),
				),
				_ => local_var_req_builder.query(&[(
					"sort",
					&local_var_str
						.iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(ref local_var_str) = filter {
			local_var_req_builder = local_var_req_builder
				.query(&[("filter", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = search {
			local_var_req_builder = local_var_req_builder
				.query(&[("search", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		local_var_req_builder = local_var_req_builder.json(&update_webhooks_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateWebhooksError> =
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

/// struct for typed errors of method [`create_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhookError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksError {
	Status401(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksError {
	Status401(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}
