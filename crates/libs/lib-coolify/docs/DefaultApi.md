# \DefaultApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_api**](DefaultApi.md#disable_api) | **GET** /disable | Disable API
[**enable_api**](DefaultApi.md#enable_api) | **GET** /enable | Enable API
[**healthcheck**](DefaultApi.md#healthcheck) | **GET** /health | Healthcheck
[**version**](DefaultApi.md#version) | **GET** /version | Version



## disable_api

> models::DisableApi200Response disable_api()
Disable API

Disable API (only with root permissions).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DisableApi200Response**](disable_api_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_api

> models::EnableApi200Response enable_api()
Enable API

Enable API (only with root permissions).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::EnableApi200Response**](enable_api_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## healthcheck

> String healthcheck()
Healthcheck

Healthcheck endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version

> String version()
Version

Get Coolify version.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

