# CreateDockerfileApplicationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_uuid** | **String** | The project UUID. | 
**server_uuid** | **String** | The server UUID. | 
**environment_name** | **String** | The environment name. | 
**dockerfile** | **String** | The Dockerfile content. | 
**build_pack** | Option<**String**> | The build pack type. | [optional]
**ports_exposes** | Option<**String**> | The ports to expose. | [optional]
**destination_uuid** | Option<**String**> | The destination UUID. | [optional]
**name** | Option<**String**> | The application name. | [optional]
**description** | Option<**String**> | The application description. | [optional]
**domains** | Option<**String**> | The application domains. | [optional]
**docker_registry_image_name** | Option<**String**> | The docker registry image name. | [optional]
**docker_registry_image_tag** | Option<**String**> | The docker registry image tag. | [optional]
**ports_mappings** | Option<**String**> | The ports mappings. | [optional]
**base_directory** | Option<**String**> | The base directory for all commands. | [optional]
**health_check_enabled** | Option<**bool**> | Health check enabled. | [optional]
**health_check_path** | Option<**String**> | Health check path. | [optional]
**health_check_port** | Option<**String**> | Health check port. | [optional]
**health_check_host** | Option<**String**> | Health check host. | [optional]
**health_check_method** | Option<**String**> | Health check method. | [optional]
**health_check_return_code** | Option<**i32**> | Health check return code. | [optional]
**health_check_scheme** | Option<**String**> | Health check scheme. | [optional]
**health_check_response_text** | Option<**String**> | Health check response text. | [optional]
**health_check_interval** | Option<**i32**> | Health check interval in seconds. | [optional]
**health_check_timeout** | Option<**i32**> | Health check timeout in seconds. | [optional]
**health_check_retries** | Option<**i32**> | Health check retries count. | [optional]
**health_check_start_period** | Option<**i32**> | Health check start period in seconds. | [optional]
**limits_memory** | Option<**String**> | Memory limit. | [optional]
**limits_memory_swap** | Option<**String**> | Memory swap limit. | [optional]
**limits_memory_swappiness** | Option<**i32**> | Memory swappiness. | [optional]
**limits_memory_reservation** | Option<**String**> | Memory reservation. | [optional]
**limits_cpus** | Option<**String**> | CPU limit. | [optional]
**limits_cpuset** | Option<**String**> | CPU set. | [optional]
**limits_cpu_shares** | Option<**i32**> | CPU shares. | [optional]
**custom_labels** | Option<**String**> | Custom labels. | [optional]
**custom_docker_run_options** | Option<**String**> | Custom docker run options. | [optional]
**post_deployment_command** | Option<**String**> | Post deployment command. | [optional]
**post_deployment_command_container** | Option<**String**> | Post deployment command container. | [optional]
**pre_deployment_command** | Option<**String**> | Pre deployment command. | [optional]
**pre_deployment_command_container** | Option<**String**> | Pre deployment command container. | [optional]
**manual_webhook_secret_github** | Option<**String**> | Manual webhook secret for Github. | [optional]
**manual_webhook_secret_gitlab** | Option<**String**> | Manual webhook secret for Gitlab. | [optional]
**manual_webhook_secret_bitbucket** | Option<**String**> | Manual webhook secret for Bitbucket. | [optional]
**manual_webhook_secret_gitea** | Option<**String**> | Manual webhook secret for Gitea. | [optional]
**redirect** | Option<**String**> | How to set redirect with Traefik / Caddy. www<->non-www. | [optional]
**instant_deploy** | Option<**bool**> | The flag to indicate if the application should be deployed instantly. | [optional]
**use_build_server** | Option<**bool**> | Use build server. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


