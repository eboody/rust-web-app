# \SecurityApi

All URIs are relative to *https://spdf.eman.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_password**](SecurityApi.md#add_password) | **POST** /api/v1/security/add-password | Add password to a PDF file
[**add_watermark**](SecurityApi.md#add_watermark) | **POST** /api/v1/security/add-watermark | Add watermark to a PDF file
[**get_pdf_info**](SecurityApi.md#get_pdf_info) | **POST** /api/v1/security/get-info-on-pdf | Summary here
[**redact_pdf**](SecurityApi.md#redact_pdf) | **POST** /api/v1/security/auto-redact | Redacts listOfText in a PDF document
[**remove_cert_sign_pdf**](SecurityApi.md#remove_cert_sign_pdf) | **POST** /api/v1/security/remove-cert-sign | Remove digital signature from PDF
[**remove_password**](SecurityApi.md#remove_password) | **POST** /api/v1/security/remove-password | Remove password from a PDF file
[**sanitize_pdf**](SecurityApi.md#sanitize_pdf) | **POST** /api/v1/security/sanitize-pdf | Sanitize a PDF file
[**sign_pdf_with_cert**](SecurityApi.md#sign_pdf_with_cert) | **POST** /api/v1/security/cert-sign | Sign PDF with a Digital Certificate



## add_password

> Vec<String> add_password(file_input, owner_password, password, key_length, can_assemble_document, can_extract_content, can_extract_for_accessibility, can_fill_in_form, can_modify, can_modify_annotations, can_print, can_print_faithful)
Add password to a PDF file

This endpoint adds password protection to a PDF file. Users can specify a set of permissions that should be applied to the file. Input:PDF Output:PDF

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**owner_password** | Option<**String**> | The owner password to be added to the PDF file (Restricts what can be done with the document once it is opened) |  |
**password** | Option<**String**> | The password to be added to the PDF file (Restricts the opening of the document itself.) |  |
**key_length** | Option<**i32**> | The length of the encryption key |  |[default to 256]
**can_assemble_document** | Option<**bool**> | Whether the document assembly is allowed |  |
**can_extract_content** | Option<**bool**> | Whether content extraction for accessibility is allowed |  |
**can_extract_for_accessibility** | Option<**bool**> | Whether content extraction for accessibility is allowed |  |
**can_fill_in_form** | Option<**bool**> | Whether form filling is allowed |  |
**can_modify** | Option<**bool**> | Whether the document modification is allowed |  |
**can_modify_annotations** | Option<**bool**> | Whether modification of annotations is allowed |  |
**can_print** | Option<**bool**> | Whether printing of the document is allowed |  |
**can_print_faithful** | Option<**bool**> | Whether faithful printing is allowed |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_watermark

> Vec<String> add_watermark(watermark_type, file_input, watermark_text, watermark_image, alphabet, font_size, rotation, opacity, width_spacer, height_spacer, convert_pdfto_image)
Add watermark to a PDF file

This endpoint adds a watermark to a given PDF file. Users can specify the watermark type (text or image), rotation, opacity, width spacer, and height spacer. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watermark_type** | **String** | The watermark type (text or image) | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**watermark_text** | Option<**String**> | The watermark text |  |
**watermark_image** | Option<**std::path::PathBuf**> |  |  |
**alphabet** | Option<**String**> | The selected alphabet |  |[default to roman]
**font_size** | Option<**f32**> | The font size of the watermark text |  |
**rotation** | Option<**f32**> | The rotation of the watermark in degrees |  |
**opacity** | Option<**f32**> | The opacity of the watermark (0.0 - 1.0) |  |
**width_spacer** | Option<**i32**> | The width spacer between watermark elements |  |
**height_spacer** | Option<**i32**> | The height spacer between watermark elements |  |
**convert_pdfto_image** | Option<**bool**> | Convert the redacted PDF to an image |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pdf_info

> Vec<String> get_pdf_info(file_input)
Summary here

desc. Input:PDF Output:JSON Type:SISO

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


## redact_pdf

> Vec<String> redact_pdf(list_of_text, file_input, use_regex, whole_word_search, redact_color, custom_padding, convert_pdfto_image)
Redacts listOfText in a PDF document

This operation takes an input PDF file and redacts the provided listOfText. Input:PDF, Output:PDF, Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_of_text** | **String** | List of text to redact from the PDF | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |
**use_regex** | Option<**bool**> | Whether to use regex for the listOfText |  |[default to false]
**whole_word_search** | Option<**bool**> | Whether to use whole word search |  |[default to false]
**redact_color** | Option<**String**> | The color for redaction |  |[default to #000000]
**custom_padding** | Option<**f64**> | Custom padding for redaction |  |
**convert_pdfto_image** | Option<**bool**> | Convert the redacted PDF to an image |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_cert_sign_pdf

> Vec<String> remove_cert_sign_pdf(file_input)
Remove digital signature from PDF

This endpoint accepts a PDF file and returns the PDF file without the digital signature. Input: PDF, Output: PDF

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


## remove_password

> Vec<String> remove_password(password, file_input)
Remove password from a PDF file

This endpoint removes the password from a protected PDF file. Users need to provide the existing password. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password** | **String** | The password of the PDF file | [required] |
**file_input** | Option<**std::path::PathBuf**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sanitize_pdf

> Vec<String> sanitize_pdf(file_input, remove_java_script, remove_embedded_files, remove_metadata, remove_links, remove_fonts)
Sanitize a PDF file

This endpoint processes a PDF file and removes specific elements based on the provided options. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**remove_java_script** | Option<**bool**> | Remove JavaScript actions from the PDF |  |[default to false]
**remove_embedded_files** | Option<**bool**> | Remove embedded files from the PDF |  |[default to false]
**remove_metadata** | Option<**bool**> | Remove metadata from the PDF |  |[default to false]
**remove_links** | Option<**bool**> | Remove links from the PDF |  |[default to false]
**remove_fonts** | Option<**bool**> | Remove fonts from the PDF |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_pdf_with_cert

> Vec<String> sign_pdf_with_cert(file_input, cert_type, private_key_file, cert_file, p12_file, jks_file, password, show_signature, reason, location, name, page_number, show_logo)
Sign PDF with a Digital Certificate

This endpoint accepts a PDF file, a digital certificate and related information to sign the PDF. It then returns the digitally signed PDF file. Input:PDF Output:PDF Type:SISO

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_input** | Option<**std::path::PathBuf**> |  |  |
**cert_type** | Option<**String**> | The type of the digital certificate |  |
**private_key_file** | Option<**std::path::PathBuf**> |  |  |
**cert_file** | Option<**std::path::PathBuf**> |  |  |
**p12_file** | Option<**std::path::PathBuf**> |  |  |
**jks_file** | Option<**std::path::PathBuf**> |  |  |
**password** | Option<**String**> | The password for the keystore or the private key |  |
**show_signature** | Option<**bool**> | Whether to visually show the signature in the PDF file |  |
**reason** | Option<**String**> | The reason for signing the PDF |  |
**location** | Option<**String**> | The location where the PDF is signed |  |
**name** | Option<**String**> | The name of the signer |  |
**page_number** | Option<**i32**> | The page number where the signature should be visible. This is required if showSignature is set to true |  |
**show_logo** | Option<**bool**> | Whether to visually show a signature logo along with the signature |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

