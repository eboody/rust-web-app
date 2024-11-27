# \ItemsEbooksTagsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_items_ebooks_tags**](ItemsEbooksTagsApi.md#create_items_ebooks_tags) | **POST** /items/ebooks_tags | Create an Item
[**delete_items_ebooks_tags**](ItemsEbooksTagsApi.md#delete_items_ebooks_tags) | **DELETE** /items/ebooks_tags | Delete Multiple Items
[**delete_single_items_ebooks_tags**](ItemsEbooksTagsApi.md#delete_single_items_ebooks_tags) | **DELETE** /items/ebooks_tags/{id} | Delete an Item
[**read_items_ebooks_tags**](ItemsEbooksTagsApi.md#read_items_ebooks_tags) | **GET** /items/ebooks_tags | List Items
[**read_single_items_ebooks_tags**](ItemsEbooksTagsApi.md#read_single_items_ebooks_tags) | **GET** /items/ebooks_tags/{id} | Retrieve an Item
[**update_items_ebooks_tags**](ItemsEbooksTagsApi.md#update_items_ebooks_tags) | **PATCH** /items/ebooks_tags | Update Multiple Items
[**update_single_items_ebooks_tags**](ItemsEbooksTagsApi.md#update_single_items_ebooks_tags) | **PATCH** /items/ebooks_tags/{id} | Update an Item



## create_items_ebooks_tags

> models::CreateItemsEbooksTags200Response create_items_ebooks_tags(meta, create_items_ebooks_tags_request)
Create an Item

Create a new ebooks_tags item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_items_ebooks_tags_request** | Option<[**CreateItemsEbooksTagsRequest**](CreateItemsEbooksTagsRequest.md)> |  |  |

### Return type

[**models::CreateItemsEbooksTags200Response**](createItemsEbooksTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items_ebooks_tags

> delete_items_ebooks_tags()
Delete Multiple Items

Delete multiple existing ebooks_tags items.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_single_items_ebooks_tags

> delete_single_items_ebooks_tags(id)
Delete an Item

Delete an existing ebooks_tags item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_items_ebooks_tags

> models::ReadItemsEbooksTags200Response read_items_ebooks_tags(fields, limit, meta, offset, sort, filter, search)
List Items

List the ebooks_tags items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**serde_json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |

### Return type

[**models::ReadItemsEbooksTags200Response**](readItemsEbooksTags_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_single_items_ebooks_tags

> models::ReadSingleItemsEbooksTags200Response read_single_items_ebooks_tags(id, fields, meta, version)
Retrieve an Item

Retrieve a single ebooks_tags item by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**version** | Option<**String**> | Retrieve an item's state from a specific Content Version. The value corresponds to the \"key\" of the Content Version.  |  |

### Return type

[**models::ReadSingleItemsEbooksTags200Response**](readSingleItemsEbooksTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_items_ebooks_tags

> models::UpdateItemsEbooksTags200Response update_items_ebooks_tags(fields, limit, meta, offset, sort, filter, search, create_items_ebooks_tags_request)
Update Multiple Items

Update multiple ebooks_tags items at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**serde_json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**create_items_ebooks_tags_request** | Option<[**CreateItemsEbooksTagsRequest**](CreateItemsEbooksTagsRequest.md)> |  |  |

### Return type

[**models::UpdateItemsEbooksTags200Response**](updateItemsEbooksTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_items_ebooks_tags

> models::ReadSingleItemsEbooksTags200Response update_single_items_ebooks_tags(id, fields, meta, items_ebooks_tags)
Update an Item

Update an existing ebooks_tags item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**items_ebooks_tags** | Option<[**ItemsEbooksTags**](ItemsEbooksTags.md)> |  |  |

### Return type

[**models::ReadSingleItemsEbooksTags200Response**](readSingleItemsEbooksTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
