# \ProjectsApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectsApi.md#create_project) | **POST** /projects | Create
[**delete_project_by_uuid**](ProjectsApi.md#delete_project_by_uuid) | **DELETE** /projects/{uuid} | Delete
[**get_environment_by_name**](ProjectsApi.md#get_environment_by_name) | **GET** /projects/{uuid}/{environment_name} | Environment
[**get_project_by_uuid**](ProjectsApi.md#get_project_by_uuid) | **GET** /projects/{uuid} | Get
[**list_projects**](ProjectsApi.md#list_projects) | **GET** /projects | List
[**update_project_by_uuid**](ProjectsApi.md#update_project_by_uuid) | **PATCH** /projects/{uuid} | Update



## create_project

> models::CreateProject201Response create_project(create_project_request)
Create

Create Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_request** | [**CreateProjectRequest**](CreateProjectRequest.md) | Project created. | [required] |

### Return type

[**models::CreateProject201Response**](create_project_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_by_uuid

> models::DeleteProjectByUuid200Response delete_project_by_uuid(uuid)
Delete

Delete project by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the application. | [required] |

### Return type

[**models::DeleteProjectByUuid200Response**](delete_project_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_by_name

> models::Environment get_environment_by_name(uuid, environment_name)
Environment

Get environment by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Project UUID | [required] |
**environment_name** | **String** | Environment name | [required] |

### Return type

[**models::Environment**](Environment.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_by_uuid

> models::Project get_project_by_uuid(uuid)
Get

Get project by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Project UUID | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> Vec<models::Project> list_projects()
List

List projects.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_by_uuid

> models::UpdateProjectByUuid201Response update_project_by_uuid(create_project_request)
Update

Update Project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_request** | [**CreateProjectRequest**](CreateProjectRequest.md) | Project updated. | [required] |

### Return type

[**models::UpdateProjectByUuid201Response**](update_project_by_uuid_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

