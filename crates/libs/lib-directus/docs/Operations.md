# Operations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the operation. | [optional]
**name** | Option<**String**> | The name of the operation. | [optional]
**key** | Option<**String**> | Key for the operation. Must be unique within a given flow. | [optional]
**r#type** | Option<**String**> | Type of operation. One of `log`, `mail`, `notification`, `create`, `read`, `request`, `sleep`, `transform`, `trigger`, `condition`, or any type of custom operation extensions. | [optional]
**position_x** | Option<**i32**> | Position of the operation on the X axis within the flow workspace. | [optional]
**position_y** | Option<**i32**> | Position of the operation on the Y axis within the flow workspace. | [optional]
**options** | Option<[**json::Value**](.md)> | Options depending on the type of the operation. | [optional]
**resolve** | Option<[**models::OperationsResolve**](Operations_resolve.md)> |  | [optional]
**reject** | Option<[**models::OperationsReject**](Operations_reject.md)> |  | [optional]
**flow** | Option<[**models::OperationsFlow**](Operations_flow.md)> |  | [optional]
**date_created** | Option<**String**> | Timestamp in ISO8601 when the operation was created. | [optional]
**user_created** | Option<[**models::OperationsUserCreated**](Operations_user_created.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


