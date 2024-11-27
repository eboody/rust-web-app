# \SystemSettingsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_system_env_dump_get**](SystemSettingsApi.md#v1_system_env_dump_get) | **GET** /v1/system/env-dump | 
[**v1_system_export_chats_get**](SystemSettingsApi.md#v1_system_export_chats_get) | **GET** /v1/system/export-chats | 
[**v1_system_get**](SystemSettingsApi.md#v1_system_get) | **GET** /v1/system | 
[**v1_system_remove_documents_delete**](SystemSettingsApi.md#v1_system_remove_documents_delete) | **DELETE** /v1/system/remove-documents | 
[**v1_system_update_env_post**](SystemSettingsApi.md#v1_system_update_env_post) | **POST** /v1/system/update-env | 
[**v1_system_vector_count_get**](SystemSettingsApi.md#v1_system_vector_count_get) | **GET** /v1/system/vector-count | 



## v1_system_env_dump_get

> v1_system_env_dump_get()


Dump all settings to file storage

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_system_export_chats_get

> serde_json::Value v1_system_export_chats_get(r#type)


Export all of the chats from the system in a known format. Output depends on the type sent. Will be send with the correct header for the output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Export format jsonl, json, csv, jsonAlpaca |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_system_get

> serde_json::Value v1_system_get()


Get all current system settings that are defined.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_system_remove_documents_delete

> serde_json::Value v1_system_remove_documents_delete(v1_system_remove_documents_delete_request)


Permanently remove documents from the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_system_remove_documents_delete_request** | [**V1SystemRemoveDocumentsDeleteRequest**](V1SystemRemoveDocumentsDeleteRequest.md) | Array of document names to be removed permanently. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_system_update_env_post

> serde_json::Value v1_system_update_env_post()


Update a system setting or preference.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_system_vector_count_get

> serde_json::Value v1_system_vector_count_get()


Number of all vectors in connected vector database

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

