# \WebhooksApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksApi.md#create_webhook) | **POST** /webhooks | Create a Webhook
[**delete_webhook**](WebhooksApi.md#delete_webhook) | **DELETE** /webhooks/{id} | Delete a Webhook
[**delete_webhooks**](WebhooksApi.md#delete_webhooks) | **DELETE** /webhooks | Delete Multiple Webhooks
[**get_webhook**](WebhooksApi.md#get_webhook) | **GET** /webhooks/{id} | Retrieve a Webhook
[**get_webhooks**](WebhooksApi.md#get_webhooks) | **GET** /webhooks | List Webhooks
[**update_webhook**](WebhooksApi.md#update_webhook) | **PATCH** /webhooks/{id} | Update a Webhook
[**update_webhooks**](WebhooksApi.md#update_webhooks) | **PATCH** /webhooks | Update Multiple Webhooks



## create_webhook

> models::CreateRole200Response create_webhook(fields, meta, create_webhook_request)
Create a Webhook

Create a new webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_webhook_request** | Option<[**CreateWebhookRequest**](CreateWebhookRequest.md)> |  |  |

### Return type

[**models::CreateRole200Response**](createRole_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> delete_webhook(id)
Delete a Webhook

Delete an existing webhook

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


## delete_webhooks

> delete_webhooks()
Delete Multiple Webhooks

Delete multiple existing webhooks.

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


## get_webhook

> models::GetWebhooks200Response get_webhook(id)
Retrieve a Webhook

Retrieve a single webhook by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |

### Return type

[**models::GetWebhooks200Response**](getWebhooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> models::GetWebhooks200Response get_webhooks()
List Webhooks

Get all webhooks.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWebhooks200Response**](getWebhooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::CreateRole200Response update_webhook(id, fields, meta, create_webhook_request)
Update a Webhook

Update an existing webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_webhook_request** | Option<[**CreateWebhookRequest**](CreateWebhookRequest.md)> |  |  |

### Return type

[**models::CreateRole200Response**](createRole_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhooks

> models::UpdateWebhooks200Response update_webhooks(fields, limit, meta, offset, sort, filter, search, update_webhooks_request)
Update Multiple Webhooks

Update multiple webhooks at the same time.

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
**update_webhooks_request** | Option<[**UpdateWebhooksRequest**](UpdateWebhooksRequest.md)> |  |  |

### Return type

[**models::UpdateWebhooks200Response**](updateWebhooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

