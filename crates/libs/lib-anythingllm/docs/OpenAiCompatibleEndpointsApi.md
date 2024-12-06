# \OpenAiCompatibleEndpointsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_openai_chat_completions_post**](OpenAiCompatibleEndpointsApi.md#v1_openai_chat_completions_post) | **POST** /v1/openai/chat/completions | 
[**v1_openai_embeddings_post**](OpenAiCompatibleEndpointsApi.md#v1_openai_embeddings_post) | **POST** /v1/openai/embeddings | 
[**v1_openai_models_get**](OpenAiCompatibleEndpointsApi.md#v1_openai_models_get) | **GET** /v1/openai/models | 
[**v1_openai_vector_stores_get**](OpenAiCompatibleEndpointsApi.md#v1_openai_vector_stores_get) | **GET** /v1/openai/vector_stores | 



## v1_openai_chat_completions_post

> v1_openai_chat_completions_post()


Execute a chat with a workspace with OpenAI compatibility. Supports streaming as well. Model must be a workspace slug from /models.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_openai_embeddings_post

> v1_openai_embeddings_post()


Get the embeddings of any arbitrary text string. This will use the embedder provider set in the system. Please ensure the token length of each string fits within the context of your embedder model.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_openai_models_get

> json::Value v1_openai_models_get()


Get all available \"models\" which are workspaces you can use for chatting.

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


## v1_openai_vector_stores_get

> json::Value v1_openai_vector_stores_get()


List all the vector database collections connected to AnythingLLM. These are essentially workspaces but return their unique vector db identifier - this is the same as the workspace slug.

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

