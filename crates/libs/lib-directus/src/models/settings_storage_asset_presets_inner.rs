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
pub struct SettingsStorageAssetPresetsInner {
    /// Key for the asset. Used in the assets endpoint.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Whether to crop the thumbnail to match the size, or maintain the aspect ratio.
    #[serde(rename = "fit", skip_serializing_if = "Option::is_none")]
    pub fit: Option<Fit>,
    /// Width of the thumbnail.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Height of the thumbnail.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// No image upscale
    #[serde(rename = "withoutEnlargement", skip_serializing_if = "Option::is_none")]
    pub without_enlargement: Option<bool>,
    /// Quality of the compression used.
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Reformat output image
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    /// Additional transformations to apply
    #[serde(rename = "transforms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Option<Vec<models::SettingsStorageAssetPresetsInnerTransformsInner>>>,
}

impl SettingsStorageAssetPresetsInner {
    pub fn new() -> SettingsStorageAssetPresetsInner {
        SettingsStorageAssetPresetsInner {
            key: None,
            fit: None,
            width: None,
            height: None,
            without_enlargement: None,
            quality: None,
            format: None,
            transforms: None,
        }
    }
}
/// Whether to crop the thumbnail to match the size, or maintain the aspect ratio.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Fit {
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "contain")]
    Contain,
    #[serde(rename = "inside")]
    Inside,
    #[serde(rename = "outside")]
    Outside,
}

impl Default for Fit {
    fn default() -> Fit {
        Self::Cover
    }
}
/// Reformat output image
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "tiff")]
    Tiff,
    #[serde(rename = "avif")]
    Avif,
}

impl Default for Format {
    fn default() -> Format {
        Self::Empty
    }
}
