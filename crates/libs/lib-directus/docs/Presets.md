# Presets

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for this single collection preset. | [optional]
**bookmark** | Option<**String**> | Name for the bookmark. If this is set, the preset will be considered a bookmark. | [optional]
**user** | Option<[**models::PresetsUser**](Presets_user.md)> |  | [optional]
**role** | Option<[**models::PresetsRole**](Presets_role.md)> |  | [optional]
**collection** | Option<[**models::PresetsCollection**](Presets_collection.md)> |  | [optional]
**search** | Option<**String**> | Search query. | [optional]
**layout** | Option<**String**> | Key of the layout that is used. | [optional]
**layout_query** | Option<[**json::Value**](.md)> | Layout query that's saved per layout type. Controls what data is fetched on load. These follow the same format as the JS SDK parameters. | [optional]
**layout_options** | Option<[**json::Value**](.md)> | Options of the views. The properties in here are controlled by the layout. | [optional]
**refresh_interval** | Option<**i32**> |  | [optional]
**filter** | Option<[**json::Value**](.md)> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**color** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


