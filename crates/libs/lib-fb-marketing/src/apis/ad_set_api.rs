/*
 * Facebook Marketing API
 *
 * This is a generated connector for [Facebook Marketing API v12.0](https://developers.facebook.com/docs/marketing-apis) OpenAPI specification.  Facebook is an American online social media and social networking service owned by Facebook, Inc.Facebook Marketing  APIs are a collection of Graph API endpoints that can be used to help you advertise on Facebook. 
 *
 * The version of the OpenAPI document: v12.0
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
pub trait AdSetApi: Send + Sync {
    async fn create_ad_set<'ad_account_id, 'properties>(&self, ad_account_id: &'ad_account_id str, properties: models::AdSet) -> Result<models::AdSetResponse, Error<CreateAdSetError>>;
    async fn delete_ad_set<'ad_set_id>(&self, ad_set_id: &'ad_set_id str) -> Result<models::AdSetResponse, Error<DeleteAdSetError>>;
    async fn get_ad_set<'ad_set_id, 'date_preset, 'time_range, 'fields>(&self, ad_set_id: &'ad_set_id str, date_preset: Option<&'date_preset str>, time_range: Option<models::TimeRange>, fields: Option<Vec<String>>) -> Result<models::AdSet, Error<GetAdSetError>>;
    async fn get_ad_sets<'ad_account_id, 'date_preset, 'time_range, 'fields>(&self, ad_account_id: &'ad_account_id str, date_preset: Option<&'date_preset str>, time_range: Option<models::TimeRange>, fields: Option<Vec<String>>) -> Result<models::AdSetList, Error<GetAdSetsError>>;
    async fn update_ad_set<'ad_set_id, 'properties>(&self, ad_set_id: &'ad_set_id str, properties: models::AdSetUpdate) -> Result<models::AdSetResponse, Error<UpdateAdSetError>>;
}

pub struct AdSetApiClient {
    configuration: Arc<configuration::Configuration>
}

impl AdSetApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl AdSetApi for AdSetApiClient {
    /// Cerates an ad set.
    async fn create_ad_set<'ad_account_id, 'properties>(&self, ad_account_id: &'ad_account_id str, properties: models::AdSet) -> Result<models::AdSetResponse, Error<CreateAdSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/act_{ad_account_id}/adsets", local_var_configuration.base_path, ad_account_id=crate::apis::urlencode(ad_account_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("properties", &properties.to_string())]);
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.query(&[("access_token", local_var_value)]);
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
            let local_var_entity: Option<CreateAdSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deletes a ad set.
    async fn delete_ad_set<'ad_set_id>(&self, ad_set_id: &'ad_set_id str) -> Result<models::AdSetResponse, Error<DeleteAdSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/{ad_set_id}", local_var_configuration.base_path, ad_set_id=crate::apis::urlencode(ad_set_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.query(&[("access_token", local_var_value)]);
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
            let local_var_entity: Option<DeleteAdSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Return date related to an ad set.
    async fn get_ad_set<'ad_set_id, 'date_preset, 'time_range, 'fields>(&self, ad_set_id: &'ad_set_id str, date_preset: Option<&'date_preset str>, time_range: Option<models::TimeRange>, fields: Option<Vec<String>>) -> Result<models::AdSet, Error<GetAdSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/{ad_set_id}", local_var_configuration.base_path, ad_set_id=crate::apis::urlencode(ad_set_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = date_preset {
            local_var_req_builder = local_var_req_builder.query(&[("date_preset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = time_range {
            local_var_req_builder = local_var_req_builder.query(&[("time_range", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.query(&[("access_token", local_var_value)]);
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
            let local_var_entity: Option<GetAdSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns all ad sets from one ad account
    async fn get_ad_sets<'ad_account_id, 'date_preset, 'time_range, 'fields>(&self, ad_account_id: &'ad_account_id str, date_preset: Option<&'date_preset str>, time_range: Option<models::TimeRange>, fields: Option<Vec<String>>) -> Result<models::AdSetList, Error<GetAdSetsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/act_{ad_account_id}/adsets", local_var_configuration.base_path, ad_account_id=crate::apis::urlencode(ad_account_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = date_preset {
            local_var_req_builder = local_var_req_builder.query(&[("date_preset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = time_range {
            local_var_req_builder = local_var_req_builder.query(&[("time_range", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = fields {
            local_var_req_builder = match "csv" {
                "multi" => local_var_req_builder.query(&local_var_str.iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
                _ => local_var_req_builder.query(&[("fields", &local_var_str.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
            };
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.query(&[("access_token", local_var_value)]);
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
            let local_var_entity: Option<GetAdSetsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates an ad set.
    async fn update_ad_set<'ad_set_id, 'properties>(&self, ad_set_id: &'ad_set_id str, properties: models::AdSetUpdate) -> Result<models::AdSetResponse, Error<UpdateAdSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/{ad_set_id}", local_var_configuration.base_path, ad_set_id=crate::apis::urlencode(ad_set_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("properties", &properties.to_string())]);
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.query(&[("access_token", local_var_value)]);
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
            let local_var_entity: Option<UpdateAdSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`create_ad_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAdSetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_ad_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAdSetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ad_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAdSetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ad_sets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAdSetsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_ad_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAdSetError {
    UnknownValue(serde_json::Value),
}

