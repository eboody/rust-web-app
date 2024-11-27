/*
 * Dynamic API Specification
 *
 * This is a dynamically generated API specification for all endpoints existing on the current project.
 *
 * The version of the OpenAPI document: 11.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemsArticlesDirectusUsers {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "articles_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub articles_id: Option<Option<Box<models::ItemsArticlesDirectusUsersArticlesId>>>,
    #[serde(rename = "directus_users_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub directus_users_id: Option<Option<Box<models::FilesModifiedBy>>>,
}

impl ItemsArticlesDirectusUsers {
    pub fn new() -> ItemsArticlesDirectusUsers {
        ItemsArticlesDirectusUsers {
            id: None,
            articles_id: None,
            directus_users_id: None,
        }
    }
}

