# Webhooks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The index of the webhook. | [optional]
**name** | Option<**String**> | The name of the webhook. | [optional]
**method** | Option<**String**> | Method used in the webhook. | [optional]
**url** | Option<**String**> | The url of the webhook. | [optional]
**status** | Option<**String**> | The status of the webhook. | [optional]
**data** | Option<**bool**> | If yes, send the content of what was done | [optional]
**actions** | Option<**Vec<String>**> | The actions that triggers this webhook. | [optional]
**collections** | Option<**Vec<String>**> |  | [optional]
**headers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**was_active_before_deprecation** | Option<**bool**> |  | [optional]
**migrated_flow** | Option<[**models::WebhooksMigratedFlow**](Webhooks_migrated_flow.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


