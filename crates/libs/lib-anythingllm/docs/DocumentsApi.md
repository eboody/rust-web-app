# \DocumentsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_document_accepted_file_types_get**](DocumentsApi.md#v1_document_accepted_file_types_get) | **GET** /v1/document/accepted-file-types | 
[**v1_document_create_folder_post**](DocumentsApi.md#v1_document_create_folder_post) | **POST** /v1/document/create-folder | 
[**v1_document_doc_name_get**](DocumentsApi.md#v1_document_doc_name_get) | **GET** /v1/document/{docName} | 
[**v1_document_metadata_schema_get**](DocumentsApi.md#v1_document_metadata_schema_get) | **GET** /v1/document/metadata-schema | 
[**v1_document_move_files_post**](DocumentsApi.md#v1_document_move_files_post) | **POST** /v1/document/move-files | 
[**v1_document_raw_text_post**](DocumentsApi.md#v1_document_raw_text_post) | **POST** /v1/document/raw-text | 
[**v1_document_upload_link_post**](DocumentsApi.md#v1_document_upload_link_post) | **POST** /v1/document/upload-link | 
[**v1_document_upload_post**](DocumentsApi.md#v1_document_upload_post) | **POST** /v1/document/upload | 
[**v1_documents_get**](DocumentsApi.md#v1_documents_get) | **GET** /v1/documents | 



## v1_document_accepted_file_types_get

> json::Value v1_document_accepted_file_types_get()


Check available filetypes and MIMEs that can be uploaded.

### Parameters

This endpoint does not need any parameter.

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_create_folder_post

> json::Value v1_document_create_folder_post(body)


Create a new folder inside the documents storage directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Name of the folder to create. | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_doc_name_get

> json::Value v1_document_doc_name_get(doc_name)


Get a single document by its unique AnythingLLM document name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doc_name** | **String** | Unique document name to find (name in /documents) | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_metadata_schema_get

> json::Value v1_document_metadata_schema_get()


Get the known available metadata schema for when doing a raw-text upload and the acceptable type of value for each key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_move_files_post

> json::Value v1_document_move_files_post(body)


Move files within the documents storage directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **json::Value** | Array of objects containing source and destination paths of files to move. | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_raw_text_post

> json::Value v1_document_raw_text_post(body)


Upload a file by specifying its raw text content and metadata values without having to upload a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **json::Value** | Text content and metadata of the file to be saved to the system. Use metadata-schema endpoint to get the possible metadata keys | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_upload_link_post

> json::Value v1_document_upload_link_post(body)


Upload a valid URL for AnythingLLM to scrape and prepare for embedding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **json::Value** | Link of web address to be scraped. | [required] |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_document_upload_post

> json::Value v1_document_upload_post(file)


Upload a new file to AnythingLLM to be parsed and prepared for embedding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_documents_get

> json::Value v1_documents_get()


List of all locally-stored documents in instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**json::Value**](json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

