# \UtilitiesApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_cache**](UtilitiesApi.md#clear_cache) | **POST** /utils/cache/clear | Clear Cache
[**export**](UtilitiesApi.md#export) | **POST** /utils/export/{collection} | Export Items
[**hash_generate**](UtilitiesApi.md#hash_generate) | **POST** /utils/hash/generate | Hash a string
[**hash_verify**](UtilitiesApi.md#hash_verify) | **POST** /utils/hash/verify | Hash a string
[**import**](UtilitiesApi.md#import) | **POST** /utils/import/{collection} | Import Items
[**random**](UtilitiesApi.md#random) | **GET** /utils/random/string | Get a Random String
[**sort**](UtilitiesApi.md#sort) | **POST** /utils/sort/{collection} | Sort Items



## clear_cache

> clear_cache()
Clear Cache

Resets both the data and schema cache of Directus.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export

> export(collection, export_request)
Export Items

Export a larger data set to a file in the File Library

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Collection identifier | [required] |
**export_request** | Option<[**ExportRequest**](ExportRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hash_generate

> models::HashGenerate200Response hash_generate(hash_generate_request)
Hash a string

Generate a hash for a given string.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash_generate_request** | Option<[**HashGenerateRequest**](HashGenerateRequest.md)> |  |  |

### Return type

[**models::HashGenerate200Response**](hash_generate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hash_verify

> models::HashVerify200Response hash_verify(hash_verify_request)
Hash a string

Generate a hash for a given string.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash_verify_request** | Option<[**HashVerifyRequest**](HashVerifyRequest.md)> |  |  |

### Return type

[**models::HashVerify200Response**](hash_verify_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import

> import(collection, file)
Import Items

Import multiple records from a JSON or CSV file into a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Collection identifier | [required] |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## random

> models::Random200Response random(length)
Get a Random String

Returns a random string of given length.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**length** | Option<**i32**> | Length of the random string. |  |

### Return type

[**models::Random200Response**](random_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sort

> sort(collection, sort_request)
Sort Items

Re-sort items in collection based on start and to value of item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Collection identifier | [required] |
**sort_request** | Option<[**SortRequest**](SortRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

