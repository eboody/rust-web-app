# \PresetsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_preset**](PresetsApi.md#create_preset) | **POST** /presets | Create a Preset
[**delete_preset**](PresetsApi.md#delete_preset) | **DELETE** /presets/{id} | Delete a Preset
[**delete_presets**](PresetsApi.md#delete_presets) | **DELETE** /presets | Delete Multiple Presets
[**get_preset**](PresetsApi.md#get_preset) | **GET** /presets/{id} | Retrieve a Preset
[**get_presets**](PresetsApi.md#get_presets) | **GET** /presets | List Presets
[**update_preset**](PresetsApi.md#update_preset) | **PATCH** /presets/{id} | Update a Preset
[**update_presets**](PresetsApi.md#update_presets) | **PATCH** /presets | Update Multiple Presets



## create_preset

> models::CreatePreset200Response create_preset(fields, meta, create_preset_request)
Create a Preset

Create a new preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_preset_request** | Option<[**CreatePresetRequest**](CreatePresetRequest.md)> |  |  |

### Return type

[**models::CreatePreset200Response**](createPreset_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_preset

> delete_preset(id)
Delete a Preset

Delete an existing preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |

### Return type

 (empty response body)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_presets

> delete_presets()
Delete Multiple Presets

Delete multiple existing presets.

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


## get_preset

> models::CreatePreset200Response get_preset(id, fields, meta)
Retrieve a Preset

Retrieve a single preset by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreatePreset200Response**](createPreset_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_presets

> models::GetPresets200Response get_presets(fields, limit, offset, page, sort, filter, search, meta)
List Presets

List the presets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**page** | Option<**i32**> | Cursor for use in pagination. Often used in combination with limit. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**serde_json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::GetPresets200Response**](getPresets_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preset

> models::CreatePreset200Response update_preset(id, fields, meta, update_preset_request)
Update a Preset

Update an existing preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**update_preset_request** | Option<[**UpdatePresetRequest**](UpdatePresetRequest.md)> |  |  |

### Return type

[**models::CreatePreset200Response**](createPreset_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_presets

> models::GetPresets200Response update_presets(fields, limit, meta, offset, sort, filter, search, update_presets_request)
Update Multiple Presets

Update multiple presets at the same time.

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
**update_presets_request** | Option<[**UpdatePresetsRequest**](UpdatePresetsRequest.md)> |  |  |

### Return type

[**models::GetPresets200Response**](getPresets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

