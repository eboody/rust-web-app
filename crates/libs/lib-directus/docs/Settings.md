# Settings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for the setting. | [optional]
**project_name** | Option<**String**> | The name of the project. | [optional]
**project_url** | Option<**String**> | The url of the project. | [optional]
**project_color** | Option<**String**> | The brand color of the project. | [optional]
**project_logo** | Option<**String**> | The logo of the project. | [optional]
**public_foreground** | Option<**String**> | The foreground of the project. | [optional]
**public_background** | Option<[**models::SettingsPublicBackground**](Settings_public_background.md)> |  | [optional]
**public_note** | Option<**String**> | Note rendered on the public pages of the app. | [optional]
**auth_login_attempts** | Option<**i32**> | Allowed authentication login attempts before the user's status is set to blocked. | [optional]
**auth_password_policy** | Option<**String**> | Authentication password policy. | [optional]
**storage_asset_transform** | Option<**String**> | What transformations are allowed in the assets endpoint. | [optional]
**storage_asset_presets** | Option<[**Vec<models::SettingsStorageAssetPresetsInner>**](Settings_storage_asset_presets_inner.md)> | Array of allowed | [optional]
**custom_css** | Option<**String**> |  | [optional]
**storage_default_folder** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Default folder to place files | [optional]
**basemaps** | Option<[**serde_json::Value**](.md)> |  | [optional]
**mapbox_key** | Option<**String**> |  | [optional]
**module_bar** | Option<[**serde_json::Value**](.md)> |  | [optional]
**project_descriptor** | Option<**String**> |  | [optional]
**default_language** | Option<**String**> |  | [optional]
**custom_aspect_ratios** | Option<[**serde_json::Value**](.md)> |  | [optional]
**public_favicon** | Option<[**models::SettingsPublicFavicon**](Settings_public_favicon.md)> |  | [optional]
**default_appearance** | Option<**String**> |  | [optional]
**default_theme_light** | Option<**String**> |  | [optional]
**theme_light_overrides** | Option<[**serde_json::Value**](.md)> |  | [optional]
**default_theme_dark** | Option<**String**> |  | [optional]
**theme_dark_overrides** | Option<[**serde_json::Value**](.md)> |  | [optional]
**report_error_url** | Option<**String**> |  | [optional]
**report_bug_url** | Option<**String**> |  | [optional]
**report_feature_url** | Option<**String**> |  | [optional]
**public_registration** | Option<**bool**> | $t:fields.directus_settings.public_registration_note | [optional]
**public_registration_verify_email** | Option<**bool**> | $t:fields.directus_settings.public_registration_verify_email_note | [optional]
**public_registration_role** | Option<[**models::SettingsPublicRegistrationRole**](Settings_public_registration_role.md)> |  | [optional]
**public_registration_email_filter** | Option<[**serde_json::Value**](.md)> | $t:fields.directus_settings.public_registration_email_filter_note | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


