# \SettingsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_settings**](SettingsApi.md#get_settings) | **GET** /settings | Retrieve Settings
[**update_setting**](SettingsApi.md#update_setting) | **PATCH** /settings | Update Settings



## get_settings

> models::GetSettings200Response get_settings(limit, offset, meta, page)
Retrieve Settings

List the settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**page** | Option<**i32**> | Cursor for use in pagination. Often used in combination with limit. |  |

### Return type

[**models::GetSettings200Response**](getSettings_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_setting

> models::GetSettings200Response update_setting(body)
Update Settings

Update the settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**json::Value**> |  |  |

### Return type

[**models::GetSettings200Response**](getSettings_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

