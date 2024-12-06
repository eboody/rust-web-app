# \RelationsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_relation**](RelationsApi.md#create_relation) | **POST** /relations | Create a Relation
[**delete_relation**](RelationsApi.md#delete_relation) | **DELETE** /relations/{id} | Delete a Relation
[**get_relation**](RelationsApi.md#get_relation) | **GET** /relations/{id} | Retrieve a Relation
[**get_relations**](RelationsApi.md#get_relations) | **GET** /relations | List Relations
[**update_relation**](RelationsApi.md#update_relation) | **PATCH** /relations/{id} | Update a Relation



## create_relation

> models::CreateRelation200Response create_relation(fields, meta, create_relation_request)
Create a Relation

Create a new relation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_relation_request** | Option<[**CreateRelationRequest**](CreateRelationRequest.md)> |  |  |

### Return type

[**models::CreateRelation200Response**](createRelation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_relation

> delete_relation(id)
Delete a Relation

Delete an existing relation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_relation

> models::CreateRelation200Response get_relation(id, fields, meta)
Retrieve a Relation

Retrieve a single relation by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateRelation200Response**](createRelation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_relations

> models::GetRelations200Response get_relations(fields, limit, offset, meta, sort, filter, search, page)
List Relations

List the relations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**page** | Option<**i32**> | Cursor for use in pagination. Often used in combination with limit. |  |

### Return type

[**models::GetRelations200Response**](getRelations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_relation

> models::CreateRelation200Response update_relation(id, fields, meta, update_relation_request)
Update a Relation

Update an existing relation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Index | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**update_relation_request** | Option<[**UpdateRelationRequest**](UpdateRelationRequest.md)> |  |  |

### Return type

[**models::CreateRelation200Response**](createRelation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

