/*
 * AnythingLLM Developer API
 *
 * API endpoints that enable programmatic reading, writing, and updating of your AnythingLLM instance. UI supplied by Swagger.io.
 *
 * The version of the OpenAPI document: 1.0.0
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
pub trait AdminApi: Send + Sync {
    async fn v1_admin_invite_id_delete<'id>(&self, id: &'id str) -> Result<serde_json::Value, Error<V1AdminInviteIdDeleteError>>;
    async fn v1_admin_invite_new_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminInviteNewPostError>>;
    async fn v1_admin_invites_get<>(&self, ) -> Result<serde_json::Value, Error<V1AdminInvitesGetError>>;
    async fn v1_admin_is_multi_user_mode_get<>(&self, ) -> Result<serde_json::Value, Error<V1AdminIsMultiUserModeGetError>>;
    async fn v1_admin_preferences_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminPreferencesPostError>>;
    async fn v1_admin_users_get<>(&self, ) -> Result<serde_json::Value, Error<V1AdminUsersGetError>>;
    async fn v1_admin_users_id_delete<'id>(&self, id: &'id str) -> Result<serde_json::Value, Error<V1AdminUsersIdDeleteError>>;
    async fn v1_admin_users_id_post<'id>(&self, id: &'id str) -> Result<serde_json::Value, Error<V1AdminUsersIdPostError>>;
    async fn v1_admin_users_new_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminUsersNewPostError>>;
    async fn v1_admin_workspace_chats_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminWorkspaceChatsPostError>>;
    async fn v1_admin_workspaces_workspace_id_update_users_post<'workspace_id>(&self, workspace_id: &'workspace_id str) -> Result<serde_json::Value, Error<V1AdminWorkspacesWorkspaceIdUpdateUsersPostError>>;
    async fn v1_admin_workspaces_workspace_id_users_get<'workspace_id>(&self, workspace_id: &'workspace_id str) -> Result<serde_json::Value, Error<V1AdminWorkspacesWorkspaceIdUsersGetError>>;
}

pub struct AdminApiClient {
    configuration: Arc<configuration::Configuration>
}

impl AdminApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl AdminApi for AdminApiClient {
    /// Deactivates (soft-delete) invite by id. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_invite_id_delete<'id>(&self, id: &'id str) -> Result<serde_json::Value, Error<V1AdminInviteIdDeleteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/invite/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminInviteIdDeleteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Create a new invite code for someone to use to register with instance. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_invite_new_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminInviteNewPostError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/invite/new", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminInviteNewPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// List all existing invitations to instance regardless of status. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_invites_get<>(&self, ) -> Result<serde_json::Value, Error<V1AdminInvitesGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/invites", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminInvitesGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Check to see if the instance is in multi-user-mode first. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_is_multi_user_mode_get<>(&self, ) -> Result<serde_json::Value, Error<V1AdminIsMultiUserModeGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/is-multi-user-mode", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminIsMultiUserModeGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update multi-user preferences for instance. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_preferences_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminPreferencesPostError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/preferences", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminPreferencesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Check to see if the instance is in multi-user-mode first. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_users_get<>(&self, ) -> Result<serde_json::Value, Error<V1AdminUsersGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/users", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminUsersGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete existing user by id. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_users_id_delete<'id>(&self, id: &'id str) -> Result<serde_json::Value, Error<V1AdminUsersIdDeleteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/users/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminUsersIdDeleteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Update existing user settings. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_users_id_post<'id>(&self, id: &'id str) -> Result<serde_json::Value, Error<V1AdminUsersIdPostError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/users/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminUsersIdPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Create a new user with username and password. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_users_new_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminUsersNewPostError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/users/new", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminUsersNewPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// All chats in the system ordered by most recent. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_workspace_chats_post<>(&self, ) -> Result<serde_json::Value, Error<V1AdminWorkspaceChatsPostError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/workspace-chats", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminWorkspaceChatsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Overwrite workspace permissions to only be accessible by the given user ids and admins. Methods are disabled until multi user mode is enabled via the UI.
    async fn v1_admin_workspaces_workspace_id_update_users_post<'workspace_id>(&self, workspace_id: &'workspace_id str) -> Result<serde_json::Value, Error<V1AdminWorkspacesWorkspaceIdUpdateUsersPostError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/workspaces/{workspaceId}/update-users", local_var_configuration.base_path, workspaceId=crate::apis::urlencode(workspace_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminWorkspacesWorkspaceIdUpdateUsersPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve a list of users with permissions to access the specified workspace.
    async fn v1_admin_workspaces_workspace_id_users_get<'workspace_id>(&self, workspace_id: &'workspace_id str) -> Result<serde_json::Value, Error<V1AdminWorkspacesWorkspaceIdUsersGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/admin/workspaces/{workspaceId}/users", local_var_configuration.base_path, workspaceId=crate::apis::urlencode(workspace_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<V1AdminWorkspacesWorkspaceIdUsersGetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`v1_admin_invite_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminInviteIdDeleteError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_invite_new_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminInviteNewPostError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_invites_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminInvitesGetError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_is_multi_user_mode_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminIsMultiUserModeGetError {
    Status403(models::InvalidApiKey),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_preferences_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminPreferencesPostError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_users_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminUsersGetError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_users_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminUsersIdDeleteError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_users_id_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminUsersIdPostError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_users_new_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminUsersNewPostError {
    Status400(),
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_workspace_chats_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminWorkspaceChatsPostError {
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_workspaces_workspace_id_update_users_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminWorkspacesWorkspaceIdUpdateUsersPostError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_admin_workspaces_workspace_id_users_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1AdminWorkspacesWorkspaceIdUsersGetError {
    Status401(),
    Status403(models::InvalidApiKey),
    Status500(),
    UnknownValue(serde_json::Value),
}
