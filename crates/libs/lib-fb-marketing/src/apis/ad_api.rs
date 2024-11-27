/*
 * Facebook Marketing API
 *
 * This is a generated connector for [Facebook Marketing API v12.0](https://developers.facebook.com/docs/marketing-apis) OpenAPI specification.  Facebook is an American online social media and social networking service owned by Facebook, Inc.Facebook Marketing  APIs are a collection of Graph API endpoints that can be used to help you advertise on Facebook.
 *
 * The version of the OpenAPI document: v12.0
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
pub trait AdApi: Send + Sync {
	async fn create_ad<'ad_account_id, 'properties>(
		&self,
		ad_account_id: &'ad_account_id str,
		properties: models::Ad,
	) -> Result<models::AdResponse, Error<CreateAdError>>;
	async fn delete_ad<'ad_id>(
		&self,
		ad_id: &'ad_id str,
	) -> Result<models::AdResponse, Error<DeleteAdError>>;
	async fn get_ad<'ad_id, 'date_preset, 'time_range, 'updated_since, 'fields>(
		&self,
		ad_id: &'ad_id str,
		date_preset: Option<&'date_preset str>,
		time_range: Option<models::TimeRange>,
		updated_since: Option<i32>,
		fields: Option<Vec<String>>,
	) -> Result<models::Ad, Error<GetAdError>>;
	async fn get_ads<
		'ad_account_id,
		'date_preset,
		'effective_status,
		'time_range,
		'updated_since,
		'fields,
		'summary,
	>(
		&self,
		ad_account_id: &'ad_account_id str,
		date_preset: Option<&'date_preset str>,
		effective_status: Option<Vec<String>>,
		time_range: Option<models::TimeRange>,
		updated_since: Option<i32>,
		fields: Option<Vec<String>>,
		summary: Option<Vec<String>>,
	) -> Result<models::AdList, Error<GetAdsError>>;
	async fn update_ad<'ad_id, 'properties>(
		&self,
		ad_id: &'ad_id str,
		properties: models::AdUpdate,
	) -> Result<models::AdResponse, Error<UpdateAdError>>;
}

pub struct AdApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl AdApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl AdApi for AdApiClient {
	/// Cerates an ad.
	async fn create_ad<'ad_account_id, 'properties>(
		&self,
		ad_account_id: &'ad_account_id str,
		properties: models::Ad,
	) -> Result<models::AdResponse, Error<CreateAdError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/act_{ad_account_id}/ads",
			local_var_configuration.base_path,
			ad_account_id = crate::apis::urlencode(ad_account_id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		local_var_req_builder =
			local_var_req_builder.query(&[("properties", &properties.to_string())]);
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => {
					format!("{} {}", local_var_prefix, local_var_key)
				}
				None => local_var_key,
			};
			local_var_req_builder =
				local_var_req_builder.query(&[("access_token", local_var_value)]);
		}
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
			let local_var_entity: Option<CreateAdError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Deletes an ad.
	async fn delete_ad<'ad_id>(
		&self,
		ad_id: &'ad_id str,
	) -> Result<models::AdResponse, Error<DeleteAdError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/{ad_id}",
			local_var_configuration.base_path,
			ad_id = crate::apis::urlencode(ad_id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => {
					format!("{} {}", local_var_prefix, local_var_key)
				}
				None => local_var_key,
			};
			local_var_req_builder =
				local_var_req_builder.query(&[("access_token", local_var_value)]);
		}
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
			let local_var_entity: Option<DeleteAdError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns data of an ad.
	async fn get_ad<'ad_id, 'date_preset, 'time_range, 'updated_since, 'fields>(
		&self,
		ad_id: &'ad_id str,
		date_preset: Option<&'date_preset str>,
		time_range: Option<models::TimeRange>,
		updated_since: Option<i32>,
		fields: Option<Vec<String>>,
	) -> Result<models::Ad, Error<GetAdError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/{ad_id}",
			local_var_configuration.base_path,
			ad_id = crate::apis::urlencode(ad_id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = date_preset {
			local_var_req_builder = local_var_req_builder
				.query(&[("date_preset", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = time_range {
			local_var_req_builder = local_var_req_builder
				.query(&[("time_range", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = updated_since {
			local_var_req_builder = local_var_req_builder
				.query(&[("updated_since", &local_var_str.to_string())]);
		}
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
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => {
					format!("{} {}", local_var_prefix, local_var_key)
				}
				None => local_var_key,
			};
			local_var_req_builder =
				local_var_req_builder.query(&[("access_token", local_var_value)]);
		}
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
			let local_var_entity: Option<GetAdError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns ads under this ad account.
	async fn get_ads<
		'ad_account_id,
		'date_preset,
		'effective_status,
		'time_range,
		'updated_since,
		'fields,
		'summary,
	>(
		&self,
		ad_account_id: &'ad_account_id str,
		date_preset: Option<&'date_preset str>,
		effective_status: Option<Vec<String>>,
		time_range: Option<models::TimeRange>,
		updated_since: Option<i32>,
		fields: Option<Vec<String>>,
		summary: Option<Vec<String>>,
	) -> Result<models::AdList, Error<GetAdsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/act_{ad_account_id}/ads",
			local_var_configuration.base_path,
			ad_account_id = crate::apis::urlencode(ad_account_id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = date_preset {
			local_var_req_builder = local_var_req_builder
				.query(&[("date_preset", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = effective_status {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(
					&local_var_str
						.iter()
						.map(|p| ("effective_status".to_owned(), p.to_string()))
						.collect::<Vec<(std::string::String, std::string::String)>>(
						),
				),
				_ => local_var_req_builder.query(&[(
					"effective_status",
					&local_var_str
						.iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(ref local_var_str) = time_range {
			local_var_req_builder = local_var_req_builder
				.query(&[("time_range", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = updated_since {
			local_var_req_builder = local_var_req_builder
				.query(&[("updated_since", &local_var_str.to_string())]);
		}
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
		if let Some(ref local_var_str) = summary {
			local_var_req_builder = match "csv" {
				"multi" => local_var_req_builder.query(
					&local_var_str
						.iter()
						.map(|p| ("summary".to_owned(), p.to_string()))
						.collect::<Vec<(std::string::String, std::string::String)>>(
						),
				),
				_ => local_var_req_builder.query(&[(
					"summary",
					&local_var_str
						.iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => {
					format!("{} {}", local_var_prefix, local_var_key)
				}
				None => local_var_key,
			};
			local_var_req_builder =
				local_var_req_builder.query(&[("access_token", local_var_value)]);
		}
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
			let local_var_entity: Option<GetAdsError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Updates an ad.
	async fn update_ad<'ad_id, 'properties>(
		&self,
		ad_id: &'ad_id str,
		properties: models::AdUpdate,
	) -> Result<models::AdResponse, Error<UpdateAdError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/{ad_id}",
			local_var_configuration.base_path,
			ad_id = crate::apis::urlencode(ad_id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		local_var_req_builder =
			local_var_req_builder.query(&[("properties", &properties.to_string())]);
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => {
					format!("{} {}", local_var_prefix, local_var_key)
				}
				None => local_var_key,
			};
			local_var_req_builder =
				local_var_req_builder.query(&[("access_token", local_var_value)]);
		}
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
			let local_var_entity: Option<UpdateAdError> =
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

/// struct for typed errors of method [`create_ad`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAdError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_ad`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAdError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ad`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAdError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAdsError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_ad`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAdError {
	UnknownValue(serde_json::Value),
}