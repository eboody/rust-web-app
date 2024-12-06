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

/// SettingsPublicFavicon : $t:field_options.directus_settings.project_favicon_note
/// $t:field_options.directus_settings.project_favicon_note
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SettingsPublicFavicon {
	String(uuid::Uuid),
	Files(Box<models::Files>),
}

impl Default for SettingsPublicFavicon {
	fn default() -> Self {
		Self::String(Default::default())
	}
}
