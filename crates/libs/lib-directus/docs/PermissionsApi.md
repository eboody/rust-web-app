# \PermissionsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_permission**](PermissionsApi.md#create_permission) | **POST** /permissions | Create a Permission
[**delete_permission**](PermissionsApi.md#delete_permission) | **DELETE** /permissions/{id} | Delete a Permission
[**delete_permissions**](PermissionsApi.md#delete_permissions) | **DELETE** /permissions | Delete Multiple Permissions
[**get_my_permissions**](PermissionsApi.md#get_my_permissions) | **GET** /permissions/me | List My Permissions
[**get_permission**](PermissionsApi.md#get_permission) | **GET** /permissions/{id} | Retrieve a Permission
[**get_permissions**](PermissionsApi.md#get_permissions) | **GET** /permissions | List Permissions
[**update_permission**](PermissionsApi.md#update_permission) | **PATCH** /permissions/{id} | Update a Permission
[**update_permissions**](PermissionsApi.md#update_permissions) | **PATCH** /permissions | Update Multiple Permissions



## create_permission

> models::CreatePermission200Response create_permission(meta, create_permission_request)
Create a Permission

Create a new permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_permission_request** | Option<[**CreatePermissionRequest**](CreatePermissionRequest.md)> |  |  |

### Return type

[**models::CreatePermission200Response**](createPermission_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_permission

> delete_permission(id)
Delete a Permission

Delete an existing permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_permissions

> delete_permissions()
Delete Multiple Permissions

Delete multiple existing permissions.

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


## get_my_permissions

> models::GetMyPermissions200Response get_my_permissions()
List My Permissions

List the permissions that apply to the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetMyPermissions200Response**](getMyPermissions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permission

> models::CreatePermission200Response get_permission(id, fields, meta)
Retrieve a Permission

Retrieve a single permissions object by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreatePermission200Response**](createPermission_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions

> models::GetPermissions200Response get_permissions(fields, limit, offset, meta, sort, filter, search, page)
List Permissions

List all permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**page** | Option<**i32**> | Cursor for use in pagination. Often used in combination with limit. |  |

### Return type

[**models::GetPermissions200Response**](getPermissions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permission

> models::CreatePermission200Response update_permission(id, meta, update_permission_request)
Update a Permission

Update an existing permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**update_permission_request** | Option<[**UpdatePermissionRequest**](UpdatePermissionRequest.md)> |  |  |

### Return type

[**models::CreatePermission200Response**](createPermission_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permissions

> models::GetPermissions200Response update_permissions(fields, limit, meta, offset, sort, filter, search, update_permissions_request)
Update Multiple Permissions

Update multiple permissions at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**update_permissions_request** | Option<[**UpdatePermissionsRequest**](UpdatePermissionsRequest.md)> |  |  |

### Return type

[**models::GetPermissions200Response**](getPermissions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

