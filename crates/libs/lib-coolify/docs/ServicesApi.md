# \ServicesApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_env_by_service_uuid**](ServicesApi.md#create_env_by_service_uuid) | **POST** /services/{uuid}/envs | Create Env
[**create_service**](ServicesApi.md#create_service) | **POST** /services | Create
[**delete_env_by_service_uuid**](ServicesApi.md#delete_env_by_service_uuid) | **DELETE** /services/{uuid}/envs/{env_uuid} | Delete Env
[**delete_service_by_uuid**](ServicesApi.md#delete_service_by_uuid) | **DELETE** /services/{uuid} | Delete
[**get_service_by_uuid**](ServicesApi.md#get_service_by_uuid) | **GET** /services/{uuid} | Get
[**list_envs_by_service_uuid**](ServicesApi.md#list_envs_by_service_uuid) | **GET** /services/{uuid}/envs | List Envs
[**list_services**](ServicesApi.md#list_services) | **GET** /services | List
[**restart_service_by_uuid**](ServicesApi.md#restart_service_by_uuid) | **GET** /services/{uuid}/restart | Restart
[**start_service_by_uuid**](ServicesApi.md#start_service_by_uuid) | **GET** /services/{uuid}/start | Start
[**stop_service_by_uuid**](ServicesApi.md#stop_service_by_uuid) | **GET** /services/{uuid}/stop | Stop
[**update_env_by_service_uuid**](ServicesApi.md#update_env_by_service_uuid) | **PATCH** /services/{uuid}/envs | Update Env
[**update_envs_by_service_uuid**](ServicesApi.md#update_envs_by_service_uuid) | **PATCH** /services/{uuid}/envs/bulk | Update Envs (Bulk)



## create_env_by_service_uuid

> models::CreateEnvByApplicationUuid201Response create_env_by_service_uuid(uuid, create_env_by_application_uuid_request)
Create Env

Create env by service UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |
**create_env_by_application_uuid_request** | [**CreateEnvByApplicationUuidRequest**](CreateEnvByApplicationUuidRequest.md) | Env created. | [required] |

### Return type

[**models::CreateEnvByApplicationUuid201Response**](create_env_by_application_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service

> models::CreateService201Response create_service(create_service_request)
Create

Create a one-click service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service_request** | [**CreateServiceRequest**](CreateServiceRequest.md) |  | [required] |

### Return type

[**models::CreateService201Response**](create_service_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_env_by_service_uuid

> models::DeleteEnvByApplicationUuid200Response delete_env_by_service_uuid(uuid, env_uuid)
Delete Env

Delete env by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |
**env_uuid** | **uuid::Uuid** | UUID of the environment variable. | [required] |

### Return type

[**models::DeleteEnvByApplicationUuid200Response**](delete_env_by_application_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_by_uuid

> models::DeleteServiceByUuid200Response delete_service_by_uuid(uuid, delete_configurations, delete_volumes, docker_cleanup, delete_connected_networks)
Delete

Delete service by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Service UUID | [required] |
**delete_configurations** | Option<**bool**> | Delete configurations. |  |[default to true]
**delete_volumes** | Option<**bool**> | Delete volumes. |  |[default to true]
**docker_cleanup** | Option<**bool**> | Run docker cleanup. |  |[default to true]
**delete_connected_networks** | Option<**bool**> | Delete connected networks. |  |[default to true]

### Return type

[**models::DeleteServiceByUuid200Response**](delete_service_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_by_uuid

> models::Service get_service_by_uuid(uuid)
Get

Get service by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Service UUID | [required] |

### Return type

[**models::Service**](Service.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_envs_by_service_uuid

> Vec<models::EnvironmentVariable> list_envs_by_service_uuid(uuid)
List Envs

List all envs by service UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |

### Return type

[**Vec<models::EnvironmentVariable>**](EnvironmentVariable.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_services

> Vec<models::Service> list_services()
List

List all services.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Service>**](Service.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_service_by_uuid

> models::RestartServiceByUuid200Response restart_service_by_uuid(uuid)
Restart

Restart service. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |

### Return type

[**models::RestartServiceByUuid200Response**](restart_service_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_service_by_uuid

> models::StartServiceByUuid200Response start_service_by_uuid(uuid)
Start

Start service. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |

### Return type

[**models::StartServiceByUuid200Response**](start_service_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_service_by_uuid

> models::StopServiceByUuid200Response stop_service_by_uuid(uuid)
Stop

Stop service. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |

### Return type

[**models::StopServiceByUuid200Response**](stop_service_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_env_by_service_uuid

> models::UpdateEnvByApplicationUuid201Response update_env_by_service_uuid(uuid, update_env_by_application_uuid_request)
Update Env

Update env by service UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |
**update_env_by_application_uuid_request** | [**UpdateEnvByApplicationUuidRequest**](UpdateEnvByApplicationUuidRequest.md) | Env updated. | [required] |

### Return type

[**models::UpdateEnvByApplicationUuid201Response**](update_env_by_application_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_envs_by_service_uuid

> models::UpdateEnvsByApplicationUuid201Response update_envs_by_service_uuid(uuid, update_envs_by_application_uuid_request)
Update Envs (Bulk)

Update multiple envs by service UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the service. | [required] |
**update_envs_by_application_uuid_request** | [**UpdateEnvsByApplicationUuidRequest**](UpdateEnvsByApplicationUuidRequest.md) | Bulk envs updated. | [required] |

### Return type

[**models::UpdateEnvsByApplicationUuid201Response**](update_envs_by_application_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

