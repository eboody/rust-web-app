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
pub trait VersionsApi: Send + Sync {
	async fn compare_content_version<'id>(
		&self,
		id: &'id str,
	) -> Result<models::ServerInfo200Response, Error<CompareContentVersionError>>;
	async fn create_content_version<'fields, 'meta, 'versions>(
		&self,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		versions: Option<models::Versions>,
	) -> Result<
		models::CreateContentVersion200Response,
		Error<CreateContentVersionError>,
	>;
	async fn delete_content_version<'id>(
		&self,
		id: &'id str,
	) -> Result<(), Error<DeleteContentVersionError>>;
	async fn delete_content_versions(
		&self,
	) -> Result<(), Error<DeleteContentVersionsError>>;
	async fn get_content_version<'id, 'fields, 'meta>(
		&self,
		id: &'id str,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
	) -> Result<models::CreateContentVersion200Response, Error<GetContentVersionError>>;
	async fn get_content_versions<
		'fields,
		'limit,
		'offset,
		'meta,
		'sort,
		'filter,
		'search,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		offset: Option<i32>,
		meta: Option<&'meta str>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
	) -> Result<models::GetContentVersions200Response, Error<GetContentVersionsError>>;
	async fn promote_content_version<'id, 'promote_content_version_request>(
		&self,
		id: &'id str,
		promote_content_version_request: Option<
			models::PromoteContentVersionRequest,
		>,
	) -> Result<json::Value, Error<PromoteContentVersionError>>;
	async fn save_content_version<'id, 'body>(
		&self,
		id: &'id str,
		body: Option<json::Value>,
	) -> Result<json::Value, Error<SaveContentVersionError>>;
	async fn update_content_version<'id, 'fields, 'meta, 'versions>(
		&self,
		id: &'id str,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		versions: Option<models::Versions>,
	) -> Result<
		models::CreateContentVersion200Response,
		Error<UpdateContentVersionError>,
	>;
	async fn update_content_versions<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'update_content_versions_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
		update_content_versions_request: Option<
			models::UpdateContentVersionsRequest,
		>,
	) -> Result<
		models::GetContentVersions200Response,
		Error<UpdateContentVersionsError>,
	>;
}

pub struct VersionsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl VersionsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl VersionsApi for VersionsApiClient {
	/// Compare an existing Content Version with the main version of the item.
	async fn compare_content_version<'id>(
		&self,
		id: &'id str,
	) -> Result<models::ServerInfo200Response, Error<CompareContentVersionError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/versions/{id}/compare",
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
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CompareContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Create multiple new Content Versions.
	async fn create_content_version<'fields, 'meta, 'versions>(
		&self,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		versions: Option<models::Versions>,
	) -> Result<
		models::CreateContentVersion200Response,
		Error<CreateContentVersionError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/versions", local_var_configuration.base_path);
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
		local_var_req_builder = local_var_req_builder.json(&versions);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete an existing Content Version.
	async fn delete_content_version<'id>(
		&self,
		id: &'id str,
	) -> Result<(), Error<DeleteContentVersionError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/versions/{id}",
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
			let local_var_entity: Option<DeleteContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete multiple existing Content Versions.
	async fn delete_content_versions(
		&self,
	) -> Result<(), Error<DeleteContentVersionsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/versions", local_var_configuration.base_path);
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
			let local_var_entity: Option<DeleteContentVersionsError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve a single Content Version by unique identifier.
	async fn get_content_version<'id, 'fields, 'meta>(
		&self,
		id: &'id str,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
	) -> Result<models::CreateContentVersion200Response, Error<GetContentVersionError>>
	{
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/versions/{id}",
			local_var_configuration.base_path,
			id = crate::apis::urlencode(id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

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

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get all Content Versions.
	async fn get_content_versions<
		'fields,
		'limit,
		'offset,
		'meta,
		'sort,
		'filter,
		'search,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		offset: Option<i32>,
		meta: Option<&'meta str>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
	) -> Result<models::GetContentVersions200Response, Error<GetContentVersionsError>>
	{
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/versions", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
		if let Some(ref local_var_str) = offset {
			local_var_req_builder = local_var_req_builder
				.query(&[("offset", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = meta {
			local_var_req_builder =
				local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
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

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetContentVersionsError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Pass the current hash of the main version of the item (obtained from the `compare` endpoint) along with an optional array of field names of which the values are to be promoted (by default, all fields are selected).
	async fn promote_content_version<'id, 'promote_content_version_request>(
		&self,
		id: &'id str,
		promote_content_version_request: Option<
			models::PromoteContentVersionRequest,
		>,
	) -> Result<json::Value, Error<PromoteContentVersionError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/versions/{id}/promote",
			local_var_configuration.base_path,
			id = crate::apis::urlencode(id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		local_var_req_builder =
			local_var_req_builder.json(&promote_content_version_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<PromoteContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Save item changes to an existing Content Version.
	async fn save_content_version<'id, 'body>(
		&self,
		id: &'id str,
		body: Option<json::Value>,
	) -> Result<json::Value, Error<SaveContentVersionError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/versions/{id}/save",
			local_var_configuration.base_path,
			id = crate::apis::urlencode(id)
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		local_var_req_builder = local_var_req_builder.json(&body);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<SaveContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update an existing Content Version.
	async fn update_content_version<'id, 'fields, 'meta, 'versions>(
		&self,
		id: &'id str,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		versions: Option<models::Versions>,
	) -> Result<
		models::CreateContentVersion200Response,
		Error<UpdateContentVersionError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/versions/{id}",
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
		local_var_req_builder = local_var_req_builder.json(&versions);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateContentVersionError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update multiple Content Versions at the same time.
	async fn update_content_versions<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'update_content_versions_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
		update_content_versions_request: Option<
			models::UpdateContentVersionsRequest,
		>,
	) -> Result<
		models::GetContentVersions200Response,
		Error<UpdateContentVersionsError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/versions", local_var_configuration.base_path);
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
		local_var_req_builder =
			local_var_req_builder.json(&update_content_versions_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateContentVersionsError> =
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

/// struct for typed errors of method [`compare_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompareContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`create_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`delete_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`delete_content_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContentVersionsError {
	Status401(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_content_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContentVersionsError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`promote_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromoteContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`save_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SaveContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`update_content_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateContentVersionError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`update_content_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateContentVersionsError {
	Status401(models::GetAsset404Response),
	UnknownValue(json::Value),
}
