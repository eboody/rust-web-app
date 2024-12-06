# \ItemsEbooksDirectusUsersApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#create_items_ebooks_directus_users) | **POST** /items/ebooks_directus_users | Create an Item
[**delete_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#delete_items_ebooks_directus_users) | **DELETE** /items/ebooks_directus_users | Delete Multiple Items
[**delete_single_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#delete_single_items_ebooks_directus_users) | **DELETE** /items/ebooks_directus_users/{id} | Delete an Item
[**read_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#read_items_ebooks_directus_users) | **GET** /items/ebooks_directus_users | List Items
[**read_single_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#read_single_items_ebooks_directus_users) | **GET** /items/ebooks_directus_users/{id} | Retrieve an Item
[**update_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#update_items_ebooks_directus_users) | **PATCH** /items/ebooks_directus_users | Update Multiple Items
[**update_single_items_ebooks_directus_users**](ItemsEbooksDirectusUsersApi.md#update_single_items_ebooks_directus_users) | **PATCH** /items/ebooks_directus_users/{id} | Update an Item



## create_items_ebooks_directus_users

> models::CreateItemsEbooksDirectusUsers200Response create_items_ebooks_directus_users(meta, create_items_ebooks_directus_users_request)
Create an Item

Create a new ebooks_directus_users item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_items_ebooks_directus_users_request** | Option<[**CreateItemsEbooksDirectusUsersRequest**](CreateItemsEbooksDirectusUsersRequest.md)> |  |  |

### Return type

[**models::CreateItemsEbooksDirectusUsers200Response**](createItemsEbooksDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items_ebooks_directus_users

> delete_items_ebooks_directus_users()
Delete Multiple Items

Delete multiple existing ebooks_directus_users items.

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


## delete_single_items_ebooks_directus_users

> delete_single_items_ebooks_directus_users(id)
Delete an Item

Delete an existing ebooks_directus_users item.

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


## read_items_ebooks_directus_users

> models::ReadItemsEbooksDirectusUsers200Response read_items_ebooks_directus_users(fields, limit, meta, offset, sort, filter, search)
List Items

List the ebooks_directus_users items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |

### Return type

[**models::ReadItemsEbooksDirectusUsers200Response**](readItemsEbooksDirectusUsers_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_single_items_ebooks_directus_users

> models::ReadSingleItemsEbooksDirectusUsers200Response read_single_items_ebooks_directus_users(id, fields, meta, version)
Retrieve an Item

Retrieve a single ebooks_directus_users item by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**version** | Option<**String**> | Retrieve an item's state from a specific Content Version. The value corresponds to the \"key\" of the Content Version.  |  |

### Return type

[**models::ReadSingleItemsEbooksDirectusUsers200Response**](readSingleItemsEbooksDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_items_ebooks_directus_users

> models::UpdateItemsEbooksDirectusUsers200Response update_items_ebooks_directus_users(fields, limit, meta, offset, sort, filter, search, create_items_ebooks_directus_users_request)
Update Multiple Items

Update multiple ebooks_directus_users items at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**limit** | Option<**i32**> | A limit on the number of objects that are returned. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**offset** | Option<**i32**> | How many items to skip when fetching data. |  |
**sort** | Option<[**Vec<String>**](String.md)> | How to sort the returned items. `sort` is a CSV of fields used to sort the fetched items. Sorting defaults to ascending (ASC) order but a minus sign (` - `) can be used to reverse this to descending (DESC) order. Fields are prioritized by their order in the CSV. You can also use a ` ? ` to sort randomly.  |  |
**filter** | Option<[**json::Value**](.md)> | Select items in collection by given conditions. |  |
**search** | Option<**String**> | Filter by items that contain the given search query in one of their fields. |  |
**create_items_ebooks_directus_users_request** | Option<[**CreateItemsEbooksDirectusUsersRequest**](CreateItemsEbooksDirectusUsersRequest.md)> |  |  |

### Return type

[**models::UpdateItemsEbooksDirectusUsers200Response**](updateItemsEbooksDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_items_ebooks_directus_users

> models::ReadSingleItemsEbooksDirectusUsers200Response update_single_items_ebooks_directus_users(id, fields, meta, items_ebooks_directus_users)
Update an Item

Update an existing ebooks_directus_users item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**items_ebooks_directus_users** | Option<[**ItemsEbooksDirectusUsers**](ItemsEbooksDirectusUsers.md)> |  |  |

### Return type

[**models::ReadSingleItemsEbooksDirectusUsers200Response**](readSingleItemsEbooksDirectusUsers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

