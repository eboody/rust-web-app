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

/// RolesParent : $t:field_options.directus_roles.parent_note
/// $t:field_options.directus_roles.parent_note
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesParent {
    String(uuid::Uuid),
    Roles(Box<models::Roles>),
}

impl Default for RolesParent {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
