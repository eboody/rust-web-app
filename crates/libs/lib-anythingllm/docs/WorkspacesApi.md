# \WorkspacesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_workspace_new_post**](WorkspacesApi.md#v1_workspace_new_post) | **POST** /v1/workspace/new | 
[**v1_workspace_slug_chat_post**](WorkspacesApi.md#v1_workspace_slug_chat_post) | **POST** /v1/workspace/{slug}/chat | 
[**v1_workspace_slug_chats_get**](WorkspacesApi.md#v1_workspace_slug_chats_get) | **GET** /v1/workspace/{slug}/chats | 
[**v1_workspace_slug_delete**](WorkspacesApi.md#v1_workspace_slug_delete) | **DELETE** /v1/workspace/{slug} | 
[**v1_workspace_slug_get**](WorkspacesApi.md#v1_workspace_slug_get) | **GET** /v1/workspace/{slug} | 
[**v1_workspace_slug_stream_chat_post**](WorkspacesApi.md#v1_workspace_slug_stream_chat_post) | **POST** /v1/workspace/{slug}/stream-chat | 
[**v1_workspace_slug_update_embeddings_post**](WorkspacesApi.md#v1_workspace_slug_update_embeddings_post) | **POST** /v1/workspace/{slug}/update-embeddings | 
[**v1_workspace_slug_update_pin_post**](WorkspacesApi.md#v1_workspace_slug_update_pin_post) | **POST** /v1/workspace/{slug}/update-pin | 
[**v1_workspace_slug_update_post**](WorkspacesApi.md#v1_workspace_slug_update_post) | **POST** /v1/workspace/{slug}/update | 
[**v1_workspaces_get**](WorkspacesApi.md#v1_workspaces_get) | **GET** /v1/workspaces | 



## v1_workspace_new_post

> serde_json::Value v1_workspace_new_post()


Create a new workspace

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


## v1_workspace_slug_chat_post

> serde_json::Value v1_workspace_slug_chat_post(slug)


Execute a chat with a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_chats_get

> serde_json::Value v1_workspace_slug_chats_get(slug, api_session_id, limit, order_by)


Get a workspaces chats regardless of user by its unique slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace to find | [required] |
**api_session_id** | Option<**String**> | Optional apiSessionId to filter by |  |
**limit** | Option<**i32**> | Optional number of chat messages to return (default: 100) |  |
**order_by** | Option<**String**> | Optional order of chat messages (asc or desc) |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_delete

> v1_workspace_slug_delete(slug)


Deletes a workspace by its slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace to delete | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_get

> serde_json::Value v1_workspace_slug_get(slug)


Get a workspace by its unique slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace to find | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_stream_chat_post

> Vec<String> v1_workspace_slug_stream_chat_post(slug)


Execute a streamable chat with a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_update_embeddings_post

> serde_json::Value v1_workspace_slug_update_embeddings_post(slug)


Add or remove documents from a workspace by its unique slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace to find | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_update_pin_post

> serde_json::Value v1_workspace_slug_update_pin_post(slug)


Add or remove pin from a document in a workspace by its unique slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace to find | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_update_post

> serde_json::Value v1_workspace_slug_update_post(slug)


Update workspace settings by its unique slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace to find | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspaces_get

> serde_json::Value v1_workspaces_get()


List all current workspaces

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

