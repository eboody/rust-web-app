# Query

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fields** | Option<**Vec<String>**> | Control what fields are being returned in the object. | [optional]
**filter** | Option<[**serde_json::Value**](.md)> |  | [optional]
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. | [optional]
**sort** | Option<**Vec<String>**> | How to sort the returned items. | [optional]
**limit** | Option<**f64**> | Set the maximum number of items that will be returned | [optional]
**offset** | Option<**f64**> | How many items to skip when fetching data. | [optional]
**page** | Option<**f64**> | Cursor for use in pagination. Often used in combination with limit. | [optional]
**deep** | Option<[**serde_json::Value**](.md)> | Deep allows you to set any of the other query parameters on a nested relational dataset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


