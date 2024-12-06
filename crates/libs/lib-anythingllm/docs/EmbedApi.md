# \EmbedApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_embed_embed_uuid_chats_get**](EmbedApi.md#v1_embed_embed_uuid_chats_get) | **GET** /v1/embed/{embedUuid}/chats | 
[**v1_embed_embed_uuid_chats_session_uuid_get**](EmbedApi.md#v1_embed_embed_uuid_chats_session_uuid_get) | **GET** /v1/embed/{embedUuid}/chats/{sessionUuid} | 
[**v1_embed_get**](EmbedApi.md#v1_embed_get) | **GET** /v1/embed | 



## v1_embed_embed_uuid_chats_get

> json::Value v1_embed_embed_uuid_chats_get(embed_uuid)


Get all chats for a specific embed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**embed_uuid** | **String** | UUID of the embed | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_embed_embed_uuid_chats_session_uuid_get

> json::Value v1_embed_embed_uuid_chats_session_uuid_get(embed_uuid, session_uuid)


Get chats for a specific embed and session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**embed_uuid** | **String** | UUID of the embed | [required] |
**session_uuid** | **String** | UUID of the session | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_embed_get

> json::Value v1_embed_get()


List all active embeds

### Parameters

This endpoint does not need any parameter.

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

