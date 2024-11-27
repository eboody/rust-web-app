# \WorkspaceThreadsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_workspace_slug_thread_new_post**](WorkspaceThreadsApi.md#v1_workspace_slug_thread_new_post) | **POST** /v1/workspace/{slug}/thread/new | 
[**v1_workspace_slug_thread_thread_slug_chat_post**](WorkspaceThreadsApi.md#v1_workspace_slug_thread_thread_slug_chat_post) | **POST** /v1/workspace/{slug}/thread/{threadSlug}/chat | 
[**v1_workspace_slug_thread_thread_slug_chats_get**](WorkspaceThreadsApi.md#v1_workspace_slug_thread_thread_slug_chats_get) | **GET** /v1/workspace/{slug}/thread/{threadSlug}/chats | 
[**v1_workspace_slug_thread_thread_slug_delete**](WorkspaceThreadsApi.md#v1_workspace_slug_thread_thread_slug_delete) | **DELETE** /v1/workspace/{slug}/thread/{threadSlug} | 
[**v1_workspace_slug_thread_thread_slug_stream_chat_post**](WorkspaceThreadsApi.md#v1_workspace_slug_thread_thread_slug_stream_chat_post) | **POST** /v1/workspace/{slug}/thread/{threadSlug}/stream-chat | 
[**v1_workspace_slug_thread_thread_slug_update_post**](WorkspaceThreadsApi.md#v1_workspace_slug_thread_thread_slug_update_post) | **POST** /v1/workspace/{slug}/thread/{threadSlug}/update | 



## v1_workspace_slug_thread_new_post

> serde_json::Value v1_workspace_slug_thread_new_post(slug)


Create a new workspace thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_thread_thread_slug_chat_post

> serde_json::Value v1_workspace_slug_thread_thread_slug_chat_post(slug, thread_slug)


Chat with a workspace thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace | [required] |
**thread_slug** | **String** | Unique slug of thread | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_thread_thread_slug_chats_get

> serde_json::Value v1_workspace_slug_thread_thread_slug_chats_get(slug, thread_slug)


Get chats for a workspace thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace | [required] |
**thread_slug** | **String** | Unique slug of thread | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_thread_thread_slug_delete

> v1_workspace_slug_thread_thread_slug_delete(slug, thread_slug)


Delete a workspace thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace | [required] |
**thread_slug** | **String** | Unique slug of thread | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_thread_thread_slug_stream_chat_post

> Vec<String> v1_workspace_slug_thread_thread_slug_stream_chat_post(slug, thread_slug)


Stream chat with a workspace thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace | [required] |
**thread_slug** | **String** | Unique slug of thread | [required] |

### Return type

**Vec<String>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_workspace_slug_thread_thread_slug_update_post

> serde_json::Value v1_workspace_slug_thread_thread_slug_update_post(slug, thread_slug)


Update thread name by its unique slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique slug of workspace | [required] |
**thread_slug** | **String** | Unique slug of thread | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

