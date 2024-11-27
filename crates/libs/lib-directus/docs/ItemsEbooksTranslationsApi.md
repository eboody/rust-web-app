# \ItemsEbooksTranslationsApi

All URIs are relative to *http://directus.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#create_items_ebooks_translations) | **POST** /items/ebooks_translations | Create an Item
[**delete_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#delete_items_ebooks_translations) | **DELETE** /items/ebooks_translations | Delete Multiple Items
[**delete_single_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#delete_single_items_ebooks_translations) | **DELETE** /items/ebooks_translations/{id} | Delete an Item
[**read_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#read_items_ebooks_translations) | **GET** /items/ebooks_translations | List Items
[**read_single_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#read_single_items_ebooks_translations) | **GET** /items/ebooks_translations/{id} | Retrieve an Item
[**update_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#update_items_ebooks_translations) | **PATCH** /items/ebooks_translations | Update Multiple Items
[**update_single_items_ebooks_translations**](ItemsEbooksTranslationsApi.md#update_single_items_ebooks_translations) | **PATCH** /items/ebooks_translations/{id} | Update an Item



## create_items_ebooks_translations

> models::CreateItemsEbooksTranslations200Response create_items_ebooks_translations(meta, create_items_ebooks_translations_request)
Create an Item

Create a new ebooks_translations item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meta** | Option<**String**> | What metadata to return in the response. |  |
**create_items_ebooks_translations_request** | Option<[**CreateItemsEbooksTranslationsRequest**](CreateItemsEbooksTranslationsRequest.md)> |  |  |

### Return type

[**models::CreateItemsEbooksTranslations200Response**](createItemsEbooksTranslations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items_ebooks_translations

> delete_items_ebooks_translations()
Delete Multiple Items

Delete multiple existing ebooks_translations items.

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


## delete_single_items_ebooks_translations

> delete_single_items_ebooks_translations(id)
Delete an Item

Delete an existing ebooks_translations item.

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


## read_items_ebooks_translations

> models::ReadItemsEbooksTranslations200Response read_items_ebooks_translations(fields, limit, meta, offset, sort, filter, search)
List Items

List the ebooks_translations items.

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

[**models::ReadItemsEbooksTranslations200Response**](readItemsEbooksTranslations_200_response.md)

### Authorization

[Auth](../README.md#Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_single_items_ebooks_translations

> models::ReadSingleItemsEbooksTranslations200Response read_single_items_ebooks_translations(id, fields, meta, version)
Retrieve an Item

Retrieve a single ebooks_translations item by unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**version** | Option<**String**> | Retrieve an item's state from a specific Content Version. The value corresponds to the \"key\" of the Content Version.  |  |

### Return type

[**models::ReadSingleItemsEbooksTranslations200Response**](readSingleItemsEbooksTranslations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_items_ebooks_translations

> models::UpdateItemsEbooksTranslations200Response update_items_ebooks_translations(fields, limit, meta, offset, sort, filter, search, create_items_ebooks_translations_request)
Update Multiple Items

Update multiple ebooks_translations items at the same time.

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
**create_items_ebooks_translations_request** | Option<[**CreateItemsEbooksTranslationsRequest**](CreateItemsEbooksTranslationsRequest.md)> |  |  |

### Return type

[**models::UpdateItemsEbooksTranslations200Response**](updateItemsEbooksTranslations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_items_ebooks_translations

> models::ReadSingleItemsEbooksTranslations200Response update_single_items_ebooks_translations(id, fields, meta, items_ebooks_translations)
Update an Item

Update an existing ebooks_translations item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**ReadSingleItemsLanguagesIdParameter**](.md) | Index of the item. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Control what fields are being returned in the object. |  |
**meta** | Option<**String**> | What metadata to return in the response. |  |
**items_ebooks_translations** | Option<[**ItemsEbooksTranslations**](ItemsEbooksTranslations.md)> |  |  |

### Return type

[**models::ReadSingleItemsEbooksTranslations200Response**](readSingleItemsEbooksTranslations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

