# \FieldsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_field**](FieldsApi.md#create_field) | **POST** /fields/{collection} | Create Field in Collection
[**delete_field**](FieldsApi.md#delete_field) | **DELETE** /fields/{collection}/{id} | Delete a Field
[**get_collection_field**](FieldsApi.md#get_collection_field) | **GET** /fields/{collection}/{id} | Retrieve a Field
[**get_collection_fields**](FieldsApi.md#get_collection_fields) | **GET** /fields/{collection} | List Fields in Collection
[**get_fields**](FieldsApi.md#get_fields) | **GET** /fields | List All Fields
[**update_field**](FieldsApi.md#update_field) | **PATCH** /fields/{collection}/{id} | Update a Field



## create_field

> models::CreateField200Response create_field(collection, create_field_request)
Create Field in Collection

Create a new field in a given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Unique identifier of the collection the item resides in. | [required] |
**create_field_request** | Option<[**CreateFieldRequest**](CreateFieldRequest.md)> |  |  |

### Return type

[**models::CreateField200Response**](createField_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_field

> delete_field(collection, id)
Delete a Field

Delete an existing field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Unique identifier of the collection the item resides in. | [required] |
**id** | **String** | Unique identifier of the field. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_field

> models::CreateField200Response get_collection_field(collection, id)
Retrieve a Field

Retrieves the details of a single field in a given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Unique identifier of the collection the item resides in. | [required] |
**id** | **String** | Unique identifier of the field. | [required] |

### Return type

[**models::CreateField200Response**](createField_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_fields

> models::GetFields200Response get_collection_fields(collection, sort)
List Fields in Collection

Returns a list of the fields available in the given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Unique identifier of the collection the item resides in. | [required] |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |

### Return type

[**models::GetFields200Response**](getFields_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fields

> models::GetFields200Response get_fields(limit, sort)
List All Fields

Returns a list of the fields available in the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |

### Return type

[**models::GetFields200Response**](getFields_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field

> models::CreateField200Response update_field(collection, id, update_field_request)
Update a Field

Update an existing field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Unique identifier of the collection the item resides in. | [required] |
**id** | **String** | Unique identifier of the field. | [required] |
**update_field_request** | Option<[**UpdateFieldRequest**](UpdateFieldRequest.md)> |  |  |

### Return type

[**models::CreateField200Response**](createField_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

