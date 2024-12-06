# \VersionsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compare_content_version**](VersionsApi.md#compare_content_version) | **GET** /versions/{id}/compare | Compare a Content Version
[**create_content_version**](VersionsApi.md#create_content_version) | **POST** /versions | Create Multiple Content Versions
[**delete_content_version**](VersionsApi.md#delete_content_version) | **DELETE** /versions/{id} | Delete a Content Version
[**delete_content_versions**](VersionsApi.md#delete_content_versions) | **DELETE** /versions | Delete Multiple Content Versions
[**get_content_version**](VersionsApi.md#get_content_version) | **GET** /versions/{id} | Retrieve a Content Version
[**get_content_versions**](VersionsApi.md#get_content_versions) | **GET** /versions | List Content Versions
[**promote_content_version**](VersionsApi.md#promote_content_version) | **POST** /versions/{id}/promote | Promote a Content Version
[**save_content_version**](VersionsApi.md#save_content_version) | **POST** /versions/{id}/save | Save to a Content Version
[**update_content_version**](VersionsApi.md#update_content_version) | **PATCH** /versions/{id} | Update a Content Version
[**update_content_versions**](VersionsApi.md#update_content_versions) | **PATCH** /versions | Update Multiple Content Versions



## compare_content_version

> models::ServerInfo200Response compare_content_version(id)
Compare a Content Version

Compare an existing Content Version with the main version of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |

### Return type

[**models::ServerInfo200Response**](serverInfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_content_version

> models::CreateContentVersion200Response create_content_version(fields, meta, versions)
Create Multiple Content Versions

Create multiple new Content Versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**versions** | Option<[**Versions**](Versions.md)> |  |  |

### Return type

[**models::CreateContentVersion200Response**](createContentVersion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_content_version

> delete_content_version(id)
Delete a Content Version

Delete an existing Content Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_content_versions

> delete_content_versions()
Delete Multiple Content Versions

Delete multiple existing Content Versions.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_version

> models::CreateContentVersion200Response get_content_version(id, fields, meta)
Retrieve a Content Version

Retrieve a single Content Version by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateContentVersion200Response**](createContentVersion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_versions

> models::GetContentVersions200Response get_content_versions(fields, limit, offset, meta, sort, filter, search)
List Content Versions

Get all Content Versions.

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

### Return type

[**models::GetContentVersions200Response**](getContentVersions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_content_version

> json::Value promote_content_version(id, promote_content_version_request)
Promote a Content Version

Pass the current hash of the main version of the item (obtained from the `compare` endpoint) along with an optional array of field names of which the values are to be promoted (by default, all fields are selected).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**promote_content_version_request** | Option<[**PromoteContentVersionRequest**](PromoteContentVersionRequest.md)> |  |  |

### Return type

[**json::Value**](json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_content_version

> json::Value save_content_version(id, body)
Save to a Content Version

Save item changes to an existing Content Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**body** | Option<**json::Value**> |  |  |

### Return type

[**json::Value**](json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_content_version

> models::CreateContentVersion200Response update_content_version(id, fields, meta, versions)
Update a Content Version

Update an existing Content Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**versions** | Option<[**Versions**](Versions.md)> |  |  |

### Return type

[**models::CreateContentVersion200Response**](createContentVersion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_content_versions

> models::GetContentVersions200Response update_content_versions(fields, limit, meta, offset, sort, filter, search, update_content_versions_request)
Update Multiple Content Versions

Update multiple Content Versions at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**update_content_versions_request** | Option<[**UpdateContentVersionsRequest**](UpdateContentVersionsRequest.md)> |  |  |

### Return type

[**models::GetContentVersions200Response**](getContentVersions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

