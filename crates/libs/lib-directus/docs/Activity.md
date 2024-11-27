# Activity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for the object. | [optional]
**action** | Option<**String**> | Action that was performed. | [optional]
**user** | Option<[**models::ActivityUser**](Activity_user.md)> |  | [optional]
**timestamp** | Option<**String**> | When the action happened. | [optional]
**ip** | Option<[**models::ActivityIp**](Activity_ip.md)> |  | [optional]
**user_agent** | Option<**String**> | User agent string of the browser the user used when the action took place. | [optional]
**collection** | Option<[**models::ActivityCollection**](Activity_collection.md)> |  | [optional]
**item** | Option<**String**> | Unique identifier for the item the action applied to. This is always a string, even for integer primary keys. | [optional]
**comment** | Option<**String**> | User comment. This will store the comments that show up in the right sidebar of the item edit page in the admin app. | [optional]
**origin** | Option<**String**> | Origin of the request when the action took place. | [optional]
**revisions** | Option<[**Vec<models::ActivityRevisionsInner>**](Activity_revisions_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


