# \ApplicationsApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dockercompose_application**](ApplicationsApi.md#create_dockercompose_application) | **POST** /applications/dockercompose | Create (Docker Compose)
[**create_dockerfile_application**](ApplicationsApi.md#create_dockerfile_application) | **POST** /applications/dockerfile | Create (Dockerfile)
[**create_dockerimage_application**](ApplicationsApi.md#create_dockerimage_application) | **POST** /applications/dockerimage | Create (Docker Image)
[**create_env_by_application_uuid**](ApplicationsApi.md#create_env_by_application_uuid) | **POST** /applications/{uuid}/envs | Create Env
[**create_private_deploy_key_application**](ApplicationsApi.md#create_private_deploy_key_application) | **POST** /applications/private-deploy-key | Create (Private - Deploy Key)
[**create_private_github_app_application**](ApplicationsApi.md#create_private_github_app_application) | **POST** /applications/private-github-app | Create (Private - GH App)
[**create_public_application**](ApplicationsApi.md#create_public_application) | **POST** /applications/public | Create (Public)
[**delete_application_by_uuid**](ApplicationsApi.md#delete_application_by_uuid) | **DELETE** /applications/{uuid} | Delete
[**delete_env_by_application_uuid**](ApplicationsApi.md#delete_env_by_application_uuid) | **DELETE** /applications/{uuid}/envs/{env_uuid} | Delete Env
[**execute_command_application**](ApplicationsApi.md#execute_command_application) | **POST** /applications/{uuid}/execute | Execute Command
[**get_application_by_uuid**](ApplicationsApi.md#get_application_by_uuid) | **GET** /applications/{uuid} | Get
[**list_applications**](ApplicationsApi.md#list_applications) | **GET** /applications | List
[**list_envs_by_application_uuid**](ApplicationsApi.md#list_envs_by_application_uuid) | **GET** /applications/{uuid}/envs | List Envs
[**restart_application_by_uuid**](ApplicationsApi.md#restart_application_by_uuid) | **GET** /applications/{uuid}/restart | Restart
[**start_application_by_uuid**](ApplicationsApi.md#start_application_by_uuid) | **GET** /applications/{uuid}/start | Start
[**stop_application_by_uuid**](ApplicationsApi.md#stop_application_by_uuid) | **GET** /applications/{uuid}/stop | Stop
[**update_application_by_uuid**](ApplicationsApi.md#update_application_by_uuid) | **PATCH** /applications/{uuid} | Update
[**update_env_by_application_uuid**](ApplicationsApi.md#update_env_by_application_uuid) | **PATCH** /applications/{uuid}/envs | Update Env
[**update_envs_by_application_uuid**](ApplicationsApi.md#update_envs_by_application_uuid) | **PATCH** /applications/{uuid}/envs/bulk | Update Envs (Bulk)



## create_dockercompose_application

> create_dockercompose_application(create_dockercompose_application_request)
Create (Docker Compose)

Create new application based on a docker-compose file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dockercompose_application_request** | [**CreateDockercomposeApplicationRequest**](CreateDockercomposeApplicationRequest.md) | Application object that needs to be created. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dockerfile_application

> create_dockerfile_application(create_dockerfile_application_request)
Create (Dockerfile)

Create new application based on a simple Dockerfile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dockerfile_application_request** | [**CreateDockerfileApplicationRequest**](CreateDockerfileApplicationRequest.md) | Application object that needs to be created. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dockerimage_application

> create_dockerimage_application(create_dockerimage_application_request)
Create (Docker Image)

Create new application based on a prebuilt docker image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dockerimage_application_request** | [**CreateDockerimageApplicationRequest**](CreateDockerimageApplicationRequest.md) | Application object that needs to be created. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_env_by_application_uuid

> models::CreateEnvByApplicationUuid201Response create_env_by_application_uuid(uuid, create_env_by_application_uuid_request)
Create Env

Create env by application UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**create_env_by_application_uuid_request** | [**CreateEnvByApplicationUuidRequest**](CreateEnvByApplicationUuidRequest.md) | Env created. | [required] |

### Return type

[**models::CreateEnvByApplicationUuid201Response**](create_env_by_application_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_private_deploy_key_application

> create_private_deploy_key_application(create_private_deploy_key_application_request)
Create (Private - Deploy Key)

Create new application based on a private repository through a Deploy Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_private_deploy_key_application_request** | [**CreatePrivateDeployKeyApplicationRequest**](CreatePrivateDeployKeyApplicationRequest.md) | Application object that needs to be created. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_private_github_app_application

> create_private_github_app_application(create_private_github_app_application_request)
Create (Private - GH App)

Create new application based on a private repository through a Github App.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_private_github_app_application_request** | [**CreatePrivateGithubAppApplicationRequest**](CreatePrivateGithubAppApplicationRequest.md) | Application object that needs to be created. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_public_application

> create_public_application(create_public_application_request)
Create (Public)

Create new application based on a public git repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_public_application_request** | [**CreatePublicApplicationRequest**](CreatePublicApplicationRequest.md) | Application object that needs to be created. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_by_uuid

> models::DeleteApplicationByUuid200Response delete_application_by_uuid(uuid, delete_configurations, delete_volumes, docker_cleanup, delete_connected_networks)
Delete

Delete application by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**delete_configurations** | Option<**bool**> | Delete configurations. |  |[default to true]
**delete_volumes** | Option<**bool**> | Delete volumes. |  |[default to true]
**docker_cleanup** | Option<**bool**> | Run docker cleanup. |  |[default to true]
**delete_connected_networks** | Option<**bool**> | Delete connected networks. |  |[default to true]

### Return type

[**models::DeleteApplicationByUuid200Response**](delete_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_env_by_application_uuid

> models::DeleteEnvByApplicationUuid200Response delete_env_by_application_uuid(uuid, env_uuid)
Delete Env

Delete env by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**env_uuid** | **uuid::Uuid** | UUID of the environment variable. | [required] |

### Return type

[**models::DeleteEnvByApplicationUuid200Response**](delete_env_by_application_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_command_application

> models::ExecuteCommandApplication200Response execute_command_application(uuid, execute_command_application_request)
Execute Command

Execute a command on the application's current container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**execute_command_application_request** | [**ExecuteCommandApplicationRequest**](ExecuteCommandApplicationRequest.md) | Command to execute. | [required] |

### Return type

[**models::ExecuteCommandApplication200Response**](execute_command_application_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_by_uuid

> models::Application get_application_by_uuid(uuid)
Get

Get application by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_applications

> Vec<models::Application> list_applications()
List

List all applications.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Application>**](Application.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_envs_by_application_uuid

> Vec<models::EnvironmentVariable> list_envs_by_application_uuid(uuid)
List Envs

List all envs by application UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |

### Return type

[**Vec<models::EnvironmentVariable>**](EnvironmentVariable.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_application_by_uuid

> models::RestartApplicationByUuid200Response restart_application_by_uuid(uuid)
Restart

Restart application. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |

### Return type

[**models::RestartApplicationByUuid200Response**](restart_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_application_by_uuid

> models::StartApplicationByUuid200Response start_application_by_uuid(uuid, force, instant_deploy)
Start

Start application. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**force** | Option<**bool**> | Force rebuild. |  |[default to false]
**instant_deploy** | Option<**bool**> | Instant deploy (skip queuing). |  |[default to false]

### Return type

[**models::StartApplicationByUuid200Response**](start_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_application_by_uuid

> models::StopApplicationByUuid200Response stop_application_by_uuid(uuid)
Stop

Stop application. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |

### Return type

[**models::StopApplicationByUuid200Response**](stop_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_by_uuid

> models::UpdateApplicationByUuid200Response update_application_by_uuid(update_application_by_uuid_request)
Update

Update application by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_application_by_uuid_request** | [**UpdateApplicationByUuidRequest**](UpdateApplicationByUuidRequest.md) | Application updated. | [required] |

### Return type

[**models::UpdateApplicationByUuid200Response**](update_application_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_env_by_application_uuid

> models::UpdateEnvByApplicationUuid201Response update_env_by_application_uuid(uuid, update_env_by_application_uuid_request)
Update Env

Update env by application UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**update_env_by_application_uuid_request** | [**UpdateEnvByApplicationUuidRequest**](UpdateEnvByApplicationUuidRequest.md) | Env updated. | [required] |

### Return type

[**models::UpdateEnvByApplicationUuid201Response**](update_env_by_application_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_envs_by_application_uuid

> models::UpdateEnvsByApplicationUuid201Response update_envs_by_application_uuid(uuid, update_envs_by_application_uuid_request)
Update Envs (Bulk)

Update multiple envs by application UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |
**update_envs_by_application_uuid_request** | [**UpdateEnvsByApplicationUuidRequest**](UpdateEnvsByApplicationUuidRequest.md) | Bulk envs updated. | [required] |

### Return type

[**models::UpdateEnvsByApplicationUuid201Response**](update_envs_by_application_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

