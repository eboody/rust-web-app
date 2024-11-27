# \InfoApi

All URIs are relative to *https://spdf.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_endpoint_loads**](InfoApi.md#get_all_endpoint_loads) | **GET** /api/v1/info/load/all | GET requests count for all endpoints
[**get_all_post_requests**](InfoApi.md#get_all_post_requests) | **GET** /api/v1/info/requests/all | POST requests count for all endpoints
[**get_all_unique_endpoint_loads**](InfoApi.md#get_all_unique_endpoint_loads) | **GET** /api/v1/info/load/all/unique | Unique users count for GET requests for all endpoints
[**get_all_unique_post_requests**](InfoApi.md#get_all_unique_post_requests) | **GET** /api/v1/info/requests/all/unique | Unique users count for POST requests for all endpoints
[**get_page_loads**](InfoApi.md#get_page_loads) | **GET** /api/v1/info/load | GET request count
[**get_status**](InfoApi.md#get_status) | **GET** /api/v1/info/status | Application status and version
[**get_total_requests**](InfoApi.md#get_total_requests) | **GET** /api/v1/info/requests | POST request count
[**get_unique_page_loads**](InfoApi.md#get_unique_page_loads) | **GET** /api/v1/info/load/unique | Unique users count for GET requests
[**get_unique_total_requests**](InfoApi.md#get_unique_total_requests) | **GET** /api/v1/info/requests/unique | Unique users count for POST requests
[**get_uptime**](InfoApi.md#get_uptime) | **GET** /api/v1/info/uptime | 



## get_all_endpoint_loads

> serde_json::Value get_all_endpoint_loads()
GET requests count for all endpoints

This endpoint returns the count of GET requests for each endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_post_requests

> serde_json::Value get_all_post_requests()
POST requests count for all endpoints

This endpoint returns the count of POST requests for each endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_unique_endpoint_loads

> serde_json::Value get_all_unique_endpoint_loads()
Unique users count for GET requests for all endpoints

This endpoint returns the count of unique users for GET requests for each endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_unique_post_requests

> serde_json::Value get_all_unique_post_requests()
Unique users count for POST requests for all endpoints

This endpoint returns the count of unique users for POST requests for each endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_page_loads

> serde_json::Value get_page_loads(endpoint)
GET request count

This endpoint returns the total count of GET requests for a specific endpoint or all endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | Option<**String**> | endpoint |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> serde_json::Value get_status()
Application status and version

This endpoint returns the status of the application and its version number.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_total_requests

> serde_json::Value get_total_requests(endpoint)
POST request count

This endpoint returns the total count of POST requests for a specific endpoint or all endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | Option<**String**> | endpoint |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unique_page_loads

> serde_json::Value get_unique_page_loads(endpoint)
Unique users count for GET requests

This endpoint returns the count of unique users for GET requests for a specific endpoint or all endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | Option<**String**> | endpoint |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unique_total_requests

> serde_json::Value get_unique_total_requests(endpoint)
Unique users count for POST requests

This endpoint returns the count of unique users for POST requests for a specific endpoint or all endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | Option<**String**> | endpoint |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_uptime

> serde_json::Value get_uptime()


### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

