# \FlowsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_flow**](FlowsApi.md#create_flow) | **POST** /flows | Create a Flow
[**delete_flow**](FlowsApi.md#delete_flow) | **DELETE** /flows/{id} | Delete a Flow
[**delete_flows**](FlowsApi.md#delete_flows) | **DELETE** /flows | Delete Multiple Flows
[**get_flow**](FlowsApi.md#get_flow) | **GET** /flows/{id} | Retrieve a Flow
[**get_flows**](FlowsApi.md#get_flows) | **GET** /flows | List Flows
[**update_flow**](FlowsApi.md#update_flow) | **PATCH** /flows/{id} | Update a Flow
[**update_flows**](FlowsApi.md#update_flows) | **PATCH** /flows | Update Multiple Flows



## create_flow

> models::CreateFlow200Response create_flow(fields, meta, flows)
Create a Flow

Create a new flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**flows** | Option<[**Flows**](Flows.md)> |  |  |

### Return type

[**models::CreateFlow200Response**](createFlow_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flow

> delete_flow(id)
Delete a Flow

Delete an existing flow

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


## delete_flows

> delete_flows()
Delete Multiple Flows

Delete multiple existing flows.

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


## get_flow

> models::CreateFlow200Response get_flow(id)
Retrieve a Flow

Retrieve a single flow by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |

### Return type

[**models::CreateFlow200Response**](createFlow_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows

> models::GetFlows200Response get_flows()
List Flows

Get all flows.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetFlows200Response**](getFlows_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_flow

> models::CreateFlow200Response update_flow(id, fields, meta, flows)
Update a Flow

Update an existing flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**flows** | Option<[**Flows**](Flows.md)> |  |  |

### Return type

[**models::CreateFlow200Response**](createFlow_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_flows

> models::GetFlows200Response update_flows(fields, limit, meta, offset, sort, filter, search, update_flows_request)
Update Multiple Flows

Update multiple flows at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**serde_json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**update_flows_request** | Option<[**UpdateFlowsRequest**](UpdateFlowsRequest.md)> |  |  |

### Return type

[**models::GetFlows200Response**](getFlows_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

