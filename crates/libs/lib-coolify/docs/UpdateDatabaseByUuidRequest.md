# UpdateDatabaseByUuidRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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
**postgres_user** | Option<**String**> | PostgreSQL user | [optional]
**postgres_password** | Option<**String**> | PostgreSQL password | [optional]
**postgres_db** | Option<**String**> | PostgreSQL database | [optional]
**postgres_initdb_args** | Option<**String**> | PostgreSQL initdb args | [optional]
**postgres_host_auth_method** | Option<**String**> | PostgreSQL host auth method | [optional]
**postgres_conf** | Option<**String**> | PostgreSQL conf | [optional]
**clickhouse_admin_user** | Option<**String**> | Clickhouse admin user | [optional]
**clickhouse_admin_password** | Option<**String**> | Clickhouse admin password | [optional]
**dragonfly_password** | Option<**String**> | DragonFly password | [optional]
**redis_password** | Option<**String**> | Redis password | [optional]
**redis_conf** | Option<**String**> | Redis conf | [optional]
**keydb_password** | Option<**String**> | KeyDB password | [optional]
**keydb_conf** | Option<**String**> | KeyDB conf | [optional]
**mariadb_conf** | Option<**String**> | MariaDB conf | [optional]
**mariadb_root_password** | Option<**String**> | MariaDB root password | [optional]
**mariadb_user** | Option<**String**> | MariaDB user | [optional]
**mariadb_password** | Option<**String**> | MariaDB password | [optional]
**mariadb_database** | Option<**String**> | MariaDB database | [optional]
**mongo_conf** | Option<**String**> | Mongo conf | [optional]
**mongo_initdb_root_username** | Option<**String**> | Mongo initdb root username | [optional]
**mongo_initdb_root_password** | Option<**String**> | Mongo initdb root password | [optional]
**mongo_initdb_init_database** | Option<**String**> | Mongo initdb init database | [optional]
**mysql_root_password** | Option<**String**> | MySQL root password | [optional]
**mysql_user** | Option<**String**> | MySQL user | [optional]
**mysql_database** | Option<**String**> | MySQL database | [optional]
**mysql_conf** | Option<**String**> | MySQL conf | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


