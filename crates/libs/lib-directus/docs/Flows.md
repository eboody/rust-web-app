# Flows

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the flow. | [optional]
**name** | Option<**String**> | The name of the flow. | [optional]
**icon** | Option<**String**> | Icon displayed in the Admin App for the flow. | [optional]
**color** | Option<**String**> | Color of the icon displayed in the Admin App for the flow. | [optional]
**description** | Option<**String**> |  | [optional]
**status** | Option<**String**> | Current status of the flow. | [optional][default to Active]
**trigger** | Option<**String**> | Type of trigger for the flow. One of `hook`, `webhook`, `operation`, `schedule`, `manual`. | [optional]
**accountability** | Option<**String**> | The permission used during the flow. One of `$public`, `$trigger`, `$full`, or UUID of a role. | [optional]
**options** | Option<[**json::Value**](.md)> | Options of the selected trigger for the flow. | [optional]
**operation** | Option<[**models::FlowsOperation**](Flows_operation.md)> |  | [optional]
**date_created** | Option<**String**> | Timestamp in ISO8601 when the flow was created. | [optional]
**user_created** | Option<[**models::FlowsUserCreated**](Flows_user_created.md)> |  | [optional]
**operations** | Option<[**Vec<models::FlowsOperationsInner>**](Flows_operations_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


