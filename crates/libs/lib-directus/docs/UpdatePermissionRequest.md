# UpdatePermissionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection** | Option<[**serde_json::Value**](.md)> | What collection this permission applies to. | [optional]
**comment** | Option<**String**> | If the user can post comments. `full`. | [optional]
**create** | Option<**String**> | If the user can create items. | [optional]
**delete** | Option<**String**> | If the user can update items. | [optional]
**explain** | Option<**String**> | If the user is required to leave a comment explaining what was changed. | [optional]
**read** | Option<**String**> | If the user can read items. | [optional]
**read_field_blacklist** | Option<[**serde_json::Value**](.md)> | Explicitly denies read access for specific fields. | [optional]
**role** | Option<[**serde_json::Value**](.md)> | Unique identifier of the role this permission applies to. | [optional]
**status** | Option<[**serde_json::Value**](.md)> | What status this permission applies to. | [optional]
**status_blacklist** | Option<[**serde_json::Value**](.md)> | Explicitly denies specific statuses to be used. | [optional]
**update** | Option<**String**> | If the user can update items. | [optional]
**write_field_blacklist** | Option<[**serde_json::Value**](.md)> | Explicitly denies write access for specific fields. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


