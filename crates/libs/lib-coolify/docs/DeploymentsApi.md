# \DeploymentsApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_by_tag_or_uuid**](DeploymentsApi.md#deploy_by_tag_or_uuid) | **GET** /deploy | Deploy
[**get_deployment_by_uuid**](DeploymentsApi.md#get_deployment_by_uuid) | **GET** /deployments/{uuid} | Get
[**list_deployments**](DeploymentsApi.md#list_deployments) | **GET** /deployments | List



## deploy_by_tag_or_uuid

> models::DeployByTagOrUuid200Response deploy_by_tag_or_uuid(tag, uuid, force)
Deploy

Deploy by tag or uuid. `Post` request also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | Option<**String**> | Tag name(s). Comma separated list is also accepted. |  |
**uuid** | Option<**String**> | Resource UUID(s). Comma separated list is also accepted. |  |
**force** | Option<**bool**> | Force rebuild (without cache) |  |

### Return type

[**models::DeployByTagOrUuid200Response**](deploy_by_tag_or_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_by_uuid

> models::ApplicationDeploymentQueue get_deployment_by_uuid(uuid)
Get

Get deployment by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Deployment UUID | [required] |

### Return type

[**models::ApplicationDeploymentQueue**](ApplicationDeploymentQueue.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployments

> Vec<models::ApplicationDeploymentQueue> list_deployments()
List

List currently running deployments

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationDeploymentQueue>**](ApplicationDeploymentQueue.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

