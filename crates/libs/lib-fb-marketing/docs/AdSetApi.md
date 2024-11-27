# \AdSetApi

All URIs are relative to *https://graph.facebook.com/v12.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ad_set**](AdSetApi.md#create_ad_set) | **POST** /act_{ad_account_id}/adsets | 
[**delete_ad_set**](AdSetApi.md#delete_ad_set) | **DELETE** /{ad_set_id} | 
[**get_ad_set**](AdSetApi.md#get_ad_set) | **GET** /{ad_set_id} | 
[**get_ad_sets**](AdSetApi.md#get_ad_sets) | **GET** /act_{ad_account_id}/adsets | 
[**update_ad_set**](AdSetApi.md#update_ad_set) | **POST** /{ad_set_id} | 



## create_ad_set

> models::AdSetResponse create_ad_set(ad_account_id, properties)


Cerates an ad set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**properties** | [**AdSet**](.md) | Ad set properties | [required] |

### Return type

[**models::AdSetResponse**](AdSetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ad_set

> models::AdSetResponse delete_ad_set(ad_set_id)


Deletes a ad set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_set_id** | **String** | ID of the ad set. | [required] |

### Return type

[**models::AdSetResponse**](AdSetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ad_set

> models::AdSet get_ad_set(ad_set_id, date_preset, time_range, fields)


Return date related to an ad set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_set_id** | **String** | ID of the ad set. | [required] |
**date_preset** | Option<**String**> | Predefined date range used to aggregate insights metrics. |  |
**time_range** | Option<[**TimeRange**](.md)> | Time Range. Note if time range is invalid, it will be ignored. |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields of the ad set |  |

### Return type

[**models::AdSet**](AdSet.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ad_sets

> models::AdSetList get_ad_sets(ad_account_id, date_preset, time_range, fields)


Returns all ad sets from one ad account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**date_preset** | Option<**String**> | Predefined date range used to aggregate insights metrics. |  |
**time_range** | Option<[**TimeRange**](.md)> | Time Range. Note if time range is invalid, it will be ignored. |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields of the ad set |  |

### Return type

[**models::AdSetList**](AdSetList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ad_set

> models::AdSetResponse update_ad_set(ad_set_id, properties)


Updates an ad set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_set_id** | **String** | ID of the ad set. | [required] |
**properties** | [**AdSetUpdate**](.md) | Ad set update properties | [required] |

### Return type

[**models::AdSetResponse**](AdSetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

