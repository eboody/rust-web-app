# \TeamsApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_team**](TeamsApi.md#get_current_team) | **GET** /teams/current | Authenticated Team
[**get_current_team_members**](TeamsApi.md#get_current_team_members) | **GET** /teams/current/members | Authenticated Team Members
[**get_members_by_team_id**](TeamsApi.md#get_members_by_team_id) | **GET** /teams/{id}/members | Members
[**get_team_by_id**](TeamsApi.md#get_team_by_id) | **GET** /teams/{id} | Get
[**list_teams**](TeamsApi.md#list_teams) | **GET** /teams | List



## get_current_team

> models::Team get_current_team()
Authenticated Team

Get currently authenticated team.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_team_members

> Vec<models::User> get_current_team_members()
Authenticated Team Members

Get currently authenticated team members.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_members_by_team_id

> Vec<models::User> get_members_by_team_id(id)
Members

Get members by TeamId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Team ID | [required] |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_by_id

> models::Team get_team_by_id(id)
Get

Get team by TeamId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Team ID | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_teams

> Vec<models::Team> list_teams()
List

Get all teams.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

