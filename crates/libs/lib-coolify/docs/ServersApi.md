# \ServersApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_server**](ServersApi.md#create_server) | **POST** /servers | Create
[**delete_server_by_uuid**](ServersApi.md#delete_server_by_uuid) | **DELETE** /servers/{uuid} | Delete
[**get_domains_by_server_uuid**](ServersApi.md#get_domains_by_server_uuid) | **GET** /servers/{uuid}/domains | Domains
[**get_resources_by_server_uuid**](ServersApi.md#get_resources_by_server_uuid) | **GET** /servers/{uuid}/resources | Resources
[**get_server_by_uuid**](ServersApi.md#get_server_by_uuid) | **GET** /servers/{uuid} | Get
[**list_servers**](ServersApi.md#list_servers) | **GET** /servers | List
[**update_server_by_uuid**](ServersApi.md#update_server_by_uuid) | **PATCH** /servers/{uuid} | Update
[**validate_server_by_uuid**](ServersApi.md#validate_server_by_uuid) | **GET** /servers/{uuid}/validate | Validate



## create_server

> models::CreateServer201Response create_server(create_server_request)
Create

Create Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_server_request** | [**CreateServerRequest**](CreateServerRequest.md) | Server created. | [required] |

### Return type

[**models::CreateServer201Response**](create_server_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server_by_uuid

> models::DeleteServerByUuid200Response delete_server_by_uuid(uuid)
Delete

Delete server by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the server. | [required] |

### Return type

[**models::DeleteServerByUuid200Response**](delete_server_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domains_by_server_uuid

> Vec<models::GetDomainsByServerUuid200ResponseInner> get_domains_by_server_uuid(uuid)
Domains

Get domains by server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Server's UUID | [required] |

### Return type

[**Vec<models::GetDomainsByServerUuid200ResponseInner>**](get_domains_by_server_uuid_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resources_by_server_uuid

> Vec<models::GetResourcesByServerUuid200ResponseInner> get_resources_by_server_uuid(uuid)
Resources

Get resources by server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Server's UUID | [required] |

### Return type

[**Vec<models::GetResourcesByServerUuid200ResponseInner>**](get_resources_by_server_uuid_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_by_uuid

> models::Server get_server_by_uuid(uuid)
Get

Get server by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Server's UUID | [required] |

### Return type

[**models::Server**](Server.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers

> Vec<models::Server> list_servers()
List

List all servers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Server>**](Server.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server_by_uuid

> Vec<models::Server> update_server_by_uuid(update_server_by_uuid_request)
Update

Update Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_server_by_uuid_request** | [**UpdateServerByUuidRequest**](UpdateServerByUuidRequest.md) | Server updated. | [required] |

### Return type

[**Vec<models::Server>**](Server.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_server_by_uuid

> models::ValidateServerByUuid201Response validate_server_by_uuid(uuid)
Validate

Validate server by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Server UUID | [required] |

### Return type

[**models::ValidateServerByUuid201Response**](validate_server_by_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

