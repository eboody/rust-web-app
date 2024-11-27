# CreateDatabaseMongodbRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server_uuid** | **String** | UUID of the server | 
**project_uuid** | **String** | UUID of the project | 
**environment_name** | **String** | Name of the environment | 
**destination_uuid** | Option<**String**> | UUID of the destination if the server has multiple destinations | [optional]
**mongo_conf** | Option<**String**> | MongoDB conf | [optional]
**mongo_initdb_root_username** | Option<**String**> | MongoDB initdb root username | [optional]
**name** | Option<**String**> | Name of the database | [optional]
**description** | Option<**String**> | Description of the database | [optional]
**image** | Option<**String**> | Docker Image of the database | [optional]
**is_public** | Option<**bool**> | Is the database public? | [optional]
**public_port** | Option<**i32**> | Public port of the database | [optional]
**limits_memory** | Option<**String**> | Memory limit of the database | [optional]
**limits_memory_swap** | Option<**String**> | Memory swap limit of the database | [optional]
**limits_memory_swappiness** | Option<**i32**> | Memory swappiness of the database | [optional]
**limits_memory_reservation** | Option<**String**> | Memory reservation of the database | [optional]
**limits_cpus** | Option<**String**> | CPU limit of the database | [optional]
**limits_cpuset** | Option<**String**> | CPU set of the database | [optional]
**limits_cpu_shares** | Option<**i32**> | CPU shares of the database | [optional]
**instant_deploy** | Option<**bool**> | Instant deploy the database | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


