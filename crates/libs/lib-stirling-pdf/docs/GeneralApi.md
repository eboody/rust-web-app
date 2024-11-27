# \GeneralApi

All URIs are relative to *https://spdf.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auto_split_pdf1**](GeneralApi.md#auto_split_pdf1) | **POST** /api/v1/general/split-by-size-or-count | Auto split PDF pages into separate documents based on size or count
[**crop_pdf**](GeneralApi.md#crop_pdf) | **POST** /api/v1/general/crop | Crops a PDF document
[**delete_pages**](GeneralApi.md#delete_pages) | **POST** /api/v1/general/remove-pages | Remove pages from a PDF file
[**merge_multiple_pages_into_one**](GeneralApi.md#merge_multiple_pages_into_one) | **POST** /api/v1/general/multi-page-layout | Merge multiple pages of a PDF document into a single page
[**merge_pdfs**](GeneralApi.md#merge_pdfs) | **POST** /api/v1/general/merge-pdfs | Merge multiple PDF files into one
[**overlay_pdfs**](GeneralApi.md#overlay_pdfs) | **POST** /api/v1/general/overlay-pdfs | Overlay PDF files in various modes
[**pdf_to_single_page**](GeneralApi.md#pdf_to_single_page) | **POST** /api/v1/general/pdf-to-single-page | Convert a multi-page PDF into a single long page PDF
[**rearrange_pages**](GeneralApi.md#rearrange_pages) | **POST** /api/v1/general/rearrange-pages | Rearrange pages in a PDF file
[**remove_images**](GeneralApi.md#remove_images) | **POST** /api/v1/general/remove-image-pdf | Remove images from file to reduce the file size.
[**rotate_pdf**](GeneralApi.md#rotate_pdf) | **POST** /api/v1/general/rotate-pdf | Rotate a PDF file
[**scale_pages**](GeneralApi.md#scale_pages) | **POST** /api/v1/general/scale-pages | Change the size of a PDF page/document
[**split_pdf**](GeneralApi.md#split_pdf) | **POST** /api/v1/general/split-pdf-by-sections | Split PDF pages into smaller sections
[**split_pdf1**](GeneralApi.md#split_pdf1) | **POST** /api/v1/general/split-pdf-by-chapters | Split PDFs by Chapters
[**split_pdf2**](GeneralApi.md#split_pdf2) | **POST** /api/v1/general/split-pages | Split a PDF file into separate documents



## auto_split_pdf1

> Vec<String> auto_split_pdf1(file_input, split_type, split_value)
Auto split PDF pages into separate documents based on size or count

split PDF into multiple paged documents based on size/count, ie if 20 pages and split into 5, it does 5 documents each 4 pages   if 10MB and each page is 1MB and you enter 2MB then 5 docs each 2MB (rounded so that it accepts 1.9MB but not 2.1MB) Input:PDF Output:ZIP-PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**split_type** | Option<**i32**> | Determines the type of split: 0 for size, 1 for page count, 2 for document count |  |[default to 0]
**split_value** | Option<**String**> | Value for split: size in MB (e.g., '10MB') or number of pages (e.g., '5') |  |[default to 10MB]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crop_pdf

> Vec<String> crop_pdf(file_input, x, y, width, height)
Crops a PDF document

This operation takes an input PDF file and crops it according to the given coordinates. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**x** | Option<**f64**> | The x-coordinate of the top-left corner of the crop area |  |
**y** | Option<**f64**> | The y-coordinate of the top-left corner of the crop area |  |
**width** | Option<**f64**> | The width of the crop area |  |
**height** | Option<**f64**> | The height of the crop area |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pages

> Vec<String> delete_pages(file_input, page_numbers)
Remove pages from a PDF file

This endpoint removes specified pages from a given PDF file. Users can provide a comma-separated list of page numbers or ranges to delete. Input:PDF Output:PDF Type:SISO

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


## merge_multiple_pages_into_one

> Vec<String> merge_multiple_pages_into_one(file_input, pages_per_sheet, add_border)
Merge multiple pages of a PDF document into a single page

This operation takes an input PDF file and the number of pages to merge into a single sheet in the output PDF file. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**pages_per_sheet** | Option<**i32**> | The number of pages to fit onto a single sheet in the output PDF. |  |
**add_border** | Option<**bool**> | Boolean for if you wish to add border around the pages |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_pdfs

> Vec<String> merge_pdfs(file_input, sort_type, remove_cert_sign)
Merge multiple PDF files into one

This endpoint merges multiple PDF files into a single PDF file. The merged file will contain all pages from the input files in the order they were provided. Input:PDF Output:PDF Type:MISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> | The input PDF files |  |
**sort_type** | Option<**String**> | The type of sorting to be applied on the input files before merging. |  |[default to orderProvided]
**remove_cert_sign** | Option<**bool**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## overlay_pdfs

> Vec<String> overlay_pdfs(overlay_mode, file_input, overlay_files, counts, overlay_position)
Overlay PDF files in various modes

Overlay PDF files onto a base PDF with different modes: Sequential, Interleaved, or Fixed Repeat. Input:PDF Output:PDF Type:MIMO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**overlay_mode** | **String** | The mode of overlaying: 'SequentialOverlay' for sequential application, 'InterleavedOverlay' for round-robin application, 'FixedRepeatOverlay' for fixed repetition based on provided counts | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**overlay_files** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> | An array of PDF files to be used as overlays on the base PDF. The order in these files is applied based on the selected mode. |  |
**counts** | Option<[**Vec<i32>**](i32.md)> | An array of integers specifying the number of times each corresponding overlay file should be applied in the 'FixedRepeatOverlay' mode. This should match the length of the overlayFiles array. |  |
**overlay_position** | Option<**i32**> | Overlay position 0 is Foregound, 1 is Background |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pdf_to_single_page

> Vec<String> pdf_to_single_page(file_input)
Convert a multi-page PDF into a single long page PDF

This endpoint converts a multi-page PDF document into a single paged PDF document. The width of the single page will be same as the input's width, but the height will be the sum of all the pages' heights. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rearrange_pages

> Vec<String> rearrange_pages(file_input, page_numbers, custom_mode)
Rearrange pages in a PDF file

This endpoint rearranges pages in a given PDF file based on the specified page order or custom mode. Users can provide a page order as a comma-separated list of page numbers or page ranges, or a custom mode. Input:PDF Output:PDF

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_numbers** | Option<**String**> | The pages to select, Supports ranges (e.g., '1,3,5-9'), or 'all' or functions in the format 'an+b' where 'a' is the multiplier of the page number 'n', and 'b' is a constant (e.g., '2n+1', '3n', '6n-5')\\\" |  |
**custom_mode** | Option<**String**> | The custom mode for page rearrangement. Valid values are: REVERSE_ORDER: Reverses the order of all pages. DUPLEX_SORT: Sorts pages as if all fronts were scanned then all backs in reverse (1, n, 2, n-1, ...). BOOKLET_SORT: Arranges pages for booklet printing (last, first, second, second last, ...). ODD_EVEN_SPLIT: Splits and arranges pages into odd and even numbered pages. ODD_EVEN_MERGE: Merges pages and organises them alternately into odd and even pages. REMOVE_FIRST: Removes the first page. REMOVE_LAST: Removes the last page. REMOVE_FIRST_AND_LAST: Removes both the first and the last pages.  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_images

> Vec<String> remove_images(file_input)
Remove images from file to reduce the file size.

This endpoint remove images from file to reduce the file size.Input:PDF Output:PDF Type:MISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_pdf

> Vec<String> rotate_pdf(file_input, angle)
Rotate a PDF file

This endpoint rotates a given PDF file by a specified angle. The angle must be a multiple of 90. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**angle** | Option<**i32**> | The angle by which to rotate the PDF file. This should be a multiple of 90. |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_pages

> Vec<String> scale_pages(file_input, page_size, scale_factor)
Change the size of a PDF page/document

This operation takes an input PDF file and the size to scale the pages to in the output PDF file. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_size** | Option<**String**> | The scale of pages in the output PDF. Acceptable values are A0-A6, LETTER, LEGAL, KEEP. |  |
**scale_factor** | Option<**f32**> | The scale of the content on the pages of the output PDF. Acceptable values are floats. |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## split_pdf

> Vec<String> split_pdf(file_input, horizontal_divisions, vertical_divisions, merge)
Split PDF pages into smaller sections

Split each page of a PDF into smaller sections based on the user's choice (halves, thirds, quarters, etc.), both vertically and horizontally. Input:PDF Output:ZIP-PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**horizontal_divisions** | Option<**i32**> | Number of horizontal divisions for each PDF page |  |
**vertical_divisions** | Option<**i32**> | Number of vertical divisions for each PDF page |  |
**merge** | Option<**bool**> | Merge the split documents into a single PDF |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## split_pdf1

> Vec<String> split_pdf1(file_input, include_metadata, allow_duplicates, bookmark_level)
Split PDFs by Chapters

Splits a PDF into chapters and returns a ZIP file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**include_metadata** | Option<**bool**> | Whether to include Metadata or not |  |
**allow_duplicates** | Option<**bool**> | Whether to allow duplicates or not |  |
**bookmark_level** | Option<**i32**> | Maximum bookmark level required |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## split_pdf2

> Vec<String> split_pdf2(file_input, page_numbers)
Split a PDF file into separate documents

This endpoint splits a given PDF file into separate documents based on the specified page numbers or ranges. Users can specify pages using individual numbers, ranges, or 'all' for every page. Input:PDF Output:PDF Type:SIMO

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

