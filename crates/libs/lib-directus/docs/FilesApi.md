# \FilesApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_file**](FilesApi.md#create_file) | **POST** /files | Create a File
[**delete_file**](FilesApi.md#delete_file) | **DELETE** /files/{id} | Delete a File
[**delete_files**](FilesApi.md#delete_files) | **DELETE** /files | Delete Multiple Files
[**get_file**](FilesApi.md#get_file) | **GET** /files/{id} | Retrieve a Files
[**get_files**](FilesApi.md#get_files) | **GET** /files | List Files
[**update_file**](FilesApi.md#update_file) | **PATCH** /files/{id} | Update a File
[**update_files**](FilesApi.md#update_files) | **PATCH** /files | Update Multiple Files



## create_file

> models::CreateFile200Response create_file(create_file_request)
Create a File

Create a new file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_file_request** | Option<[**CreateFileRequest**](CreateFileRequest.md)> |  |  |

### Return type

[**models::CreateFile200Response**](createFile_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> delete_file(id)
Delete a File

Delete an existing file.

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


## delete_files

> delete_files()
Delete Multiple Files

Delete multiple existing files.

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


## get_file

> models::CreateFile200Response get_file(id, fields, meta)
Retrieve a Files

Retrieve a single file by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateFile200Response**](createFile_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files

> models::GetFiles200Response get_files(fields, limit, offset, sort, filter, search, meta)
List Files

List the files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**serde_json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::GetFiles200Response**](getFiles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_file

> models::CreateFile200Response update_file(id, file, fields, meta, title, filename_download, description, folder, tags)
Update a File

Update an existing file, and/or replace it's file contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**file** | Option<[**serde_json::Value**](serde_json::Value.md)> | File contents. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**title** | Option<**String**> | Title for the file. Is extracted from the filename on upload, but can be edited by the user. |  |
**filename_download** | Option<**String**> | Preferred filename when file is downloaded. |  |
**description** | Option<**String**> | Description for the file. |  |
**folder** | Option<[**models::UpdateFileRequestFolder**](updateFile_request_folder.md)> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> | Tags for the file. Is automatically populated based on Exif data for images. |  |

### Return type

[**models::CreateFile200Response**](createFile_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_files

> models::GetFiles200Response update_files(fields, limit, meta, offset, sort, filter, search, update_files_request)
Update Multiple Files

Update multiple files at the same time.

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
**update_files_request** | Option<[**UpdateFilesRequest**](UpdateFilesRequest.md)> |  |  |

### Return type

[**models::GetFiles200Response**](getFiles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

