# Paging

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cursors** | Option<[**models::PagingCursors**](Paging_cursors.md)> |  | [optional]
**next** | Option<[**serde_json::Value**](.md)> | The Graph API endpoint that will return the next page of data. If not included, this is the last page of data.   Due to how pagination works with visibility and privacy, it is possible that a page may be empty but contain  a next paging link. Stop paging when the next link no longer appears.  | [optional]
**previous** | Option<[**serde_json::Value**](.md)> | The Graph API endpoint that will return the previous page of data. If not included, this is the first page of data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


