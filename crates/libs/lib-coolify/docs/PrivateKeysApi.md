# \PrivateKeysApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_private_key**](PrivateKeysApi.md#create_private_key) | **POST** /security/keys | Create
[**delete_private_key_by_uuid**](PrivateKeysApi.md#delete_private_key_by_uuid) | **DELETE** /security/keys/{uuid} | Delete
[**get_private_key_by_uuid**](PrivateKeysApi.md#get_private_key_by_uuid) | **GET** /security/keys/{uuid} | Get
[**list_private_keys**](PrivateKeysApi.md#list_private_keys) | **GET** /security/keys | List
[**update_private_key**](PrivateKeysApi.md#update_private_key) | **PATCH** /security/keys | Update



## create_private_key

> models::UpdateApplicationByUuid200Response create_private_key(create_private_key_request)
Create

Create a new private key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_private_key_request** | [**CreatePrivateKeyRequest**](CreatePrivateKeyRequest.md) |  | [required] |

### Return type

[**models::UpdateApplicationByUuid200Response**](update_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_private_key_by_uuid

> models::DeletePrivateKeyByUuid200Response delete_private_key_by_uuid(uuid)
Delete

Delete a private key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Private Key UUID | [required] |

### Return type

[**models::DeletePrivateKeyByUuid200Response**](delete_private_key_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_key_by_uuid

> models::PrivateKey get_private_key_by_uuid(uuid)
Get

Get key by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Private Key UUID | [required] |

### Return type

[**models::PrivateKey**](PrivateKey.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_keys

> Vec<models::PrivateKey> list_private_keys()
List

List all private keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PrivateKey>**](PrivateKey.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_private_key

> models::UpdateApplicationByUuid200Response update_private_key(update_private_key_request)
Update

Update a private key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_private_key_request** | [**UpdatePrivateKeyRequest**](UpdatePrivateKeyRequest.md) |  | [required] |

### Return type

[**models::UpdateApplicationByUuid200Response**](update_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

