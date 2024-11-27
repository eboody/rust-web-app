# \AuthenticationApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login**](AuthenticationApi.md#login) | **POST** /auth/login | Retrieve a Temporary Access Token
[**logout**](AuthenticationApi.md#logout) | **POST** /auth/logout | Log Out
[**oauth**](AuthenticationApi.md#oauth) | **GET** /auth/oauth | List OAuth Providers
[**oauth_provider**](AuthenticationApi.md#oauth_provider) | **GET** /auth/oauth/{provider} | Authenticated using an OAuth provider
[**password_request**](AuthenticationApi.md#password_request) | **POST** /auth/password/request | Request a Password Reset
[**password_reset**](AuthenticationApi.md#password_reset) | **POST** /auth/password/reset | Reset a Password
[**refresh**](AuthenticationApi.md#refresh) | **POST** /auth/refresh | Refresh Token



## login

> models::Login200Response login(login_request)
Retrieve a Temporary Access Token

Retrieve a Temporary Access Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_request** | Option<[**LoginRequest**](LoginRequest.md)> |  |  |

### Return type

[**models::Login200Response**](login_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> logout(logout_request)
Log Out

Log Out

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logout_request** | Option<[**LogoutRequest**](LogoutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth

> models::Oauth200Response oauth()
List OAuth Providers

List configured OAuth providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Oauth200Response**](oauth_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_provider

> models::OauthProvider200Response oauth_provider(provider, redirect)
Authenticated using an OAuth provider

Start OAuth flow using the specified provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | Key of the activated OAuth provider. | [required] |
**redirect** | Option<**String**> | Where to redirect on successful login.<br/>If set the authentication details are set inside cookies otherwise a JSON is returned. |  |

### Return type

[**models::OauthProvider200Response**](oauthProvider_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## password_request

> password_request(password_request_request)
Request a Password Reset

Request a reset password email to be send.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_request_request** | Option<[**PasswordRequestRequest**](PasswordRequestRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## password_reset

> password_reset(password_reset_request)
Reset a Password

The request a password reset endpoint sends an email with a link to the admin app which in turn uses this endpoint to allow the user to reset their password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_reset_request** | Option<[**PasswordResetRequest**](PasswordResetRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh

> models::Refresh200Response refresh(refresh_request)
Refresh Token

Refresh a Temporary Access Token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_request** | Option<[**RefreshRequest**](RefreshRequest.md)> |  |  |

### Return type

[**models::Refresh200Response**](refresh_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

