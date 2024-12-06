# CreateCollectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection** | **String** | Unique name of the collection. | 
**fields** | [**Vec<json::Value>**](json::Value.md) | The fields contained in this collection. See the fields reference for more information. Each individual field requires field, type, and interface to be provided. | 
**icon** | Option<**String**> | Name of a Google Material Design Icon that's assigned to this collection. | [optional]
**note** | Option<**String**> | A note describing the collection. | [optional]
**display_template** | Option<**String**> | Text representation of how items from this collection are shown across the system. | [optional]
**hidden** | Option<**bool**> | Whether or not the collection is hidden from the navigation in the admin app. | [optional]
**singleton** | Option<**bool**> | Whether or not the collection is treated as a single object. | [optional]
**translation** | Option<**String**> | Key value pairs of how to show this collection's name in different languages in the admin app. | [optional]
**versioning** | Option<**bool**> | Whether or not Content Versioning is enabled for this collection. | [optional]
**archive_field** | Option<**String**> | What field holds the archive value. | [optional]
**archive_app_filter** | Option<**String**> | What value to use for \"archived\" items. | [optional]
**archive_value** | Option<**String**> | What value to use to \"unarchive\" items. | [optional]
**unarchive_value** | Option<**String**> | Whether or not to show the \"archived\" filter. | [optional]
**sort_field** | Option<**String**> | The sort field in the collection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


