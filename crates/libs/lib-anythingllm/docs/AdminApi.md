# \AdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_admin_invite_id_delete**](AdminApi.md#v1_admin_invite_id_delete) | **DELETE** /v1/admin/invite/{id} | 
[**v1_admin_invite_new_post**](AdminApi.md#v1_admin_invite_new_post) | **POST** /v1/admin/invite/new | 
[**v1_admin_invites_get**](AdminApi.md#v1_admin_invites_get) | **GET** /v1/admin/invites | 
[**v1_admin_is_multi_user_mode_get**](AdminApi.md#v1_admin_is_multi_user_mode_get) | **GET** /v1/admin/is-multi-user-mode | 
[**v1_admin_preferences_post**](AdminApi.md#v1_admin_preferences_post) | **POST** /v1/admin/preferences | 
[**v1_admin_users_get**](AdminApi.md#v1_admin_users_get) | **GET** /v1/admin/users | 
[**v1_admin_users_id_delete**](AdminApi.md#v1_admin_users_id_delete) | **DELETE** /v1/admin/users/{id} | 
[**v1_admin_users_id_post**](AdminApi.md#v1_admin_users_id_post) | **POST** /v1/admin/users/{id} | 
[**v1_admin_users_new_post**](AdminApi.md#v1_admin_users_new_post) | **POST** /v1/admin/users/new | 
[**v1_admin_workspace_chats_post**](AdminApi.md#v1_admin_workspace_chats_post) | **POST** /v1/admin/workspace-chats | 
[**v1_admin_workspaces_workspace_id_update_users_post**](AdminApi.md#v1_admin_workspaces_workspace_id_update_users_post) | **POST** /v1/admin/workspaces/{workspaceId}/update-users | 
[**v1_admin_workspaces_workspace_id_users_get**](AdminApi.md#v1_admin_workspaces_workspace_id_users_get) | **GET** /v1/admin/workspaces/{workspaceId}/users | 



## v1_admin_invite_id_delete

> serde_json::Value v1_admin_invite_id_delete(id)


Deactivates (soft-delete) invite by id. Methods are disabled until multi user mode is enabled via the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of the invite in the database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_invite_new_post

> serde_json::Value v1_admin_invite_new_post()


Create a new invite code for someone to use to register with instance. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_invites_get

> serde_json::Value v1_admin_invites_get()


List all existing invitations to instance regardless of status. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_is_multi_user_mode_get

> serde_json::Value v1_admin_is_multi_user_mode_get()


Check to see if the instance is in multi-user-mode first. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_preferences_post

> serde_json::Value v1_admin_preferences_post()


Update multi-user preferences for instance. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_users_get

> serde_json::Value v1_admin_users_get()


Check to see if the instance is in multi-user-mode first. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_users_id_delete

> serde_json::Value v1_admin_users_id_delete(id)


Delete existing user by id. Methods are disabled until multi user mode is enabled via the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of the user in the database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_users_id_post

> serde_json::Value v1_admin_users_id_post(id)


Update existing user settings. Methods are disabled until multi user mode is enabled via the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of the user in the database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_users_new_post

> serde_json::Value v1_admin_users_new_post()


Create a new user with username and password. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_workspace_chats_post

> serde_json::Value v1_admin_workspace_chats_post()


All chats in the system ordered by most recent. Methods are disabled until multi user mode is enabled via the UI.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_workspaces_workspace_id_update_users_post

> serde_json::Value v1_admin_workspaces_workspace_id_update_users_post(workspace_id)


Overwrite workspace permissions to only be accessible by the given user ids and admins. Methods are disabled until multi user mode is enabled via the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | id of the workspace in the database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_admin_workspaces_workspace_id_users_get

> serde_json::Value v1_admin_workspaces_workspace_id_users_get(workspace_id)


Retrieve a list of users with permissions to access the specified workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | id of the workspace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

