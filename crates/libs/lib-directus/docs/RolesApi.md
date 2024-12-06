# \RolesApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_role**](RolesApi.md#create_role) | **POST** /roles | Create a Role
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /roles/{id} | Delete a Role
[**delete_roles**](RolesApi.md#delete_roles) | **DELETE** /roles | Delete Multiple Roles
[**get_role**](RolesApi.md#get_role) | **GET** /roles/{id} | Retrieve a Role
[**get_roles**](RolesApi.md#get_roles) | **GET** /roles | List Roles
[**update_role**](RolesApi.md#update_role) | **PATCH** /roles/{id} | Update a Role
[**update_roles**](RolesApi.md#update_roles) | **PATCH** /roles | Update Multiple Roles



## create_role

> models::CreateRole200Response create_role(fields, meta, create_role_request)
Create a Role

Create a new role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_role_request** | Option<[**CreateRoleRequest**](CreateRoleRequest.md)> |  |  |

### Return type

[**models::CreateRole200Response**](createRole_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> delete_role(id)
Delete a Role

Delete an existing role

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


## delete_roles

> delete_roles()
Delete Multiple Roles

Delete multiple existing roles.

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


## get_role

> models::CreateRole200Response get_role(id, fields, meta)
Retrieve a Role

Retrieve a single role by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateRole200Response**](createRole_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> models::GetRoles200Response get_roles(fields, limit, offset, meta, sort, filter, search, page)
List Roles

List the roles.

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

[**models::GetRoles200Response**](getRoles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> models::CreateRole200Response update_role(id, fields, meta, update_role_request)
Update a Role

Update an existing role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**update_role_request** | Option<[**UpdateRoleRequest**](UpdateRoleRequest.md)> |  |  |

### Return type

[**models::CreateRole200Response**](createRole_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_roles

> models::GetRoles200Response update_roles(fields, limit, meta, offset, sort, filter, search, update_roles_request)
Update Multiple Roles

Update multiple roles at the same time.

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
**update_roles_request** | Option<[**UpdateRolesRequest**](UpdateRolesRequest.md)> |  |  |

### Return type

[**models::GetRoles200Response**](getRoles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

