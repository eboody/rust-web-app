# \ConvertApi

All URIs are relative to *https://spdf.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**convert_to_image**](ConvertApi.md#convert_to_image) | **POST** /api/v1/convert/pdf/img | Convert PDF to image(s)
[**convert_to_pdf**](ConvertApi.md#convert_to_pdf) | **POST** /api/v1/convert/img/pdf | Convert images to a PDF file
[**html_to_pdf**](ConvertApi.md#html_to_pdf) | **POST** /api/v1/convert/html/pdf | Convert an HTML or ZIP (containing HTML and CSS) to PDF
[**markdown_to_pdf**](ConvertApi.md#markdown_to_pdf) | **POST** /api/v1/convert/markdown/pdf | Convert a Markdown file to PDF
[**pdf_to_csv**](ConvertApi.md#pdf_to_csv) | **POST** /api/v1/convert/pdf/csv | Extracts a CSV document from a PDF
[**pdf_to_pdf_a**](ConvertApi.md#pdf_to_pdf_a) | **POST** /api/v1/convert/pdf/pdfa | Convert a PDF to a PDF/A
[**process_file_to_pdf**](ConvertApi.md#process_file_to_pdf) | **POST** /api/v1/convert/file/pdf | Convert a file to a PDF using LibreOffice
[**process_pdf_to_html**](ConvertApi.md#process_pdf_to_html) | **POST** /api/v1/convert/pdf/html | Convert PDF to HTML
[**process_pdf_to_presentation**](ConvertApi.md#process_pdf_to_presentation) | **POST** /api/v1/convert/pdf/presentation | Convert PDF to Presentation format
[**process_pdf_to_rt_for_txt**](ConvertApi.md#process_pdf_to_rt_for_txt) | **POST** /api/v1/convert/pdf/text | Convert PDF to Text or RTF format
[**process_pdf_to_word**](ConvertApi.md#process_pdf_to_word) | **POST** /api/v1/convert/pdf/word | Convert PDF to Word document
[**process_pdf_to_xml**](ConvertApi.md#process_pdf_to_xml) | **POST** /api/v1/convert/pdf/xml | Convert PDF to XML
[**url_to_pdf**](ConvertApi.md#url_to_pdf) | **POST** /api/v1/convert/url/pdf | Convert a URL to a PDF



## convert_to_image

> Vec<String> convert_to_image(file_input, image_format, single_or_multiple, color_type, dpi)
Convert PDF to image(s)

This endpoint converts a PDF file to image(s) with the specified image format, color type, and DPI. Users can choose to get a single image or multiple images.  Input:PDF Output:Image Type:SI-Conditional

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**image_format** | Option<**String**> | The output image format |  |
**single_or_multiple** | Option<**String**> | Choose between a single image containing all pages or separate images for each page |  |
**color_type** | Option<**String**> | The color type of the output image(s) |  |
**dpi** | Option<**String**> | The DPI (dots per inch) for the output image(s) |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_to_pdf

> Vec<String> convert_to_pdf(file_input, fit_option, color_type, auto_rotate)
Convert images to a PDF file

This endpoint converts one or more images to a PDF file. Users can specify whether to stretch the images to fit the PDF page, and whether to automatically rotate the images. Input:Image Output:PDF Type:MISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> | The input images to be converted to a PDF file |  |
**fit_option** | Option<**String**> | Option to determine how the image will fit onto the page |  |
**color_type** | Option<**String**> | The color type of the output image(s) |  |
**auto_rotate** | Option<**bool**> | Whether to automatically rotate the images to better fit the PDF page |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## html_to_pdf

> Vec<String> html_to_pdf(file_input, zoom)
Convert an HTML or ZIP (containing HTML and CSS) to PDF

This endpoint takes an HTML or ZIP file input and converts it to a PDF format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**zoom** | Option<**f32**> | Zoom level for displaying the website. Default is '1'. |  |[default to 1]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## markdown_to_pdf

> Vec<String> markdown_to_pdf(file_input)
Convert a Markdown file to PDF

This endpoint takes a Markdown file input, converts it to HTML, and then to PDF format. Input:MARKDOWN Output:PDF Type:SISO

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


## pdf_to_csv

> String pdf_to_csv(file_input, page_id)
Extracts a CSV document from a PDF

This operation takes an input PDF file and returns CSV file of whole page. Input:PDF Output:CSV Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**page_id** | Option<**f64**> | Number of chosen page |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pdf_to_pdf_a

> Vec<String> pdf_to_pdf_a(file_input, output_format)
Convert a PDF to a PDF/A

This endpoint converts a PDF file to a PDF/A file. PDF/A is a format designed for long-term archiving of digital documents. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**output_format** | Option<**String**> | The output PDF/A type |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_file_to_pdf

> Vec<String> process_file_to_pdf(file_input)
Convert a file to a PDF using LibreOffice

This endpoint converts a given file to a PDF using LibreOffice API  Input:ANY Output:PDF Type:SISO

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


## process_pdf_to_html

> Vec<String> process_pdf_to_html(file_input)
Convert PDF to HTML

This endpoint converts a PDF file to HTML format. Input:PDF Output:HTML Type:SISO

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


## process_pdf_to_presentation

> Vec<String> process_pdf_to_presentation(file_input, output_format)
Convert PDF to Presentation format

This endpoint converts a given PDF file to a Presentation format. Input:PDF Output:PPT Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**output_format** | Option<**String**> | The output Presentation format |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_pdf_to_rt_for_txt

> Vec<String> process_pdf_to_rt_for_txt(file_input, output_format)
Convert PDF to Text or RTF format

This endpoint converts a given PDF file to Text or RTF format. Input:PDF Output:TXT Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**output_format** | Option<**String**> | The output Text or RTF format |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_pdf_to_word

> Vec<String> process_pdf_to_word(file_input, output_format)
Convert PDF to Word document

This endpoint converts a given PDF file to a Word document format. Input:PDF Output:WORD Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**output_format** | Option<**String**> | The output Word document format |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_pdf_to_xml

> Vec<String> process_pdf_to_xml(file_input)
Convert PDF to XML

This endpoint converts a PDF file to an XML file. Input:PDF Output:XML Type:SISO

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


## url_to_pdf

> Vec<String> url_to_pdf(url_input)
Convert a URL to a PDF

This endpoint fetches content from a URL and converts it to a PDF format. Input:N/A Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_input** | **String** | The input URL to be converted to a PDF file | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

