# \UsersApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invite**](UsersApi.md#accept_invite) | **POST** /users/invite/accept | Accept User Invite
[**create_user**](UsersApi.md#create_user) | **POST** /users | Create a User
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /users/{id} | Delete a User
[**delete_users**](UsersApi.md#delete_users) | **DELETE** /users | Delete Multiple Users
[**get_me**](UsersApi.md#get_me) | **GET** /users/me | Retrieve Current User
[**get_user**](UsersApi.md#get_user) | **GET** /users/{id} | Retrieve a User
[**get_users**](UsersApi.md#get_users) | **GET** /users | List Users
[**invite**](UsersApi.md#invite) | **POST** /users/invite | Invite User(s)
[**me_tfa_disable**](UsersApi.md#me_tfa_disable) | **POST** /users/me/tfa/disable | Disable 2FA
[**me_tfa_enable**](UsersApi.md#me_tfa_enable) | **POST** /users/me/tfa/enable | Enable 2FA
[**update_last_used_page_me**](UsersApi.md#update_last_used_page_me) | **PATCH** /users/me/track/page | Update Last Page
[**update_me**](UsersApi.md#update_me) | **PATCH** /users/me | Update Current User
[**update_user**](UsersApi.md#update_user) | **PATCH** /users/{id} | Update a User
[**update_users**](UsersApi.md#update_users) | **PATCH** /users | Update Multiple Users



## accept_invite

> models::CreateUser200Response accept_invite(accept_invite_request)
Accept User Invite

Accepts and enables an invited user using a JWT invitation token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_invite_request** | [**AcceptInviteRequest**](AcceptInviteRequest.md) |  | [required] |

### Return type

[**models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::CreateUser200Response create_user(meta, users)
Create a User

Create a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**users** | Option<[**Users**](Users.md)> |  |  |

### Return type

[**models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(id)
Delete a User

Delete an existing user

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


## delete_users

> delete_users()
Delete Multiple Users

Delete multiple existing users.

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


## get_me

> models::CreateUser200Response get_me(fields, meta)
Retrieve Current User

Retrieve the currently authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::CreateUser200Response get_user(id, fields, meta)
Retrieve a User

Retrieve a single user by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> models::GetUsers200Response get_users(fields, limit, offset, meta, sort, filter, search)
List Users

List the users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**serde_json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |

### Return type

[**models::GetUsers200Response**](getUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite

> models::CreateUser200Response invite(invite_request)
Invite User(s)

Invites one or more users to this project. It creates a user with an invited status, and then sends an email to the user with instructions on how to activate their account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_request** | Option<[**InviteRequest**](InviteRequest.md)> |  |  |

### Return type

[**models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## me_tfa_disable

> me_tfa_disable()
Disable 2FA

Disables two-factor authentication for the currently authenticated user.

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


## me_tfa_enable

> me_tfa_enable()
Enable 2FA

Enables two-factor authentication for the currently authenticated user.

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


## update_last_used_page_me

> update_last_used_page_me(update_last_used_page_me_request)
Update Last Page

Updates the last used page field of the currently authenticated user. This is used internally to be able to open the Directus admin app from the last page you used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_last_used_page_me_request** | Option<[**UpdateLastUsedPageMeRequest**](UpdateLastUsedPageMeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_me

> models::CreateUser200Response update_me()
Update Current User

Update the currently authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::ServerInfo200Response update_user(id, fields, meta, users)
Update a User

Update an existing user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the object. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**users** | Option<[**Users**](Users.md)> |  |  |

### Return type

[**models::ServerInfo200Response**](serverInfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_users

> models::GetUsers200Response update_users(fields, limit, meta, offset, sort, filter, search, update_users_request)
Update Multiple Users

Update multiple users at the same time.

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
**update_users_request** | Option<[**UpdateUsersRequest**](UpdateUsersRequest.md)> |  |  |

### Return type

[**models::GetUsers200Response**](getUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

