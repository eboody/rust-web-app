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
pub struct CreatePresetRequest {
    /// What collection this collection preset is used for.
    #[serde(rename = "collection")]
    pub collection: String,
    /// Name for the bookmark. If this is set, the collection preset will be considered to be a bookmark.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The unique identifier of a role in the platform. If user is null, this will be used to apply the collection preset or bookmark for all users in the role.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// What the user searched for in search/filter in the header bar.
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<models::CreatePresetRequestFiltersInner>>,
    /// Name of the view type that is used.
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    /// Layout query that's saved per layout type. Controls what data is fetched on load. These follow the same format as the JS SDK parameters.
    #[serde(rename = "layout_query", skip_serializing_if = "Option::is_none")]
    pub layout_query: Option<String>,
    /// Options of the views. The properties in here are controlled by the layout.
    #[serde(rename = "layout_options", skip_serializing_if = "Option::is_none")]
    pub layout_options: Option<String>,
}

impl CreatePresetRequest {
    pub fn new(collection: String) -> CreatePresetRequest {
        CreatePresetRequest {
            collection,
            title: None,
            role: None,
            search: None,
            filters: None,
            layout: None,
            layout_query: None,
            layout_options: None,
        }
    }
}

