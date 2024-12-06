# Permissions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for the permission. | [optional]
**collection** | Option<**String**> | What collection this permission applies to. | [optional]
**action** | Option<**String**> | What action this permission applies to. | [optional]
**permissions** | Option<[**json::Value**](.md)> | JSON structure containing the permissions checks for this permission. | [optional]
**validation** | Option<[**json::Value**](.md)> | JSON structure containing the validation checks for this permission. | [optional]
**presets** | Option<[**json::Value**](.md)> | JSON structure containing the preset value for created/updated items. | [optional]
**fields** | Option<**Vec<String>**> | CSV of fields that the user is allowed to interact with. | [optional]
**policy** | Option<[**json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


