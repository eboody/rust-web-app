# \SchemaApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**schema_apply**](SchemaApi.md#schema_apply) | **POST** /schema/apply | Apply Schema Difference
[**schema_diff**](SchemaApi.md#schema_diff) | **POST** /schema/diff | Retrieve Schema Difference
[**schema_snapshot**](SchemaApi.md#schema_snapshot) | **GET** /schema/snapshot | Retrieve Schema Snapshot



## schema_apply

> schema_apply(schema_apply_request)
Apply Schema Difference

Update the instance's schema by passing the diff previously retrieved via `/schema/diff` endpoint in the JSON request body or a JSON/YAML file. This endpoint is only available to admin users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_apply_request** | [**SchemaApplyRequest**](SchemaApplyRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schema_diff

> models::SchemaApplyRequest schema_diff(schema_snapshot200_response, force)
Retrieve Schema Difference

Compare the current instance's schema against the schema snapshot in JSON request body or a JSON/YAML file and retrieve the difference. This endpoint is only available to admin users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_snapshot200_response** | [**SchemaSnapshot200Response**](SchemaSnapshot200Response.md) |  | [required] |
**force** | Option<**bool**> | Bypass version and database vendor restrictions. |  |

### Return type

[**models::SchemaApplyRequest**](schemaApply_request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schema_snapshot

> models::SchemaSnapshot200Response schema_snapshot(export)
Retrieve Schema Snapshot

Retrieve the current schema. This endpoint is only available to admin users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export** | Option<**String**> | Saves the API response to a file. Accepts one of \"csv\", \"json\", \"xml\", \"yaml\". |  |

### Return type

[**models::SchemaSnapshot200Response**](schemaSnapshot_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

