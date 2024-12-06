# \ItemsEbooksApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_items_ebooks**](ItemsEbooksApi.md#create_items_ebooks) | **POST** /items/ebooks | Create an Item
[**delete_items_ebooks**](ItemsEbooksApi.md#delete_items_ebooks) | **DELETE** /items/ebooks | Delete Multiple Items
[**delete_single_items_ebooks**](ItemsEbooksApi.md#delete_single_items_ebooks) | **DELETE** /items/ebooks/{id} | Delete an Item
[**read_items_ebooks**](ItemsEbooksApi.md#read_items_ebooks) | **GET** /items/ebooks | List Items
[**read_single_items_ebooks**](ItemsEbooksApi.md#read_single_items_ebooks) | **GET** /items/ebooks/{id} | Retrieve an Item
[**update_items_ebooks**](ItemsEbooksApi.md#update_items_ebooks) | **PATCH** /items/ebooks | Update Multiple Items
[**update_single_items_ebooks**](ItemsEbooksApi.md#update_single_items_ebooks) | **PATCH** /items/ebooks/{id} | Update an Item



## create_items_ebooks

> models::CreateItemsEbooks200Response create_items_ebooks(meta, create_items_ebooks_request)
Create an Item

Create a new ebooks item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_items_ebooks_request** | Option<[**CreateItemsEbooksRequest**](CreateItemsEbooksRequest.md)> |  |  |

### Return type

[**models::CreateItemsEbooks200Response**](createItemsEbooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items_ebooks

> delete_items_ebooks()
Delete Multiple Items

Delete multiple existing ebooks items.

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


## delete_single_items_ebooks

> delete_single_items_ebooks(id)
Delete an Item

Delete an existing ebooks item.

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


## read_items_ebooks

> models::ReadItemsEbooks200Response read_items_ebooks(fields, limit, meta, offset, sort, filter, search)
List Items

List the ebooks items.

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

[**models::ReadItemsEbooks200Response**](readItemsEbooks_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_single_items_ebooks

> models::ReadSingleItemsEbooks200Response read_single_items_ebooks(id, fields, meta, version)
Retrieve an Item

Retrieve a single ebooks item by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**version** | Option<**String**> | Retrieve an item's state from a specific Content Version. The value corresponds to the \"key\" of the Content Version.  |  |

### Return type

[**models::ReadSingleItemsEbooks200Response**](readSingleItemsEbooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_items_ebooks

> models::UpdateItemsEbooks200Response update_items_ebooks(fields, limit, meta, offset, sort, filter, search, create_items_ebooks_request)
Update Multiple Items

Update multiple ebooks items at the same time.

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
**create_items_ebooks_request** | Option<[**CreateItemsEbooksRequest**](CreateItemsEbooksRequest.md)> |  |  |

### Return type

[**models::UpdateItemsEbooks200Response**](updateItemsEbooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_items_ebooks

> models::ReadSingleItemsEbooks200Response update_single_items_ebooks(id, fields, meta, items_ebooks)
Update an Item

Update an existing ebooks item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**items_ebooks** | Option<[**ItemsEbooks**](ItemsEbooks.md)> |  |  |

### Return type

[**models::ReadSingleItemsEbooks200Response**](readSingleItemsEbooks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

