use serde::{Deserialize, Serialize};
use std::str::FromStr;
use time::OffsetDateTime;
pub type Json = json::Value;
use ormlite::types::Uuid;

#[derive(Debug, ormlite::Model)]
pub struct SqlxMigrations {
    #[ormlite(primary_key)]
    pub version: i64,
    pub description: String,
    pub installed_on: OffsetDateTime,
    pub success: bool,
    pub checksum: Vec<u8>,
    pub execution_time: i64,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_access")]
pub struct Access {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub role: Option<Uuid>,
    pub user: Option<Uuid>,
    pub policy: Uuid,
    pub sort: Option<i32>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_activity")]
pub struct Activity {
    #[ormlite(primary_key)]
    pub id: i32,
    pub action: String,
    pub user: Option<Uuid>,
    pub timestamp: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub collection: String,
    pub item: String,
    pub comment: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_collections")]
pub struct Collections {
    #[ormlite(primary_key)]
    pub collection: String,
    pub icon: Option<String>,
    pub note: Option<String>,
    pub display_template: Option<String>,
    pub hidden: bool,
    pub singleton: bool,
    pub translations: Option<Json>,
    pub archive_field: Option<String>,
    pub archive_app_filter: bool,
    pub archive_value: Option<String>,
    pub unarchive_value: Option<String>,
    pub sort_field: Option<String>,
    pub accountability: Option<String>,
    pub color: Option<String>,
    pub item_duplication_fields: Option<Json>,
    pub sort: Option<i32>,
    pub group: Option<String>,
    pub collapse: String,
    pub preview_url: Option<String>,
    pub versioning: bool,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_comments")]
pub struct Comments {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub collection: String,
    pub item: String,
    pub comment: String,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub user_created: Option<Uuid>,
    pub user_updated: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_dashboards")]
pub struct Dashboards {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub note: Option<String>,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
    pub color: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_extensions")]
pub struct Extensions {
    #[ormlite(primary_key)]
    pub enabled: bool,
    pub id: Uuid,
    pub folder: String,
    pub source: String,
    pub bundle: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_fields")]
pub struct Fields {
    #[ormlite(primary_key)]
    pub id: i32,
    pub collection: String,
    pub field: String,
    pub special: Option<String>,
    pub interface: Option<String>,
    pub options: Option<Json>,
    pub display: Option<String>,
    pub display_options: Option<Json>,
    pub readonly: bool,
    pub hidden: bool,
    pub sort: Option<i32>,
    pub width: Option<String>,
    pub translations: Option<Json>,
    pub note: Option<String>,
    pub conditions: Option<Json>,
    pub required: Option<bool>,
    pub group: Option<String>,
    pub validation: Option<Json>,
    pub validation_message: Option<String>,
}

#[derive(Debug, ormlite::Model, Clone, Serialize)]
#[ormlite(table = "directus_files")]
pub struct Files {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub storage: Option<String>,
    pub filename_disk: Option<String>,
    pub filename_download: String,
    pub title: Option<String>,
    #[ormlite(column = "type")]
    pub type_: Option<String>,
    pub folder: Option<Uuid>,
    pub uploaded_by: Option<Uuid>,
    pub created_on: OffsetDateTime,
    pub modified_by: Option<Uuid>,
    pub modified_on: OffsetDateTime,
    pub charset: Option<String>,
    pub filesize: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub embed: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub tags: Option<json::Value>,
    pub metadata: Option<Json>,
    pub focal_point_x: Option<i32>,
    pub focal_point_y: Option<i32>,
    pub tus_id: Option<String>,
    pub tus_data: Option<Json>,
    pub uploaded_on: Option<OffsetDateTime>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_flows")]
pub struct Flows {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub trigger: Option<String>,
    pub accountability: Option<String>,
    pub options: Option<Json>,
    pub operation: Option<Uuid>,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_folders")]
pub struct Folders {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub parent: Option<Uuid>,
}

//#[derive(Debug, ormlite::Model)]
//#[ormlite(table = "directus_migrations")]
//pub struct Migrations {
//	#[ormlite(primary_key)]
//	pub version: String,
//	pub name: String,
//	pub timestamp: Option<String>,
//}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_notifications")]
pub struct Notifications {
    #[ormlite(primary_key)]
    pub id: i32,
    pub timestamp: Option<String>,
    pub status: Option<String>,
    pub recipient: Uuid,
    pub sender: Option<Uuid>,
    pub subject: String,
    pub message: Option<String>,
    pub collection: Option<String>,
    pub item: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_operations")]
pub struct Operations {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: Option<String>,
    pub key: String,
    #[ormlite(column = "type")]
    pub type_: String,
    pub position_x: i32,
    pub position_y: i32,
    pub options: Option<Json>,
    pub resolve: Option<Uuid>,
    pub reject: Option<Uuid>,
    pub flow: Uuid,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_panels")]
pub struct Panels {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub dashboard: Uuid,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub show_header: bool,
    pub note: Option<String>,
    #[ormlite(column = "type")]
    pub type_: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub options: Option<Json>,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_permissions")]
pub struct Permissions {
    #[ormlite(primary_key)]
    pub id: i32,
    pub collection: String,
    pub action: String,
    pub permissions: Option<Json>,
    pub validation: Option<Json>,
    pub presets: Option<Json>,
    pub fields: Option<String>,
    pub policy: Uuid,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_policies")]
pub struct Policies {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub ip_access: Option<String>,
    pub enforce_tfa: bool,
    pub admin_access: bool,
    pub app_access: bool,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_presets")]
pub struct Presets {
    #[ormlite(primary_key)]
    pub id: i32,
    pub bookmark: Option<String>,
    pub user: Option<Uuid>,
    pub role: Option<Uuid>,
    pub collection: Option<String>,
    pub search: Option<String>,
    pub layout: Option<String>,
    pub layout_query: Option<Json>,
    pub layout_options: Option<Json>,
    pub refresh_interval: Option<i32>,
    pub filter: Option<Json>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_relations")]
pub struct Relations {
    #[ormlite(primary_key)]
    pub id: i32,
    pub many_collection: String,
    pub many_field: String,
    pub one_collection: Option<String>,
    pub one_field: Option<String>,
    pub one_collection_field: Option<String>,
    pub one_allowed_collections: Option<String>,
    pub junction_field: Option<String>,
    pub sort_field: Option<String>,
    pub one_deselect_action: String,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_revisions")]
pub struct Revisions {
    #[ormlite(primary_key)]
    pub id: i32,
    pub activity: i32,
    pub collection: String,
    pub item: String,
    pub data: Option<Json>,
    pub delta: Option<Json>,
    pub parent: Option<i32>,
    pub version: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_roles")]
pub struct Roles {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub parent: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_sessions")]
pub struct Sessions {
    #[ormlite(primary_key)]
    pub token: String,
    pub user: Option<Uuid>,
    pub expires: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub share: Option<Uuid>,
    pub origin: Option<String>,
    pub next_token: Option<String>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_settings")]
pub struct Settings {
    #[ormlite(primary_key)]
    pub id: i32,
    pub project_name: String,
    pub project_url: Option<String>,
    pub project_color: String,
    pub project_logo: Option<Uuid>,
    pub public_foreground: Option<Uuid>,
    pub public_background: Option<Uuid>,
    pub public_note: Option<String>,
    pub auth_login_attempts: Option<i32>,
    pub auth_password_policy: Option<String>,
    pub storage_asset_transform: Option<String>,
    pub storage_asset_presets: Option<Json>,
    pub custom_css: Option<String>,
    pub storage_default_folder: Option<Uuid>,
    pub basemaps: Option<Json>,
    pub mapbox_key: Option<String>,
    pub module_bar: Option<Json>,
    pub project_descriptor: Option<String>,
    pub default_language: String,
    pub custom_aspect_ratios: Option<Json>,
    pub public_favicon: Option<Uuid>,
    pub default_appearance: String,
    pub default_theme_light: Option<String>,
    pub theme_light_overrides: Option<Json>,
    pub default_theme_dark: Option<String>,
    pub theme_dark_overrides: Option<Json>,
    pub report_error_url: Option<String>,
    pub report_bug_url: Option<String>,
    pub report_feature_url: Option<String>,
    pub public_registration: bool,
    pub public_registration_verify_email: bool,
    pub public_registration_role: Option<Uuid>,
    pub public_registration_email_filter: Option<Json>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_shares")]
pub struct Shares {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub role: Option<Uuid>,
    pub password: Option<String>,
    pub user_created: Option<Uuid>,
    pub date_created: Option<String>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
    pub times_used: Option<i32>,
    pub max_uses: Option<i32>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_translations")]
pub struct Translations {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub language: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, ormlite::Model, Clone, Default)]
#[ormlite(table = "directus_users")]
pub struct Users {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    //pub tags: Option<Json>,
    pub avatar: Option<Uuid>,
    pub language: Option<String>,
    pub tfa_secret: Option<String>,
    pub status: String,
    pub role: Option<Uuid>,
    pub token: Option<String>,
    pub last_access: Option<OffsetDateTime>,
    pub last_page: Option<String>,
    pub provider: String,
    pub external_identifier: Option<String>,
    //pub auth_data: Option<Json>,
    pub email_notifications: Option<bool>,
    pub appearance: Option<String>,
    pub theme_dark: Option<String>,
    pub theme_light: Option<String>,
    //pub theme_light_overrides: Option<Json>,
    //pub theme_dark_overrides: Option<Json>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_versions")]
pub struct Versions {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub key: String,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub hash: Option<String>,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub user_created: Option<Uuid>,
    pub user_updated: Option<Uuid>,
    pub delta: Option<Json>,
}

#[derive(Debug, ormlite::Model)]
#[ormlite(table = "directus_webhooks")]
pub struct Webhooks {
    #[ormlite(primary_key)]
    pub id: i32,
    pub name: String,
    pub method: String,
    pub url: String,
    pub status: String,
    pub data: bool,
    pub actions: String,
    pub collections: String,
    pub headers: Option<Json>,
    pub was_active_before_deprecation: bool,
    pub migrated_flow: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
pub struct SpatialRefSys {
    #[ormlite(primary_key)]
    pub srid: i32,
    pub auth_name: Option<String>,
    pub auth_srid: Option<i32>,
    pub srtext: Option<String>,
    pub proj_4_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UploadFilePayload {
    pub storage: String,
    pub filename_disk: String,
    pub filename_download: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub folder: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub created_on: Option<String>,
    pub modified_by: Option<Uuid>,
    pub modified_on: Option<String>,
    pub charset: Option<String>,
    pub filesize: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub embed: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub tags: Option<String>,
    pub metadata: Option<Json>,
    pub focal_point_x: Option<i32>,
    pub focal_point_y: Option<i32>,
    pub tus_id: Option<String>,
    pub tus_data: Option<Json>,
    pub uploaded_on: Option<String>,
}

impl TryFrom<Files> for url::Url {
    type Error = url::ParseError;

    fn try_from(file: Files) -> Result<url::Url, url::ParseError> {
        let file_uuid = file.id;
        let base_url = &crate::core_config().DIRECTUS_URL;

        let url = url::Url::from_str(format!("{}/assets/{}", base_url, file_uuid).as_str())?;

        Ok(url)
    }
}
