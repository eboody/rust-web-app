/*
 * Coolify
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
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
pub trait ProjectsApi: Send + Sync {
	async fn create_project<'create_project_request>(
		&self,
		create_project_request: models::CreateProjectRequest,
	) -> Result<models::CreateProject201Response, Error<CreateProjectError>>;
	async fn delete_project_by_uuid<'uuid>(
		&self,
		uuid: &str,
	) -> Result<
		models::DeleteProjectByUuid200Response,
		Error<DeleteProjectByUuidError>,
	>;
	async fn get_environment_by_name<'uuid, 'environment_name>(
		&self,
		uuid: &'uuid str,
		environment_name: &'environment_name str,
	) -> Result<models::Environment, Error<GetEnvironmentByNameError>>;
	async fn get_project_by_uuid<'uuid>(
		&self,
		uuid: &'uuid str,
	) -> Result<models::Project, Error<GetProjectByUuidError>>;
	async fn list_projects(
		&self,
	) -> Result<Vec<models::Project>, Error<ListProjectsError>>;
	async fn update_project_by_uuid<'create_project_request>(
		&self,
		uuid: String,
		create_project_request: models::CreateProjectRequest,
	) -> Result<
		models::UpdateProjectByUuid201Response,
		Error<UpdateProjectByUuidError>,
	>;
}

pub struct ProjectsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl ProjectsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl ProjectsApi for ProjectsApiClient {
	/// Create Project.
	async fn create_project<'create_project_request>(
		&self,
		create_project_request: models::CreateProjectRequest,
	) -> Result<models::CreateProject201Response, Error<CreateProjectError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/projects", local_var_configuration.base_path);
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
		local_var_req_builder = local_var_req_builder.json(&create_project_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateProjectError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete project by UUID.
	async fn delete_project_by_uuid<'uuid>(
		&self,
		uuid: &str,
	) -> Result<
		models::DeleteProjectByUuid200Response,
		Error<DeleteProjectByUuidError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/projects/{uuid}",
			local_var_configuration.base_path,
			uuid = crate::apis::urlencode(uuid)
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
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<DeleteProjectByUuidError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get environment by name.
	async fn get_environment_by_name<'uuid, 'environment_name>(
		&self,
		uuid: &'uuid str,
		environment_name: &'environment_name str,
	) -> Result<models::Environment, Error<GetEnvironmentByNameError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/projects/{uuid}/{environment_name}",
			local_var_configuration.base_path,
			uuid = crate::apis::urlencode(uuid),
			environment_name = crate::apis::urlencode(environment_name)
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
			let local_var_entity: Option<GetEnvironmentByNameError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get project by UUID.
	async fn get_project_by_uuid<'uuid>(
		&self,
		uuid: &'uuid str,
	) -> Result<models::Project, Error<GetProjectByUuidError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/projects/{uuid}",
			local_var_configuration.base_path,
			uuid = crate::apis::urlencode(uuid)
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
			let local_var_entity: Option<GetProjectByUuidError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// List projects.
	async fn list_projects(
		&self,
	) -> Result<Vec<models::Project>, Error<ListProjectsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/projects", local_var_configuration.base_path);
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
			let local_var_entity: Option<ListProjectsError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update Project.
	async fn update_project_by_uuid<'create_project_request>(
		&self,
		uuid: String,
		create_project_request: models::CreateProjectRequest,
	) -> Result<
		models::UpdateProjectByUuid201Response,
		Error<UpdateProjectByUuidError>,
	> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/projects/{uuid}", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client
			.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
		local_var_req_builder = local_var_req_builder.json(&create_project_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error()
		{
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateProjectByUuidError> =
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

/// struct for typed errors of method [`create_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateProjectError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(models::InlineObject2),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`delete_project_by_uuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteProjectByUuidError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(models::InlineObject2),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_environment_by_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEnvironmentByNameError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(models::InlineObject2),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_project_by_uuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectByUuidError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`list_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListProjectsError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`update_project_by_uuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateProjectByUuidError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(models::InlineObject2),
	UnknownValue(json::Value),
}
