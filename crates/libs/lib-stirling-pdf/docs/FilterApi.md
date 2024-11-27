# \FilterApi

All URIs are relative to *https://spdf.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contains_image**](FilterApi.md#contains_image) | **POST** /api/v1/filter/filter-contains-image | Checks if a PDF contains an image
[**contains_text**](FilterApi.md#contains_text) | **POST** /api/v1/filter/filter-contains-text | Checks if a PDF contains set text, returns true if does
[**file_size**](FilterApi.md#file_size) | **POST** /api/v1/filter/filter-file-size | Checks if a PDF is a set file size
[**page_count**](FilterApi.md#page_count) | **POST** /api/v1/filter/filter-page-count | Checks if a PDF is greater, less or equal to a setPageCount
[**page_rotation**](FilterApi.md#page_rotation) | **POST** /api/v1/filter/filter-page-rotation | Checks if a PDF is of a certain rotation
[**page_size**](FilterApi.md#page_size) | **POST** /api/v1/filter/filter-page-size | Checks if a PDF is of a certain size



## contains_image

> Vec<String> contains_image(file_input, page_numbers)
Checks if a PDF contains an image

Input:PDF Output:Boolean Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_numbers** | Option<**String**> | The pages to select, Supports ranges (e.g., '1,3,5-9'), or 'all' or functions in the format 'an+b' where 'a' is the multiplier of the page number 'n', and 'b' is a constant (e.g., '2n+1', '3n', '6n-5')\\\" |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contains_text

> Vec<String> contains_text(text, file_input, page_numbers)
Checks if a PDF contains set text, returns true if does

Input:PDF Output:Boolean Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | **String** | The text to check for | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_numbers** | Option<**String**> | The pages to select, Supports ranges (e.g., '1,3,5-9'), or 'all' or functions in the format 'an+b' where 'a' is the multiplier of the page number 'n', and 'b' is a constant (e.g., '2n+1', '3n', '6n-5')\\\" |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_size

> Vec<String> file_size(file_size, file_input, comparator)
Checks if a PDF is a set file size

Input:PDF Output:Boolean Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_size** | **String** | File Size | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**comparator** | Option<**String**> | The comparison type, accepts Greater, Equal, Less than |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_count

> Vec<String> page_count(file_input, comparator, page_count)
Checks if a PDF is greater, less or equal to a setPageCount

Input:PDF Output:Boolean Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**comparator** | Option<**String**> | The comparison type, accepts Greater, Equal, Less than |  |
**page_count** | Option<**String**> | Count |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_rotation

> Vec<String> page_rotation(rotation, file_input, comparator)
Checks if a PDF is of a certain rotation

Input:PDF Output:Boolean Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rotation** | **i32** | Rotation in degrees | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**comparator** | Option<**String**> | The comparison type, accepts Greater, Equal, Less than |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_size

> Vec<String> page_size(standard_page_size, file_input, comparator)
Checks if a PDF is of a certain size

Input:PDF Output:Boolean Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**standard_page_size** | **String** | Standard Page Size | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**comparator** | Option<**String**> | The comparison type, accepts Greater, Equal, Less than |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

