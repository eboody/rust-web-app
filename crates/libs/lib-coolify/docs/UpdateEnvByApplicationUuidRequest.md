# UpdateEnvByApplicationUuidRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | The key of the environment variable. | 
**value** | **String** | The value of the environment variable. | 
**is_preview** | Option<**bool**> | The flag to indicate if the environment variable is used in preview deployments. | [optional]
**is_build_time** | Option<**bool**> | The flag to indicate if the environment variable is used in build time. | [optional]
**is_literal** | Option<**bool**> | The flag to indicate if the environment variable is a literal, nothing espaced. | [optional]
**is_multiline** | Option<**bool**> | The flag to indicate if the environment variable is multiline. | [optional]
**is_shown_once** | Option<**bool**> | The flag to indicate if the environment variable's value is shown on the UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


