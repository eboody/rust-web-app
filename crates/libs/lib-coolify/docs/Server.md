# Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The server ID. | [optional]
**uuid** | Option<**String**> | The server UUID. | [optional]
**name** | Option<**String**> | The server name. | [optional]
**description** | Option<**String**> | The server description. | [optional]
**ip** | Option<**String**> | The IP address. | [optional]
**user** | Option<**String**> | The user. | [optional]
**port** | Option<**i32**> | The port number. | [optional]
**proxy** | Option<[**serde_json::Value**](.md)> | The proxy configuration. | [optional]
**proxy_type** | Option<**String**> | The proxy type. | [optional]
**high_disk_usage_notification_sent** | Option<**bool**> | The flag to indicate if the high disk usage notification has been sent. | [optional]
**unreachable_notification_sent** | Option<**bool**> | The flag to indicate if the unreachable notification has been sent. | [optional]
**unreachable_count** | Option<**i32**> | The unreachable count for your server. | [optional]
**validation_logs** | Option<**String**> | The validation logs. | [optional]
**log_drain_notification_sent** | Option<**bool**> | The flag to indicate if the log drain notification has been sent. | [optional]
**swarm_cluster** | Option<**String**> | The swarm cluster configuration. | [optional]
**delete_unused_volumes** | Option<**bool**> | The flag to indicate if the unused volumes should be deleted. | [optional]
**delete_unused_networks** | Option<**bool**> | The flag to indicate if the unused networks should be deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


