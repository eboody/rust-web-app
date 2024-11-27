# \ExtensionsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_extensions**](ExtensionsApi.md#list_extensions) | **GET** /extensions | List Extensions
[**update_extension_bundle**](ExtensionsApi.md#update_extension_bundle) | **PATCH** /extensions/{bundle}/{name} | Update an Extension
[**update_extensions**](ExtensionsApi.md#update_extensions) | **PATCH** /extensions/{name} | Update an Extension



## list_extensions

> models::ListExtensions200Response list_extensions()
List Extensions

List the installed extensions and their configuration in the project.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListExtensions200Response**](listExtensions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_extension_bundle

> models::UpdateExtensions200Response update_extension_bundle(bundle, name, update_extensions_request)
Update an Extension

Update an existing extension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle** | **String** |  | [required] |
**name** | **String** |  | [required] |
**update_extensions_request** | Option<[**UpdateExtensionsRequest**](UpdateExtensionsRequest.md)> |  |  |

### Return type

[**models::UpdateExtensions200Response**](updateExtensions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_extensions

> models::UpdateExtensions200Response update_extensions(name, update_extensions_request)
Update an Extension

Update an existing extension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_extensions_request** | Option<[**UpdateExtensionsRequest**](UpdateExtensionsRequest.md)> |  |  |

### Return type

[**models::UpdateExtensions200Response**](updateExtensions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

