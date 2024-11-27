# CreatePresetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection** | **String** | What collection this collection preset is used for. | 
**title** | Option<**String**> | Name for the bookmark. If this is set, the collection preset will be considered to be a bookmark. | [optional]
**role** | Option<**String**> | The unique identifier of a role in the platform. If user is null, this will be used to apply the collection preset or bookmark for all users in the role. | [optional]
**search** | Option<**String**> | What the user searched for in search/filter in the header bar. | [optional]
**filters** | Option<[**Vec<models::CreatePresetRequestFiltersInner>**](createPreset_request_filters_inner.md)> |  | [optional]
**layout** | Option<**String**> | Name of the view type that is used. | [optional]
**layout_query** | Option<**String**> | Layout query that's saved per layout type. Controls what data is fetched on load. These follow the same format as the JS SDK parameters. | [optional]
**layout_options** | Option<**String**> | Options of the views. The properties in here are controlled by the layout. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


