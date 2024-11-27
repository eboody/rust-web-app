# \AdApi

All URIs are relative to *https://graph.facebook.com/v12.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ad**](AdApi.md#create_ad) | **POST** /act_{ad_account_id}/ads | 
[**delete_ad**](AdApi.md#delete_ad) | **DELETE** /{ad_id} | 
[**get_ad**](AdApi.md#get_ad) | **GET** /{ad_id} | 
[**get_ads**](AdApi.md#get_ads) | **GET** /act_{ad_account_id}/ads | 
[**update_ad**](AdApi.md#update_ad) | **POST** /{ad_id} | 



## create_ad

> models::AdResponse create_ad(ad_account_id, properties)


Cerates an ad.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**properties** | [**Ad**](.md) | Ad properties | [required] |

### Return type

[**models::AdResponse**](AdResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ad

> models::AdResponse delete_ad(ad_id)


Deletes an ad.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_id** | **String** | ID of the ad | [required] |

### Return type

[**models::AdResponse**](AdResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ad

> models::Ad get_ad(ad_id, date_preset, time_range, updated_since, fields)


Returns data of an ad.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_id** | **String** | ID of the ad | [required] |
**date_preset** | Option<**String**> | Predefined date range used to aggregate insights metrics. |  |
**time_range** | Option<[**TimeRange**](.md)> | Date range used to aggregate insights metrics |  |
**updated_since** | Option<**i32**> | Updated since. |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields of the campaign |  |

### Return type

[**models::Ad**](Ad.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ads

> models::AdList get_ads(ad_account_id, date_preset, effective_status, time_range, updated_since, fields, summary)


Returns ads under this ad account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**date_preset** | Option<**String**> | Predefined date range used to aggregate insights metrics. |  |
**effective_status** | Option<[**Vec<String>**](String.md)> | Effective status for the ads |  |
**time_range** | Option<[**TimeRange**](.md)> | Date range used to aggregate insights metrics |  |
**updated_since** | Option<**i32**> | Updated since. |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields of the campaign |  |
**summary** | Option<[**Vec<String>**](String.md)> | Aggregated information about the edge, such as counts |  |

### Return type

[**models::AdList**](AdList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ad

> models::AdResponse update_ad(ad_id, properties)


Updates an ad.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_id** | **String** | ID of the ad | [required] |
**properties** | [**AdUpdate**](.md) | Ad set update properties | [required] |

### Return type

[**models::AdResponse**](AdResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

