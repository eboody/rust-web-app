# CreateFieldRequestSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the field. | [optional]
**table** | Option<**String**> | The collection of the field. | [optional]
**r#type** | Option<**String**> | The type of the field. | [optional]
**default_value** | Option<**String**> | The default value of the field. | [optional]
**max_length** | Option<**i32**> | The max length of the field. | [optional]
**is_nullable** | Option<**bool**> | If the field is nullable. | [optional]
**is_primary_key** | Option<**bool**> | If the field is primary key. | [optional]
**has_auto_increment** | Option<**bool**> | If the field has auto increment. | [optional]
**foreign_key_column** | Option<**String**> | Related column from the foreign key constraint. | [optional]
**foreign_key_table** | Option<**String**> | Related table from the foreign key constraint. | [optional]
**comment** | Option<**String**> | Comment as saved in the database. | [optional]
**schema** | Option<**String**> | Database schema (pg only). | [optional]
**foreign_key_schema** | Option<**String**> | Related schema from the foreign key constraint (pg only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


