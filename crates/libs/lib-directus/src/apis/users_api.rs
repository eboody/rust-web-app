/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

#[async_trait]
pub trait UsersApi: Send + Sync {
    async fn accept_invite<'accept_invite_request>(&self, accept_invite_request: models::AcceptInviteRequest) -> Result<models::CreateUser200Response, Error<AcceptInviteError>>;
    async fn create_user<'meta, 'users>(&self, meta: Option<&'meta str>, users: Option<models::Users>) -> Result<models::CreateUser200Response, Error<CreateUserError>>;
    async fn delete_user<'id>(&self, id: &'id str) -> Result<(), Error<DeleteUserError>>;
    async fn delete_users<>(&self, ) -> Result<(), Error<DeleteUsersError>>;
    async fn get_me<'fields, 'meta>(&self, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreateUser200Response, Error<GetMeError>>;
    async fn get_user<'id, 'fields, 'meta>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreateUser200Response, Error<GetUserError>>;
    async fn get_users<'fields, 'limit, 'offset, 'meta, 'sort, 'filter, 'search>(&self, fields: Option<Vec<String>>, limit: Option<i32>, offset: Option<i32>, meta: Option<&'meta str>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>) -> Result<models::GetUsers200Response, Error<GetUsersError>>;
    async fn invite<'invite_request>(&self, invite_request: Option<models::InviteRequest>) -> Result<models::CreateUser200Response, Error<InviteError>>;
    async fn me_tfa_disable<>(&self, ) -> Result<(), Error<MeTfaDisableError>>;
    async fn me_tfa_enable<>(&self, ) -> Result<(), Error<MeTfaEnableError>>;
    async fn update_last_used_page_me<'update_last_used_page_me_request>(&self, update_last_used_page_me_request: Option<models::UpdateLastUsedPageMeRequest>) -> Result<(), Error<UpdateLastUsedPageMeError>>;
    async fn update_me<>(&self, ) -> Result<models::CreateUser200Response, Error<UpdateMeError>>;
    async fn update_user<'id, 'fields, 'meta, 'users>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>, users: Option<models::Users>) -> Result<models::ServerInfo200Response, Error<UpdateUserError>>;
    async fn update_users<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'update_users_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, update_users_request: Option<models::UpdateUsersRequest>) -> Result<models::GetUsers200Response, Error<UpdateUsersError>>;
}

pub struct UsersApiClient {
    configuration: Arc<configuration::Configuration>
}

impl UsersApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl UsersApi for UsersApiClient {
    /// Accepts and enables an invited user using a JWT invitation token.
    async fn accept_invite<'accept_invite_request>(&self, accept_invite_request: models::AcceptInviteRequest) -> Result<models::CreateUser200Response, Error<AcceptInviteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/invite/accept", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&accept_invite_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<AcceptInviteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Create a new user.
    async fn create_user<'meta, 'users>(&self, meta: Option<&'meta str>, users: Option<models::Users>) -> Result<models::CreateUser200Response, Error<CreateUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&users);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete an existing user
    async fn delete_user<'id>(&self, id: &'id str) -> Result<(), Error<DeleteUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete multiple existing users.
    async fn delete_users<>(&self, ) -> Result<(), Error<DeleteUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve the currently authenticated user.
    async fn get_me<'fields, 'meta>(&self, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreateUser200Response, Error<GetMeError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetMeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve a single user by unique identifier.
    async fn get_user<'id, 'fields, 'meta>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>) -> Result<models::CreateUser200Response, Error<GetUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List the users.
    async fn get_users<'fields, 'limit, 'offset, 'meta, 'sort, 'filter, 'search>(&self, fields: Option<Vec<String>>, limit: Option<i32>, offset: Option<i32>, meta: Option<&'meta str>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>) -> Result<models::GetUsers200Response, Error<GetUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = offset {
            local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("sort".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("sort", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = filter {
            local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Invites one or more users to this project. It creates a user with an invited status, and then sends an email to the user with instructions on how to activate their account.
    async fn invite<'invite_request>(&self, invite_request: Option<models::InviteRequest>) -> Result<models::CreateUser200Response, Error<InviteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/invite", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&invite_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<InviteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Disables two-factor authentication for the currently authenticated user.
    async fn me_tfa_disable<>(&self, ) -> Result<(), Error<MeTfaDisableError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/me/tfa/disable", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<MeTfaDisableError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Enables two-factor authentication for the currently authenticated user.
    async fn me_tfa_enable<>(&self, ) -> Result<(), Error<MeTfaEnableError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/me/tfa/enable", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<MeTfaEnableError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates the last used page field of the currently authenticated user. This is used internally to be able to open the Directus admin app from the last page you used.
    async fn update_last_used_page_me<'update_last_used_page_me_request>(&self, update_last_used_page_me_request: Option<models::UpdateLastUsedPageMeRequest>) -> Result<(), Error<UpdateLastUsedPageMeError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/me/track/page", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&update_last_used_page_me_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<UpdateLastUsedPageMeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update the currently authenticated user.
    async fn update_me<>(&self, ) -> Result<models::CreateUser200Response, Error<UpdateMeError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateMeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update an existing user
    async fn update_user<'id, 'fields, 'meta, 'users>(&self, id: &'id str, fields: Option<Vec<String>>, meta: Option<&'meta str>, users: Option<models::Users>) -> Result<models::ServerInfo200Response, Error<UpdateUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&users);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update multiple users at the same time.
    async fn update_users<'fields, 'limit, 'meta, 'offset, 'sort, 'filter, 'search, 'update_users_request>(&self, fields: Option<Vec<String>>, limit: Option<i32>, meta: Option<&'meta str>, offset: Option<i32>, sort: Option<Vec<String>>, filter: Option<models::serde_json::Value>, search: Option<&'search str>, update_users_request: Option<models::UpdateUsersRequest>) -> Result<models::GetUsers200Response, Error<UpdateUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = meta {
            local_var_req_builder = local_var_req_builder.query(&[("meta", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = offset {
            local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("sort".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("sort", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_str) = filter {
            local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&update_users_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`accept_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptInviteError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUsersError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMeError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`me_tfa_disable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MeTfaDisableError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`me_tfa_enable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MeTfaEnableError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_last_used_page_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLastUsedPageMeError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMeError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    Status401(models::GetAsset404Response),
    Status404(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUsersError {
    Status401(models::GetAsset404Response),
    UnknownValue(serde_json::Value),
}

