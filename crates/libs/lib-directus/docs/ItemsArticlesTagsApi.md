# \ItemsArticlesTagsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_items_articles_tags**](ItemsArticlesTagsApi.md#create_items_articles_tags) | **POST** /items/articles_tags | Create an Item
[**delete_items_articles_tags**](ItemsArticlesTagsApi.md#delete_items_articles_tags) | **DELETE** /items/articles_tags | Delete Multiple Items
[**delete_single_items_articles_tags**](ItemsArticlesTagsApi.md#delete_single_items_articles_tags) | **DELETE** /items/articles_tags/{id} | Delete an Item
[**read_items_articles_tags**](ItemsArticlesTagsApi.md#read_items_articles_tags) | **GET** /items/articles_tags | List Items
[**read_single_items_articles_tags**](ItemsArticlesTagsApi.md#read_single_items_articles_tags) | **GET** /items/articles_tags/{id} | Retrieve an Item
[**update_items_articles_tags**](ItemsArticlesTagsApi.md#update_items_articles_tags) | **PATCH** /items/articles_tags | Update Multiple Items
[**update_single_items_articles_tags**](ItemsArticlesTagsApi.md#update_single_items_articles_tags) | **PATCH** /items/articles_tags/{id} | Update an Item



## create_items_articles_tags

> models::CreateItemsArticlesTags200Response create_items_articles_tags(meta, create_items_articles_tags_request)
Create an Item

Create a new articles_tags item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_items_articles_tags_request** | Option<[**CreateItemsArticlesTagsRequest**](CreateItemsArticlesTagsRequest.md)> |  |  |

### Return type

[**models::CreateItemsArticlesTags200Response**](createItemsArticlesTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items_articles_tags

> delete_items_articles_tags()
Delete Multiple Items

Delete multiple existing articles_tags items.

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


## delete_single_items_articles_tags

> delete_single_items_articles_tags(id)
Delete an Item

Delete an existing articles_tags item.

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


## read_items_articles_tags

> models::ReadItemsArticlesTags200Response read_items_articles_tags(fields, limit, meta, offset, sort, filter, search)
List Items

List the articles_tags items.

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

[**models::ReadItemsArticlesTags200Response**](readItemsArticlesTags_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_single_items_articles_tags

> models::ReadSingleItemsArticlesTags200Response read_single_items_articles_tags(id, fields, meta, version)
Retrieve an Item

Retrieve a single articles_tags item by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**version** | Option<**String**> | Retrieve an item's state from a specific Content Version. The value corresponds to the \"key\" of the Content Version.  |  |

### Return type

[**models::ReadSingleItemsArticlesTags200Response**](readSingleItemsArticlesTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_items_articles_tags

> models::UpdateItemsArticlesTags200Response update_items_articles_tags(fields, limit, meta, offset, sort, filter, search, create_items_articles_tags_request)
Update Multiple Items

Update multiple articles_tags items at the same time.

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
**create_items_articles_tags_request** | Option<[**CreateItemsArticlesTagsRequest**](CreateItemsArticlesTagsRequest.md)> |  |  |

### Return type

[**models::UpdateItemsArticlesTags200Response**](updateItemsArticlesTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_items_articles_tags

> models::ReadSingleItemsArticlesTags200Response update_single_items_articles_tags(id, fields, meta, items_articles_tags)
Update an Item

Update an existing articles_tags item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**items_articles_tags** | Option<[**ItemsArticlesTags**](ItemsArticlesTags.md)> |  |  |

### Return type

[**models::ReadSingleItemsArticlesTags200Response**](readSingleItemsArticlesTags_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

