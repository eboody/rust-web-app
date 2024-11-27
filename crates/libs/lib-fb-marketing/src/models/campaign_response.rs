/*
 * Facebook Marketing API
 *
 * This is a generated connector for [Facebook Marketing API v12.0](https://developers.facebook.com/docs/marketing-apis) OpenAPI specification.  Facebook is an American online social media and social networking service owned by Facebook, Inc.Facebook Marketing  APIs are a collection of Graph API endpoints that can be used to help you advertise on Facebook. 
 *
 * The version of the OpenAPI document: v12.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CampaignResponse : Campaign operation response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignResponse {
    /// Campaign ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Sucess status
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl CampaignResponse {
    /// Campaign operation response
    pub fn new() -> CampaignResponse {
        CampaignResponse {
            id: None,
            success: None,
        }
    }
}

