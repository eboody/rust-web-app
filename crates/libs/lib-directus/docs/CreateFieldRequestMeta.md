# CreateFieldRequestMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for the field in the `directus_fields` collection. | [optional]
**collection** | Option<**String**> | Unique name of the collection this field is in. | [optional]
**field** | Option<**String**> | Unique name of the field. Field name is unique within the collection. | [optional]
**special** | Option<**Vec<String>**> | Transformation flag for field | [optional]
**system_interface** | Option<**String**> | What interface is used in the admin app to edit the value for this field. | [optional]
**options** | Option<[**json::Value**](.md)> | Options for the interface that's used. This format is based on the individual interface. | [optional]
**display** | Option<**String**> | What display is used in the admin app to display the value for this field. | [optional]
**display_options** | Option<[**json::Value**](.md)> | Options for the display that's used. This format is based on the individual display. | [optional]
**locked** | Option<**bool**> | If the field can be altered by the end user. Directus system fields have this value set to `true`. | [optional]
**readonly** | Option<**bool**> | Prevents the user from editing the value in the field. | [optional]
**hidden** | Option<**bool**> | If this field should be hidden. | [optional]
**sort** | Option<**i32**> | Sort order of this field on the edit page of the admin app. | [optional]
**width** | Option<**String**> | Width of the field on the edit form. | [optional]
**group** | Option<**i32**> | What field group this field is part of. | [optional]
**translation** | Option<[**json::Value**](.md)> | Key value pair of `<language>: <translation>` that allows the user to change the displayed name of the field in the admin app. | [optional]
**note** | Option<**String**> | A user provided note for the field. Will be rendered alongside the interface on the edit page. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


