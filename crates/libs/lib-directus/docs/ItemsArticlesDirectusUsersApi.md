# \ItemsArticlesDirectusUsersApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#create_items_articles_directus_users) | **POST** /items/articles_directus_users | Create an Item
[**delete_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#delete_items_articles_directus_users) | **DELETE** /items/articles_directus_users | Delete Multiple Items
[**delete_single_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#delete_single_items_articles_directus_users) | **DELETE** /items/articles_directus_users/{id} | Delete an Item
[**read_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#read_items_articles_directus_users) | **GET** /items/articles_directus_users | List Items
[**read_single_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#read_single_items_articles_directus_users) | **GET** /items/articles_directus_users/{id} | Retrieve an Item
[**update_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#update_items_articles_directus_users) | **PATCH** /items/articles_directus_users | Update Multiple Items
[**update_single_items_articles_directus_users**](ItemsArticlesDirectusUsersApi.md#update_single_items_articles_directus_users) | **PATCH** /items/articles_directus_users/{id} | Update an Item



## create_items_articles_directus_users

> models::CreateItemsArticlesDirectusUsers200Response create_items_articles_directus_users(meta, create_items_articles_directus_users_request)
Create an Item

Create a new articles_directus_users item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_items_articles_directus_users_request** | Option<[**CreateItemsArticlesDirectusUsersRequest**](CreateItemsArticlesDirectusUsersRequest.md)> |  |  |

### Return type

[**models::CreateItemsArticlesDirectusUsers200Response**](createItemsArticlesDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items_articles_directus_users

> delete_items_articles_directus_users()
Delete Multiple Items

Delete multiple existing articles_directus_users items.

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


## delete_single_items_articles_directus_users

> delete_single_items_articles_directus_users(id)
Delete an Item

Delete an existing articles_directus_users item.

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


## read_items_articles_directus_users

> models::ReadItemsArticlesDirectusUsers200Response read_items_articles_directus_users(fields, limit, meta, offset, sort, filter, search)
List Items

List the articles_directus_users items.

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

[**models::ReadItemsArticlesDirectusUsers200Response**](readItemsArticlesDirectusUsers_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_single_items_articles_directus_users

> models::ReadSingleItemsArticlesDirectusUsers200Response read_single_items_articles_directus_users(id, fields, meta, version)
Retrieve an Item

Retrieve a single articles_directus_users item by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**version** | Option<**String**> | Retrieve an item's state from a specific Content Version. The value corresponds to the \"key\" of the Content Version.  |  |

### Return type

[**models::ReadSingleItemsArticlesDirectusUsers200Response**](readSingleItemsArticlesDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_items_articles_directus_users

> models::UpdateItemsArticlesDirectusUsers200Response update_items_articles_directus_users(fields, limit, meta, offset, sort, filter, search, create_items_articles_directus_users_request)
Update Multiple Items

Update multiple articles_directus_users items at the same time.

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
**create_items_articles_directus_users_request** | Option<[**CreateItemsArticlesDirectusUsersRequest**](CreateItemsArticlesDirectusUsersRequest.md)> |  |  |

### Return type

[**models::UpdateItemsArticlesDirectusUsers200Response**](updateItemsArticlesDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_items_articles_directus_users

> models::ReadSingleItemsArticlesDirectusUsers200Response update_single_items_articles_directus_users(id, fields, meta, items_articles_directus_users)
Update an Item

Update an existing articles_directus_users item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**items_articles_directus_users** | Option<[**ItemsArticlesDirectusUsers**](ItemsArticlesDirectusUsers.md)> |  |  |

### Return type

[**models::ReadSingleItemsArticlesDirectusUsers200Response**](readSingleItemsArticlesDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

