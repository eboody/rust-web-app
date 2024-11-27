# \AssetsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_asset**](AssetsApi.md#get_asset) | **GET** /assets/{id} | Get an Asset



## get_asset

> String get_asset(id, key, transforms, download)
Get an Asset

Image typed files can be dynamically resized and transformed to fit any need.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the file. | [required] |
**key** | Option<**String**> | The key of the asset size configured in settings. |  |
**transforms** | Option<**String**> | A JSON array of image transformations |  |
**download** | Option<**bool**> | Download the asset to your computer |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

