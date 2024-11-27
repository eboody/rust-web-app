# UpdatePresetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection** | **String** | What collection this collection preset is used for. | 
**title** | Option<**String**> | Name for the bookmark. If this is set, the collection preset will be considered to be a bookmark. | [optional]
**role** | Option<**i32**> | The unique identifier of a role in the platform. If user is null, this will be used to apply the collection preset or bookmark for all users in the role. | [optional]
**search_query** | Option<**String**> | What the user searched for in search/filter in the header bar. | [optional]
**filters** | Option<[**Vec<models::UpdatePresetRequestFiltersInner>**](updatePreset_request_filters_inner.md)> |  | [optional]
**view_type** | Option<**String**> | Name of the view type that is used. Defaults to tabular. | [optional]
**view_query** | Option<**String**> | View query that's saved per view type. Controls what data is fetched on load. These follow the same format as the JS SDK parameters. | [optional]
**view_options** | Option<**String**> | Options of the views. The properties in here are controlled by the layout. | [optional]
**translation** | Option<[**serde_json::Value**](.md)> | Key value pair of language-translation. Can be used to translate the bookmark title in multiple languages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


