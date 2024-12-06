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
pub trait ItemsArticlesDirectusUsersApi: Send + Sync {
	async fn create_items_articles_directus_users<
		'meta,
		'create_items_articles_directus_users_request,
	>(
		&self,
		meta: Option<&'meta str>,
		create_items_articles_directus_users_request: Option<
			models::CreateItemsArticlesDirectusUsersRequest,
		>,
	) -> Result<
		models::CreateItemsArticlesDirectusUsers200Response,
		Error<CreateItemsArticlesDirectusUsersError>,
	>;
	async fn delete_items_articles_directus_users(
		&self,
	) -> Result<(), Error<DeleteItemsArticlesDirectusUsersError>>;
	async fn delete_single_items_articles_directus_users<'id>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
	) -> Result<(), Error<DeleteSingleItemsArticlesDirectusUsersError>>;
	async fn read_items_articles_directus_users<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
	) -> Result<
		models::ReadItemsArticlesDirectusUsers200Response,
		Error<ReadItemsArticlesDirectusUsersError>,
	>;
	async fn read_single_items_articles_directus_users<
		'id,
		'fields,
		'meta,
		'version,
	>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		version: Option<&'version str>,
	) -> Result<
		models::ReadSingleItemsArticlesDirectusUsers200Response,
		Error<ReadSingleItemsArticlesDirectusUsersError>,
	>;
	async fn update_items_articles_directus_users<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'create_items_articles_directus_users_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
		create_items_articles_directus_users_request: Option<
			models::CreateItemsArticlesDirectusUsersRequest,
		>,
	) -> Result<
		models::UpdateItemsArticlesDirectusUsers200Response,
		Error<UpdateItemsArticlesDirectusUsersError>,
	>;
	async fn update_single_items_articles_directus_users<
		'id,
		'fields,
		'meta,
		'items_articles_directus_users,
	>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		items_articles_directus_users: Option<models::ItemsArticlesDirectusUsers>,
	) -> Result<
		models::ReadSingleItemsArticlesDirectusUsers200Response,
		Error<UpdateSingleItemsArticlesDirectusUsersError>,
	>;
}

pub struct ItemsArticlesDirectusUsersApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl ItemsArticlesDirectusUsersApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl ItemsArticlesDirectusUsersApi for ItemsArticlesDirectusUsersApiClient {
	/// Create a new articles_directus_users item.
	async fn create_items_articles_directus_users<
		'meta,
		'create_items_articles_directus_users_request,
	>(
		&self,
		meta: Option<&'meta str>,
		create_items_articles_directus_users_request: Option<
			models::CreateItemsArticlesDirectusUsersRequest,
		>,
	) -> Result<
		models::CreateItemsArticlesDirectusUsers200Response,
		Error<CreateItemsArticlesDirectusUsersError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users",
			local_var_configuration.base_path
		);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = meta {
			local_var_req_builder =
				local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder
				.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		local_var_req_builder = local_var_req_builder
			.json(&create_items_articles_directus_users_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateItemsArticlesDirectusUsersError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete multiple existing articles_directus_users items.
	async fn delete_items_articles_directus_users(
		&self,
	) -> Result<(), Error<DeleteItemsArticlesDirectusUsersError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users",
			local_var_configuration.base_path
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
			let local_var_entity: Option<DeleteItemsArticlesDirectusUsersError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete an existing articles_directus_users item.
	async fn delete_single_items_articles_directus_users<'id>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
	) -> Result<(), Error<DeleteSingleItemsArticlesDirectusUsersError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users/{id}",
			local_var_configuration.base_path,
			id = id.to_string()
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
			let local_var_entity: Option<
				DeleteSingleItemsArticlesDirectusUsersError,
			> = json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// List the articles_directus_users items.
	async fn read_items_articles_directus_users<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
	) -> Result<
		models::ReadItemsArticlesDirectusUsers200Response,
		Error<ReadItemsArticlesDirectusUsersError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users",
			local_var_configuration.base_path
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
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => {
					format!("{} {}", local_var_prefix, local_var_key)
				}
				None => local_var_key,
			};
			local_var_req_builder =
				local_var_req_builder.header("Authorization", local_var_value);
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<ReadItemsArticlesDirectusUsersError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve a single articles_directus_users item by unique identifier.
	async fn read_single_items_articles_directus_users<
		'id,
		'fields,
		'meta,
		'version,
	>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		version: Option<&'version str>,
	) -> Result<
		models::ReadSingleItemsArticlesDirectusUsers200Response,
		Error<ReadSingleItemsArticlesDirectusUsersError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users/{id}",
			local_var_configuration.base_path,
			id = id.to_string()
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
		if let Some(ref local_var_str) = version {
			local_var_req_builder = local_var_req_builder
				.query(&[("version", &local_var_str.to_string())]);
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
			let local_var_entity: Option<ReadSingleItemsArticlesDirectusUsersError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update multiple articles_directus_users items at the same time.
	async fn update_items_articles_directus_users<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'create_items_articles_directus_users_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::json::Value>,
		search: Option<&'search str>,
		create_items_articles_directus_users_request: Option<
			models::CreateItemsArticlesDirectusUsersRequest,
		>,
	) -> Result<
		models::UpdateItemsArticlesDirectusUsers200Response,
		Error<UpdateItemsArticlesDirectusUsersError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users",
			local_var_configuration.base_path
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
		local_var_req_builder = local_var_req_builder
			.json(&create_items_articles_directus_users_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateItemsArticlesDirectusUsersError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update an existing articles_directus_users item.
	async fn update_single_items_articles_directus_users<
		'id,
		'fields,
		'meta,
		'items_articles_directus_users,
	>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		items_articles_directus_users: Option<models::ItemsArticlesDirectusUsers>,
	) -> Result<
		models::ReadSingleItemsArticlesDirectusUsers200Response,
		Error<UpdateSingleItemsArticlesDirectusUsersError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_directus_users/{id}",
			local_var_configuration.base_path,
			id = id.to_string()
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
		local_var_req_builder =
			local_var_req_builder.json(&items_articles_directus_users);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<
				UpdateSingleItemsArticlesDirectusUsersError,
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

/// struct for typed errors of method [`create_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateItemsArticlesDirectusUsersError {
	Status401(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`delete_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteItemsArticlesDirectusUsersError {
	Status401(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`delete_single_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSingleItemsArticlesDirectusUsersError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`read_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadItemsArticlesDirectusUsersError {
	Status401(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`read_single_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadSingleItemsArticlesDirectusUsersError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`update_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateItemsArticlesDirectusUsersError {
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`update_single_items_articles_directus_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSingleItemsArticlesDirectusUsersError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(json::Value),
}
