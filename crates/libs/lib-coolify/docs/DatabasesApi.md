# \DatabasesApi

All URIs are relative to *https://app.coolify.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_database_clickhouse**](DatabasesApi.md#create_database_clickhouse) | **POST** /databases/clickhouse | Create (Clickhouse)
[**create_database_dragonfly**](DatabasesApi.md#create_database_dragonfly) | **POST** /databases/dragonfly | Create (DragonFly)
[**create_database_keydb**](DatabasesApi.md#create_database_keydb) | **POST** /databases/keydb | Create (KeyDB)
[**create_database_mariadb**](DatabasesApi.md#create_database_mariadb) | **POST** /databases/mariadb | Create (MariaDB)
[**create_database_mongodb**](DatabasesApi.md#create_database_mongodb) | **POST** /databases/mongodb | Create (MongoDB)
[**create_database_mysql**](DatabasesApi.md#create_database_mysql) | **POST** /databases/mysql | Create (MySQL)
[**create_database_postgresql**](DatabasesApi.md#create_database_postgresql) | **POST** /databases/postgresql | Create (PostgreSQL)
[**create_database_redis**](DatabasesApi.md#create_database_redis) | **POST** /databases/redis | Create (Redis)
[**delete_database_by_uuid**](DatabasesApi.md#delete_database_by_uuid) | **DELETE** /databases/{uuid} | Delete
[**get_database_by_uuid**](DatabasesApi.md#get_database_by_uuid) | **GET** /databases/{uuid} | Get
[**list_databases**](DatabasesApi.md#list_databases) | **GET** /databases | List
[**restart_database_by_uuid**](DatabasesApi.md#restart_database_by_uuid) | **GET** /databases/{uuid}/restart | Restart
[**start_database_by_uuid**](DatabasesApi.md#start_database_by_uuid) | **GET** /databases/{uuid}/start | Start
[**stop_database_by_uuid**](DatabasesApi.md#stop_database_by_uuid) | **GET** /databases/{uuid}/stop | Stop
[**update_database_by_uuid**](DatabasesApi.md#update_database_by_uuid) | **PATCH** /databases/{uuid} | Update



## create_database_clickhouse

> create_database_clickhouse(create_database_clickhouse_request)
Create (Clickhouse)

Create a new Clickhouse database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_clickhouse_request** | [**CreateDatabaseClickhouseRequest**](CreateDatabaseClickhouseRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_dragonfly

> create_database_dragonfly(create_database_dragonfly_request)
Create (DragonFly)

Create a new DragonFly database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_dragonfly_request** | [**CreateDatabaseDragonflyRequest**](CreateDatabaseDragonflyRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_keydb

> create_database_keydb(create_database_keydb_request)
Create (KeyDB)

Create a new KeyDB database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_keydb_request** | [**CreateDatabaseKeydbRequest**](CreateDatabaseKeydbRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_mariadb

> create_database_mariadb(create_database_mariadb_request)
Create (MariaDB)

Create a new MariaDB database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_mariadb_request** | [**CreateDatabaseMariadbRequest**](CreateDatabaseMariadbRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_mongodb

> create_database_mongodb(create_database_mongodb_request)
Create (MongoDB)

Create a new MongoDB database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_mongodb_request** | [**CreateDatabaseMongodbRequest**](CreateDatabaseMongodbRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_mysql

> create_database_mysql(create_database_mysql_request)
Create (MySQL)

Create a new MySQL database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_mysql_request** | [**CreateDatabaseMysqlRequest**](CreateDatabaseMysqlRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_postgresql

> create_database_postgresql(create_database_postgresql_request)
Create (PostgreSQL)

Create a new PostgreSQL database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_postgresql_request** | [**CreateDatabasePostgresqlRequest**](CreateDatabasePostgresqlRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_database_redis

> create_database_redis(create_database_redis_request)
Create (Redis)

Create a new Redis database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_database_redis_request** | [**CreateDatabaseRedisRequest**](CreateDatabaseRedisRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_database_by_uuid

> models::DeleteDatabaseByUuid200Response delete_database_by_uuid(uuid, delete_configurations, delete_volumes, docker_cleanup, delete_connected_networks)
Delete

Delete database by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the database. | [required] |
**delete_configurations** | Option<**bool**> | Delete configurations. |  |[default to true]
**delete_volumes** | Option<**bool**> | Delete volumes. |  |[default to true]
**docker_cleanup** | Option<**bool**> | Run docker cleanup. |  |[default to true]
**delete_connected_networks** | Option<**bool**> | Delete connected networks. |  |[default to true]

### Return type

[**models::DeleteDatabaseByUuid200Response**](delete_database_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_database_by_uuid

> String get_database_by_uuid(uuid)
Get

Get database by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the database. | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_databases

> String list_databases()
List

List all databases.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_database_by_uuid

> models::RestartDatabaseByUuid200Response restart_database_by_uuid(uuid)
Restart

Restart database. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the database. | [required] |

### Return type

[**models::RestartDatabaseByUuid200Response**](restart_database_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_database_by_uuid

> models::StartDatabaseByUuid200Response start_database_by_uuid(uuid)
Start

Start database. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the database. | [required] |

### Return type

[**models::StartDatabaseByUuid200Response**](start_database_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_database_by_uuid

> models::StopDatabaseByUuid200Response stop_database_by_uuid(uuid)
Stop

Stop database. `Post` request is also accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the database. | [required] |

### Return type

[**models::StopDatabaseByUuid200Response**](stop_database_by_uuid_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_database_by_uuid

> update_database_by_uuid(uuid, update_database_by_uuid_request)
Update

Update database by UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | UUID of the database. | [required] |
**update_database_by_uuid_request** | [**UpdateDatabaseByUuidRequest**](UpdateDatabaseByUuidRequest.md) | Database data | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

