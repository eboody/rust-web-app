# CreateDockercomposeApplicationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_uuid** | **String** | The project UUID. | 
**server_uuid** | **String** | The server UUID. | 
**environment_name** | **String** | The environment name. | 
**docker_compose_raw** | **String** | The Docker Compose raw content. | 
**destination_uuid** | Option<**String**> | The destination UUID if the server has more than one destinations. | [optional]
**name** | Option<**String**> | The application name. | [optional]
**description** | Option<**String**> | The application description. | [optional]
**instant_deploy** | Option<**bool**> | The flag to indicate if the application should be deployed instantly. | [optional]
**use_build_server** | Option<**bool**> | Use build server. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


