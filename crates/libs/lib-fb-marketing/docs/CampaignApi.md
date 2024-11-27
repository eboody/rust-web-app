# \CampaignApi

All URIs are relative to *https://graph.facebook.com/v12.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_campaign**](CampaignApi.md#create_campaign) | **POST** /act_{ad_account_id}/campaigns | 
[**delete_campaign**](CampaignApi.md#delete_campaign) | **DELETE** /{campaign_id} | 
[**dissociate_campaign**](CampaignApi.md#dissociate_campaign) | **DELETE** /act_{ad_account_id}/campaigns | 
[**get_campaigns**](CampaignApi.md#get_campaigns) | **GET** /act_{ad_account_id}/campaigns | 
[**update_campaign**](CampaignApi.md#update_campaign) | **POST** /{campaign_id} | 



## create_campaign

> models::CampaignResponse create_campaign(ad_account_id, properties)


Cerates a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**properties** | [**Campaign**](.md) | Campaign properties | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_campaign

> models::CampaignResponse delete_campaign(campaign_id)


Deletes a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | ID of the campaign. | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dissociate_campaign

> models::CampaignDissociateResponse dissociate_campaign(ad_account_id, delete_strategy, before_date, object_count)


Dissociate a campaign from an AdAccount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**delete_strategy** | **String** | Delete strategy | [required] |
**before_date** | Option<**String**> | Set a before date to delete campaigns before this date |  |
**object_count** | Option<**i32**> | Object count |  |

### Return type

[**models::CampaignDissociateResponse**](CampaignDissociateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaigns

> models::CampaignList get_campaigns(ad_account_id, date_preset, effective_status, is_completed, time_range, fields, summary)


Returns campaigns under this ad account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ad_account_id** | **String** | ID of the ad account. | [required] |
**date_preset** | Option<**String**> | Predefined date range used to aggregate insights metrics. |  |
**effective_status** | Option<[**Vec<String>**](String.md)> | Effective status for the campaigns |  |
**is_completed** | Option<**bool**> | If true, we return completed campaigns. |  |
**time_range** | Option<[**TimeRange**](.md)> | Date range used to aggregate insights metrics |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields of the campaign |  |
**summary** | Option<[**Vec<String>**](String.md)> | Aggregated information about the edge, such as counts |  |

### Return type

[**models::CampaignList**](CampaignList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_campaign

> models::CampaignResponse update_campaign(campaign_id, properties)


Updates a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | ID of the campaign. | [required] |
**properties** | [**CampaignUpdate**](.md) | Campaign update properties | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

