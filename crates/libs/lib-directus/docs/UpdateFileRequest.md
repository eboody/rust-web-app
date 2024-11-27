# UpdateFileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | Title for the file. Is extracted from the filename on upload, but can be edited by the user. | [optional]
**filename_download** | Option<**String**> | Preferred filename when file is downloaded. | [optional]
**description** | Option<**String**> | Description for the file. | [optional]
**folder** | Option<[**models::UpdateFileRequestFolder**](updateFile_request_folder.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Tags for the file. Is automatically populated based on Exif data for images. | [optional]
**file** | Option<[**serde_json::Value**](.md)> | File contents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


