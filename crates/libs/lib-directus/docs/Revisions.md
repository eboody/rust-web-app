# Revisions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for the revision. | [optional]
**activity** | Option<[**models::RevisionsActivity**](Revisions_activity.md)> |  | [optional]
**collection** | Option<[**models::RevisionsCollection**](Revisions_collection.md)> |  | [optional]
**item** | Option<**String**> | Primary key of updated item. | [optional]
**data** | Option<[**serde_json::Value**](.md)> | Copy of item state at time of update. | [optional]
**delta** | Option<[**serde_json::Value**](.md)> | Changes between the previous and the current revision. | [optional]
**parent** | Option<**i32**> | If the current item was updated relationally, this is the id of the parent revision record | [optional]
**version** | Option<[**models::RevisionsVersion**](Revisions_version.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


