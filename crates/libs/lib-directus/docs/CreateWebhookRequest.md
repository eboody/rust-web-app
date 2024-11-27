# CreateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the webhook. | [optional]
**method** | Option<**String**> | Method used in the webhook. | [optional]
**url** | Option<**String**> | The url of the webhook. | [optional]
**status** | Option<**String**> | The status of the webhook. | [optional]
**data** | Option<**bool**> | If yes, send the content of what was done | [optional]
**actions** | Option<[**serde_json::Value**](.md)> | The actions that triggers this webhook. | [optional]
**system_collections** | Option<[**serde_json::Value**](.md)> | The collections that triggers this webhook. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


