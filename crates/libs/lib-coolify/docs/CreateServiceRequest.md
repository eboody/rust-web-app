# CreateServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The one-click service type | 
**name** | Option<**String**> | Name of the service. | [optional]
**description** | Option<**String**> | Description of the service. | [optional]
**project_uuid** | **String** | Project UUID. | 
**environment_name** | **String** | Environment name. | 
**server_uuid** | **String** | Server UUID. | 
**destination_uuid** | Option<**String**> | Destination UUID. Required if server has multiple destinations. | [optional]
**instant_deploy** | Option<**bool**> | Start the service immediately after creation. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


