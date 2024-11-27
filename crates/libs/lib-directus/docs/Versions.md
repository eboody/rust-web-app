# Versions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Primary key of the Content Version. | [optional]
**key** | Option<**String**> | Key of the Content Version, used as the value for the \"version\" query parameter. | [optional]
**name** | Option<**String**> | Descriptive name of the Content Version. | [optional]
**collection** | Option<[**models::VersionsCollection**](Versions_collection.md)> |  | [optional]
**item** | Option<**String**> | The item the Content Version is created on. | [optional]
**hash** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | When the Content Version was created. | [optional]
**date_updated** | Option<**String**> | When the Content Version was last updated. | [optional]
**user_created** | Option<[**models::VersionsUserCreated**](Versions_user_created.md)> |  | [optional]
**user_updated** | Option<[**models::VersionsUserUpdated**](Versions_user_updated.md)> |  | [optional]
**delta** | Option<[**serde_json::Value**](.md)> | The current changes compared to the main version of the item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


