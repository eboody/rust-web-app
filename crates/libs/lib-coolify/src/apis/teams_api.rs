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
pub trait TeamsApi: Send + Sync {
	async fn get_current_team(
		&self,
	) -> Result<models::Team, Error<GetCurrentTeamError>>;
	async fn get_current_team_members(
		&self,
	) -> Result<Vec<models::User>, Error<GetCurrentTeamMembersError>>;
	async fn get_members_by_team_id<'id>(
		&self,
		id: i32,
	) -> Result<Vec<models::User>, Error<GetMembersByTeamIdError>>;
	async fn get_team_by_id<'id>(
		&self,
		id: i32,
	) -> Result<models::Team, Error<GetTeamByIdError>>;
	async fn list_teams(&self) -> Result<Vec<models::Team>, Error<ListTeamsError>>;
}

pub struct TeamsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl TeamsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

#[async_trait]
impl TeamsApi for TeamsApiClient {
	/// Get currently authenticated team.
	async fn get_current_team(
		&self,
	) -> Result<models::Team, Error<GetCurrentTeamError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/teams/current", local_var_configuration.base_path);
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
			let local_var_entity: Option<GetCurrentTeamError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get currently authenticated team members.
	async fn get_current_team_members(
		&self,
	) -> Result<Vec<models::User>, Error<GetCurrentTeamMembersError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/teams/current/members",
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
			json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetCurrentTeamMembersError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get members by TeamId.
	async fn get_members_by_team_id<'id>(
		&self,
		id: i32,
	) -> Result<Vec<models::User>, Error<GetMembersByTeamIdError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/teams/{id}/members",
			local_var_configuration.base_path,
			id = id
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
			let local_var_entity: Option<GetMembersByTeamIdError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get team by TeamId.
	async fn get_team_by_id<'id>(
		&self,
		id: i32,
	) -> Result<models::Team, Error<GetTeamByIdError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/teams/{id}", local_var_configuration.base_path, id = id);
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
			let local_var_entity: Option<GetTeamByIdError> =
				json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Get all teams.
	async fn list_teams(&self) -> Result<Vec<models::Team>, Error<ListTeamsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str =
			format!("{}/teams", local_var_configuration.base_path);
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
			let local_var_entity: Option<ListTeamsError> =
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

/// struct for typed errors of method [`get_current_team`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentTeamError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_current_team_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentTeamMembersError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_members_by_team_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMembersByTeamIdError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(models::InlineObject2),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`get_team_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamByIdError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	Status404(models::InlineObject2),
	UnknownValue(json::Value),
}

/// struct for typed errors of method [`list_teams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTeamsError {
	Status401(models::InlineObject1),
	Status400(models::InlineObject),
	UnknownValue(json::Value),
}
