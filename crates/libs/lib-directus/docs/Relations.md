# Relations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for the relation. | [optional]
**many_collection** | Option<**String**> | Collection that has the field that holds the foreign key. | [optional]
**many_field** | Option<**String**> | Foreign key. Field that holds the primary key of the related collection. | [optional]
**one_collection** | Option<**String**> | Collection on the _one_ side of the relationship. | [optional]
**one_field** | Option<**String**> | Alias column that serves as the _one_ side of the relationship. | [optional]
**one_collection_field** | Option<**String**> |  | [optional]
**one_allowed_collections** | Option<**Vec<String>**> |  | [optional]
**junction_field** | Option<**String**> | Field on the junction table that holds the many field of the related relation. | [optional]
**sort_field** | Option<**String**> |  | [optional]
**one_deselect_action** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


