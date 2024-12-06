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
pub trait ItemsArticlesTranslationsApi: Send + Sync {
	async fn create_items_articles_translations<
		'meta,
		'create_items_articles_translations_request,
	>(
		&self,
		meta: Option<&'meta str>,
		create_items_articles_translations_request: Option<
			models::CreateItemsArticlesTranslationsRequest,
		>,
	) -> Result<
		models::CreateItemsArticlesTranslations200Response,
		Error<CreateItemsArticlesTranslationsError>,
	>;
	async fn delete_items_articles_translations(
		&self,
	) -> Result<(), Error<DeleteItemsArticlesTranslationsError>>;
	async fn delete_single_items_articles_translations<'id>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
	) -> Result<(), Error<DeleteSingleItemsArticlesTranslationsError>>;
	async fn read_items_articles_translations<
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
		filter: Option<models::serde_json::Value>,
		search: Option<&'search str>,
	) -> Result<
		models::ReadItemsArticlesTranslations200Response,
		Error<ReadItemsArticlesTranslationsError>,
	>;
	async fn read_single_items_articles_translations<'id, 'fields, 'meta, 'version>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		version: Option<&'version str>,
	) -> Result<
		models::ReadSingleItemsArticlesTranslations200Response,
		Error<ReadSingleItemsArticlesTranslationsError>,
	>;
	async fn update_items_articles_translations<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'create_items_articles_translations_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::serde_json::Value>,
		search: Option<&'search str>,
		create_items_articles_translations_request: Option<
			models::CreateItemsArticlesTranslationsRequest,
		>,
	) -> Result<
		models::UpdateItemsArticlesTranslations200Response,
		Error<UpdateItemsArticlesTranslationsError>,
	>;
	async fn update_single_items_articles_translations<
		'id,
		'fields,
		'meta,
		'items_articles_translations,
	>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		items_articles_translations: Option<models::ItemsArticlesTranslations>,
	) -> Result<
		models::ReadSingleItemsArticlesTranslations200Response,
		Error<UpdateSingleItemsArticlesTranslationsError>,
	>;
}

pub struct ItemsArticlesTranslationsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl ItemsArticlesTranslationsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl ItemsArticlesTranslationsApi for ItemsArticlesTranslationsApiClient {
	/// Create a new articles_translations item.
	async fn create_items_articles_translations<
		'meta,
		'create_items_articles_translations_request,
	>(
		&self,
		meta: Option<&'meta str>,
		create_items_articles_translations_request: Option<
			models::CreateItemsArticlesTranslationsRequest,
		>,
	) -> Result<
		models::CreateItemsArticlesTranslations200Response,
		Error<CreateItemsArticlesTranslationsError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations",
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
		local_var_req_builder =
			local_var_req_builder.json(&create_items_articles_translations_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateItemsArticlesTranslationsError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete multiple existing articles_translations items.
	async fn delete_items_articles_translations(
		&self,
	) -> Result<(), Error<DeleteItemsArticlesTranslationsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations",
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
			let local_var_entity: Option<DeleteItemsArticlesTranslationsError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete an existing articles_translations item.
	async fn delete_single_items_articles_translations<'id>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
	) -> Result<(), Error<DeleteSingleItemsArticlesTranslationsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations/{id}",
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
				DeleteSingleItemsArticlesTranslationsError,
			> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// List the articles_translations items.
	async fn read_items_articles_translations<
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
		filter: Option<models::serde_json::Value>,
		search: Option<&'search str>,
	) -> Result<
		models::ReadItemsArticlesTranslations200Response,
		Error<ReadItemsArticlesTranslationsError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations",
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<ReadItemsArticlesTranslationsError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve a single articles_translations item by unique identifier.
	async fn read_single_items_articles_translations<
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
		models::ReadSingleItemsArticlesTranslations200Response,
		Error<ReadSingleItemsArticlesTranslationsError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations/{id}",
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
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<ReadSingleItemsArticlesTranslationsError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update multiple articles_translations items at the same time.
	async fn update_items_articles_translations<
		'fields,
		'limit,
		'meta,
		'offset,
		'sort,
		'filter,
		'search,
		'create_items_articles_translations_request,
	>(
		&self,
		fields: Option<Vec<String>>,
		limit: Option<i32>,
		meta: Option<&'meta str>,
		offset: Option<i32>,
		sort: Option<Vec<String>>,
		filter: Option<models::serde_json::Value>,
		search: Option<&'search str>,
		create_items_articles_translations_request: Option<
			models::CreateItemsArticlesTranslationsRequest,
		>,
	) -> Result<
		models::UpdateItemsArticlesTranslations200Response,
		Error<UpdateItemsArticlesTranslationsError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations",
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
		local_var_req_builder =
			local_var_req_builder.json(&create_items_articles_translations_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateItemsArticlesTranslationsError> =
				serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update an existing articles_translations item.
	async fn update_single_items_articles_translations<
		'id,
		'fields,
		'meta,
		'items_articles_translations,
	>(
		&self,
		id: models::ReadSingleItemsLanguagesIdParameter,
		fields: Option<Vec<String>>,
		meta: Option<&'meta str>,
		items_articles_translations: Option<models::ItemsArticlesTranslations>,
	) -> Result<
		models::ReadSingleItemsArticlesTranslations200Response,
		Error<UpdateSingleItemsArticlesTranslationsError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/items/articles_translations/{id}",
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
			local_var_req_builder.json(&items_articles_translations);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<
				UpdateSingleItemsArticlesTranslationsError,
			> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}

/// struct for typed errors of method [`create_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateItemsArticlesTranslationsError {
	Status401(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteItemsArticlesTranslationsError {
	Status401(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_single_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSingleItemsArticlesTranslationsError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadItemsArticlesTranslationsError {
	Status401(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_single_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadSingleItemsArticlesTranslationsError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateItemsArticlesTranslationsError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_single_items_articles_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSingleItemsArticlesTranslationsError {
	Status401(models::GetAsset404Response),
	Status404(models::GetAsset404Response),
	UnknownValue(serde_json::Value),
}
