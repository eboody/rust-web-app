# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier of the service. Only used for database identification. | [optional]
**uuid** | Option<**String**> | The unique identifier of the service. | [optional]
**name** | Option<**String**> | The name of the service. | [optional]
**environment_id** | Option<**i32**> | The unique identifier of the environment where the service is attached to. | [optional]
**server_id** | Option<**i32**> | The unique identifier of the server where the service is running. | [optional]
**description** | Option<**String**> | The description of the service. | [optional]
**docker_compose_raw** | Option<**String**> | The raw docker-compose.yml file of the service. | [optional]
**docker_compose** | Option<**String**> | The docker-compose.yml file that is parsed and modified by Coolify. | [optional]
**destination_type** | Option<**String**> | Destination type. | [optional]
**destination_id** | Option<**i32**> | The unique identifier of the destination where the service is running. | [optional]
**connect_to_docker_network** | Option<**bool**> | The flag to connect the service to the predefined Docker network. | [optional]
**is_container_label_escape_enabled** | Option<**bool**> | The flag to enable the container label escape. | [optional]
**is_container_label_readonly_enabled** | Option<**bool**> | The flag to enable the container label readonly. | [optional]
**config_hash** | Option<**String**> | The hash of the service configuration. | [optional]
**service_type** | Option<**String**> | The type of the service. | [optional]
**created_at** | Option<**String**> | The date and time when the service was created. | [optional]
**updated_at** | Option<**String**> | The date and time when the service was last updated. | [optional]
**deleted_at** | Option<**String**> | The date and time when the service was deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


