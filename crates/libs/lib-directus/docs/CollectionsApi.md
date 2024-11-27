# \CollectionsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_collection**](CollectionsApi.md#create_collection) | **POST** /collections | Create a Collection
[**delete_collection**](CollectionsApi.md#delete_collection) | **DELETE** /collections/{id} | Delete a Collection
[**get_collection**](CollectionsApi.md#get_collection) | **GET** /collections/{id} | Retrieve a Collection
[**get_collections**](CollectionsApi.md#get_collections) | **GET** /collections | List Collections
[**update_collection**](CollectionsApi.md#update_collection) | **PATCH** /collections/{id} | Update a Collection



## create_collection

> models::CreateCollection200Response create_collection(meta, create_collection_request)
Create a Collection

Create a new collection in Directus.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_collection_request** | Option<[**CreateCollectionRequest**](CreateCollectionRequest.md)> |  |  |

### Return type

[**models::CreateCollection200Response**](createCollection_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> delete_collection(id)
Delete a Collection

Delete an existing collection. Warning: This will delete the whole collection, including the items within. Proceed with caution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the collection. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection

> models::CreateCollection200Response get_collection(id, meta)
Retrieve a Collection

Retrieves the details of a single collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the collection. | [required] |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::CreateCollection200Response**](createCollection_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collections

> models::GetCollections200Response get_collections(offset, meta)
List Collections

Returns a list of the collections available in the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |

### Return type

[**models::GetCollections200Response**](getCollections_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_collection

> models::CreateCollection200Response update_collection(id, meta, update_collection_request)
Update a Collection

Update an existing collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the collection. | [required] |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**update_collection_request** | Option<[**UpdateCollectionRequest**](UpdateCollectionRequest.md)> |  |  |

### Return type

[**models::CreateCollection200Response**](createCollection_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

