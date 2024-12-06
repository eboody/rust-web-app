# \OperationsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_operation**](OperationsApi.md#create_operation) | **POST** /operations | Create an Operation
[**delete_operation**](OperationsApi.md#delete_operation) | **DELETE** /operations/{id} | Delete an Operation
[**delete_operations**](OperationsApi.md#delete_operations) | **DELETE** /operations | Delete Multiple Operations
[**get_operation**](OperationsApi.md#get_operation) | **GET** /operations/{id} | Retrieve an Operation
[**get_operations**](OperationsApi.md#get_operations) | **GET** /operations | List Operations
[**update_operation**](OperationsApi.md#update_operation) | **PATCH** /operations/{id} | Update an Operation
[**update_operations**](OperationsApi.md#update_operations) | **PATCH** /operations | Update Multiple Operations



## create_operation

> models::CreateOperation200Response create_operation(fields, meta, operations)
Create an Operation

Create a new operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**operations** | Option<[**Operations**](Operations.md)> |  |  |

### Return type

[**models::CreateOperation200Response**](createOperation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_operation

> delete_operation(id)
Delete an Operation

Delete an existing operation

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


## delete_operations

> delete_operations()
Delete Multiple Operations

Delete multiple existing operations.

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


## get_operation

> models::CreateOperation200Response get_operation(id)
Retrieve an Operation

Retrieve a single operation by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |

### Return type

[**models::CreateOperation200Response**](createOperation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_operations

> models::GetOperations200Response get_operations()
List Operations

Get all operations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetOperations200Response**](getOperations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_operation

> models::CreateOperation200Response update_operation(id, fields, meta, operations)
Update an Operation

Update an existing operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**operations** | Option<[**Operations**](Operations.md)> |  |  |

### Return type

[**models::CreateOperation200Response**](createOperation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_operations

> models::GetOperations200Response update_operations(fields, limit, meta, offset, sort, filter, search, update_operations_request)
Update Multiple Operations

Update multiple operations at the same time.

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
**update_operations_request** | Option<[**UpdateOperationsRequest**](UpdateOperationsRequest.md)> |  |  |

### Return type

[**models::GetOperations200Response**](getOperations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

