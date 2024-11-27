# \MiscApi

All URIs are relative to *https://spdf.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_page_numbers**](MiscApi.md#add_page_numbers) | **POST** /api/v1/misc/add-page-numbers | Add page numbers to a PDF document
[**add_stamp**](MiscApi.md#add_stamp) | **POST** /api/v1/misc/add-stamp | Add stamp to a PDF file
[**auto_split_pdf**](MiscApi.md#auto_split_pdf) | **POST** /api/v1/misc/auto-split-pdf | Auto split PDF pages into separate documents
[**extract_header**](MiscApi.md#extract_header) | **POST** /api/v1/misc/show-javascript | Grabs all JS from a PDF and returns a single JS file with all code
[**extract_header1**](MiscApi.md#extract_header1) | **POST** /api/v1/misc/auto-rename | Extract header from PDF file
[**extract_image_scans**](MiscApi.md#extract_image_scans) | **POST** /api/v1/misc/extract-image-scans | Extract image scans from an input file
[**extract_images**](MiscApi.md#extract_images) | **POST** /api/v1/misc/extract-images | Extract images from a PDF file
[**flatten**](MiscApi.md#flatten) | **POST** /api/v1/misc/flatten | Flatten PDF form fields or full page
[**metadata**](MiscApi.md#metadata) | **POST** /api/v1/misc/update-metadata | Update metadata of a PDF file
[**optimize_pdf**](MiscApi.md#optimize_pdf) | **POST** /api/v1/misc/compress-pdf | Optimize PDF file
[**overlay_image**](MiscApi.md#overlay_image) | **POST** /api/v1/misc/add-image | Overlay image onto a PDF file
[**process_pdf_with_ocr**](MiscApi.md#process_pdf_with_ocr) | **POST** /api/v1/misc/ocr-pdf | Process a PDF file with OCR
[**remove_blank_pages**](MiscApi.md#remove_blank_pages) | **POST** /api/v1/misc/remove-blanks | Remove blank pages from a PDF file
[**repair_pdf**](MiscApi.md#repair_pdf) | **POST** /api/v1/misc/repair | Repair a PDF file
[**replace_and_invert_color**](MiscApi.md#replace_and_invert_color) | **POST** /api/v1/misc/replace-invert-pdf | Replace-Invert Color PDF



## add_page_numbers

> Vec<String> add_page_numbers(file_input, page_numbers, custom_margin, font_size, font_type, position, starting_number, pages_to_number, custom_text)
Add page numbers to a PDF document

This operation takes an input PDF file and adds page numbers to it. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_numbers** | Option<**String**> | The pages to select, Supports ranges (e.g., '1,3,5-9'), or 'all' or functions in the format 'an+b' where 'a' is the multiplier of the page number 'n', and 'b' is a constant (e.g., '2n+1', '3n', '6n-5')\\\" |  |
**custom_margin** | Option<**String**> | Custom margin: small/medium/large |  |
**font_size** | Option<**f32**> |  |  |
**font_type** | Option<**String**> |  |  |
**position** | Option<**i32**> | Position: 1 of 9 positions |  |
**starting_number** | Option<**i32**> | Starting number |  |
**pages_to_number** | Option<**String**> | Which pages to number, default all |  |
**custom_text** | Option<**String**> | Custom text: defaults to just number but can have things like \\\"Page {n} of {p}\\\" |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_stamp

> Vec<String> add_stamp(stamp_type, file_input, page_numbers, stamp_text, stamp_image, alphabet, font_size, rotation, opacity, position, override_x, override_y, custom_margin, custom_color)
Add stamp to a PDF file

This endpoint adds a stamp to a given PDF file. Users can specify the stamp type (text or image), rotation, opacity, width spacer, and height spacer. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_type** | **String** | The stamp type (text or image) | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_numbers** | Option<**String**> | The pages to select, Supports ranges (e.g., '1,3,5-9'), or 'all' or functions in the format 'an+b' where 'a' is the multiplier of the page number 'n', and 'b' is a constant (e.g., '2n+1', '3n', '6n-5')\\\" |  |
**stamp_text** | Option<**String**> | The stamp text |  |
**stamp_image** | Option<**std::path::PathBuf**> |  |  |
**alphabet** | Option<**String**> | The selected alphabet |  |[default to roman]
**font_size** | Option<**f32**> | The font size of the stamp text |  |
**rotation** | Option<**f32**> | The rotation of the stamp in degrees |  |
**opacity** | Option<**f32**> | The opacity of the stamp (0.0 - 1.0) |  |
**position** | Option<**i32**> | Position for stamp placement based on a 1-9 grid (1: bottom-left, 2: bottom-center, ..., 9: top-right) |  |
**override_x** | Option<**f32**> | Override X coordinate for stamp placement. If set, it will override the position-based calculation. Negative value means no override. |  |
**override_y** | Option<**f32**> | Override Y coordinate for stamp placement. If set, it will override the position-based calculation. Negative value means no override. |  |
**custom_margin** | Option<**String**> | Specifies the margin size for the stamp. |  |[default to medium]
**custom_color** | Option<**String**> | The color for stamp |  |[default to #d3d3d3]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auto_split_pdf

> Vec<String> auto_split_pdf(file_input, duplex_mode)
Auto split PDF pages into separate documents

This endpoint accepts a PDF file, scans each page for a specific QR code, and splits the document at the QR code boundaries. The output is a zip file containing each separate PDF document. Input:PDF Output:ZIP-PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**duplex_mode** | Option<**bool**> | Flag indicating if the duplex mode is active, where the page after the divider also gets removed. |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extract_header

> Vec<String> extract_header(file_input)
Grabs all JS from a PDF and returns a single JS file with all code

desc. Input:PDF Output:JS Type:SISO

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


## extract_header1

> Vec<String> extract_header1(file_input, use_first_text_as_fallback)
Extract header from PDF file

This endpoint accepts a PDF file and attempts to extract its title or header based on heuristics. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**use_first_text_as_fallback** | Option<**bool**> | Flag indicating whether to use the first text as a fallback if no suitable title is found. Defaults to false. |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extract_image_scans

> Vec<String> extract_image_scans(file_input, angle_threshold, tolerance, min_area, min_contour_area, border_size)
Extract image scans from an input file

This endpoint extracts image scans from a given file based on certain parameters. Users can specify angle threshold, tolerance, minimum area, minimum contour area, and border size. Input:PDF Output:IMAGE/ZIP Type:SIMO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | **std::path::PathBuf** |  | [required] |
**angle_threshold** | Option<**i32**> | The angle threshold for the image scan extraction |  |[default to 5]
**tolerance** | Option<**i32**> | The tolerance for the image scan extraction |  |[default to 20]
**min_area** | Option<**i32**> | The minimum area for the image scan extraction |  |[default to 8000]
**min_contour_area** | Option<**i32**> | The minimum contour area for the image scan extraction |  |[default to 500]
**border_size** | Option<**i32**> | The border size for the image scan extraction |  |[default to 1]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extract_images

> Vec<String> extract_images(file_input, format, allow_duplicates)
Extract images from a PDF file

This endpoint extracts images from a given PDF file and returns them in a zip file. Users can specify the output image format. Input: PDF Output: IMAGE/ZIP Type: SIMO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**format** | Option<**String**> | The output image format e.g., 'png', 'jpeg', or 'gif' |  |
**allow_duplicates** | Option<**bool**> | Boolean to enable/disable the saving of duplicate images, true to enable duplicates |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flatten

> Vec<String> flatten(file_input, flatten_only_forms)
Flatten PDF form fields or full page

Flattening just PDF form fields or converting each page to images to make text unselectable. Input: PDF, Output: PDF. Type: SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**flatten_only_forms** | Option<**bool**> | True to flatten only the forms, false to flatten full PDF (Convert page to image) |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata

> Vec<String> metadata(file_input, delete_all, author, creation_date, creator, keywords, modification_date, producer, subject, title, trapped, all_request_params)
Update metadata of a PDF file

This endpoint allows you to update the metadata of a given PDF file. You can add, modify, or delete standard and custom metadata fields. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**delete_all** | Option<**bool**> | Delete all metadata if set to true |  |
**author** | Option<**String**> | The author of the document |  |
**creation_date** | Option<**String**> | The creation date of the document (format: yyyy/MM/dd HH:mm:ss) |  |
**creator** | Option<**String**> | The creator of the document |  |
**keywords** | Option<**String**> | The keywords for the document |  |
**modification_date** | Option<**String**> | The modification date of the document (format: yyyy/MM/dd HH:mm:ss) |  |
**producer** | Option<**String**> | The producer of the document |  |
**subject** | Option<**String**> | The subject of the document |  |
**title** | Option<**String**> | The title of the document |  |
**trapped** | Option<**String**> | The trapped status of the document |  |
**all_request_params** | Option<[**std::collections::HashMap<String, String>**](std::collections::HashMap.md)> | Map list of key and value of custom parameters. Note these must start with customKey and customValue if they are non-standard |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## optimize_pdf

> Vec<String> optimize_pdf(file_input, optimize_level, expected_output_size)
Optimize PDF file

This endpoint accepts a PDF file and optimizes it based on the provided parameters. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**optimize_level** | Option<**i32**> | The level of optimization to apply to the PDF file. Higher values indicate greater compression but may reduce quality. |  |
**expected_output_size** | Option<**String**> | The expected output size, e.g. '100MB', '25KB', etc. |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## overlay_image

> Vec<String> overlay_image(file_input, image_file, x, y, every_page)
Overlay image onto a PDF file

This endpoint overlays an image onto a PDF file at the specified coordinates. The image can be overlaid on every page of the PDF if specified.  Input:PDF/IMAGE Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**image_file** | Option<**std::path::PathBuf**> |  |  |
**x** | Option<**f32**> | The x-coordinate at which to place the top-left corner of the image. |  |
**y** | Option<**f32**> | The y-coordinate at which to place the top-left corner of the image. |  |
**every_page** | Option<**bool**> | Whether to overlay the image onto every page of the PDF. |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_pdf_with_ocr

> Vec<String> process_pdf_with_ocr(file_input, languages, sidecar, deskew, clean, clean_final, ocr_type, ocr_render_type, remove_images_after)
Process a PDF file with OCR

This endpoint processes a PDF file using OCR (Optical Character Recognition). Users can specify languages, sidecar, deskew, clean, cleanFinal, ocrType, ocrRenderType, and removeImagesAfter options. Input:PDF Output:PDF Type:SI-Conditional

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**languages** | Option<[**Vec<String>**](String.md)> | List of languages to use in OCR processing |  |
**sidecar** | Option<**bool**> | Include OCR text in a sidecar text file if set to true |  |
**deskew** | Option<**bool**> | Deskew the input file if set to true |  |
**clean** | Option<**bool**> | Clean the input file if set to true |  |
**clean_final** | Option<**bool**> | Clean the final output if set to true |  |
**ocr_type** | Option<**String**> | Specify the OCR type, e.g., 'skip-text', 'force-ocr', or 'Normal' |  |
**ocr_render_type** | Option<**String**> | Specify the OCR render type, either 'hocr' or 'sandwich' |  |[default to hocr]
**remove_images_after** | Option<**bool**> | Remove images from the output PDF if set to true |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_blank_pages

> Vec<String> remove_blank_pages(file_input, threshold, white_percent)
Remove blank pages from a PDF file

This endpoint removes blank pages from a given PDF file. Users can specify the threshold and white percentage to tune the detection of blank pages. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**threshold** | Option<**i32**> | The threshold value to determine blank pages |  |[default to 10]
**white_percent** | Option<**f32**> | The percentage of white color on a page to consider it as blank |  |[default to 99.9]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repair_pdf

> Vec<String> repair_pdf(file_input)
Repair a PDF file

This endpoint repairs a given PDF file by running Ghostscript command. The PDF is first saved to a temporary location, repaired, read back, and then returned as a response. Input:PDF Output:PDF Type:SISO

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


## replace_and_invert_color

> std::path::PathBuf replace_and_invert_color(file_input, replace_and_invert_option, high_contrast_color_combination, back_ground_color, text_color)
Replace-Invert Color PDF

This endpoint accepts a PDF file and option of invert all colors or replace text and background colors. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**replace_and_invert_option** | Option<**String**> | Replace and Invert color options of a pdf. |  |
**high_contrast_color_combination** | Option<**String**> | If HIGH_CONTRAST_COLOR option selected, then pick the default color option for text and background. |  |
**back_ground_color** | Option<**String**> | If CUSTOM_COLOR option selected, then pick the custom color for background. Expected color value should be 24bit decimal value of a color |  |
**text_color** | Option<**String**> | If CUSTOM_COLOR option selected, then pick the custom color for text. Expected color value should be 24bit decimal value of a color |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

