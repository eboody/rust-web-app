# Files

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the file. | [optional]
**storage** | Option<**String**> | Where the file is stored. Either `local` for the local filesystem or the name of the storage adapter (for example `s3`). | [optional]
**filename_disk** | Option<**String**> | Name of the file on disk. By default, Directus uses a random hash for the filename. | [optional]
**filename_download** | Option<**String**> | How you want to the file to be named when it's being downloaded. | [optional]
**title** | Option<**String**> | Title for the file. Is extracted from the filename on upload, but can be edited by the user. | [optional]
**r#type** | Option<**String**> | MIME type of the file. | [optional]
**folder** | Option<[**models::UpdateFileRequestFolder**](updateFile_request_folder.md)> |  | [optional]
**uploaded_by** | Option<[**models::FilesUploadedBy**](Files_uploaded_by.md)> |  | [optional]
**created_on** | Option<**String**> | When the file was created. | [optional]
**modified_by** | Option<[**models::FilesModifiedBy**](Files_modified_by.md)> |  | [optional]
**modified_on** | Option<**String**> |  | [optional]
**charset** | Option<**String**> | Character set of the file. | [optional]
**filesize** | Option<**i32**> | Size of the file in bytes. | [optional]
**width** | Option<**i32**> | Width of the file in pixels. Only applies to images. | [optional]
**height** | Option<**i32**> | Height of the file in pixels. Only applies to images. | [optional]
**duration** | Option<**i32**> | Duration of the file in seconds. Only applies to audio and video. | [optional]
**embed** | Option<**String**> | Where the file was embedded from. | [optional]
**description** | Option<**String**> | Description for the file. | [optional]
**location** | Option<**String**> | Where the file was created. Is automatically populated based on Exif data for images. | [optional]
**tags** | Option<**Vec<String>**> | Tags for the file. Is automatically populated based on Exif data for images. | [optional]
**metadata** | Option<[**json::Value**](.md)> | IPTC, Exif, and ICC metadata extracted from file | [optional]
**focal_point_x** | Option<**i32**> |  | [optional]
**focal_point_y** | Option<**i32**> |  | [optional]
**tus_id** | Option<**String**> |  | [optional]
**tus_data** | Option<[**json::Value**](.md)> |  | [optional]
**uploaded_on** | Option<**String**> | When the file was last uploaded/replaced. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


