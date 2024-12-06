# \RevisionsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_revision**](RevisionsApi.md#get_revision) | **GET** /revisions/{id} | Retrieve a Revision
[**get_revisions**](RevisionsApi.md#get_revisions) | **GET** /revisions | List Revisions



## get_revision

> models::GetRevision200Response get_revision(id, fields, meta)
Retrieve a Revision

Retrieve a single revision by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::GetRevision200Response**](getRevision_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revisions

> models::GetRevisions200Response get_revisions(fields, limit, offset, meta, sort, filter, search, page)
List Revisions

List the revisions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**page** | Option<**i32**> | Cursor for use in pagination. Often used in combination with limit. |  |

### Return type

[**models::GetRevisions200Response**](getRevisions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

