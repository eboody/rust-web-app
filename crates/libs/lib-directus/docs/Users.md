# Users

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the user. | [optional]
**first_name** | Option<**String**> | First name of the user. | [optional]
**last_name** | Option<**String**> | Last name of the user. | [optional]
**email** | Option<**String**> | Unique email address for the user. | [optional]
**password** | Option<**String**> | Password of the user. | [optional]
**location** | Option<**String**> | The user's location. | [optional]
**title** | Option<**String**> | The user's title. | [optional]
**description** | Option<**String**> | The user's description. | [optional]
**tags** | Option<**Vec<String>**> | The user's tags. | [optional]
**avatar** | Option<[**models::UsersAvatar**](Users_avatar.md)> |  | [optional]
**language** | Option<**String**> | The user's language used in Directus. | [optional]
**tfa_secret** | Option<**String**> | The 2FA secret string that's used to generate one time passwords. | [optional]
**status** | Option<**String**> | Status of the user. | [optional]
**role** | Option<[**models::UsersRole**](Users_role.md)> |  | [optional]
**token** | Option<**String**> | Static token for the user. | [optional]
**last_access** | Option<**String**> | When this user used the API last. | [optional]
**last_page** | Option<**String**> | Last page that the user was on. | [optional]
**provider** | Option<**String**> |  | [optional]
**external_identifier** | Option<**String**> |  | [optional]
**auth_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**email_notifications** | Option<**bool**> |  | [optional]
**appearance** | Option<**String**> |  | [optional]
**theme_dark** | Option<**String**> |  | [optional]
**theme_light** | Option<**String**> |  | [optional]
**theme_light_overrides** | Option<[**serde_json::Value**](.md)> |  | [optional]
**theme_dark_overrides** | Option<[**serde_json::Value**](.md)> |  | [optional]
**policies** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


