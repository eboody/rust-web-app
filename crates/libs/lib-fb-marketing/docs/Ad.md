# Ad

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adlabels** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Ad labels associated with this ad | [optional]
**adset_id** | Option<**i32**> | The ID of the ad set, required on creation | [optional]
**adset_spec** | Option<[**serde_json::Value**](.md)> | The ad set spec for this ad. When the spec is provided, adset_id field is not required. | [optional]
**audience_id** | Option<**String**> | The ID of the ad set, required on creation | [optional]
**bid_amount** | Option<**i32**> | Bid amount for this ad which will be used in auction instead of the ad set bid_amount, if specified. | [optional]
**conversion_domain** | Option<**String**> | The domain where conversions happen. Required to create or update an ad in a campaign that shares data with a pixel. | [optional]
**creative** | [**serde_json::Value**](.md) | This field is required for create. The ID or creative spec of the ad creative to be used by this ad. | 
**date_format** | Option<**String**> | The format of the date. | [optional]
**display_sequence** | Option<**i32**> | The sequence of the ad within the same campaign | [optional]
**draft_adgroup_id** | Option<[**models::AdUpdateDraftAdgroupId**](AdUpdate_draft_adgroup_id.md)> |  | [optional]
**engagement_audience** | Option<**String**> | Flag to create a new audience based on users who engage with this ad | [optional]
**execution_options** | Option<**Vec<String>**> | An execution setting | [optional]
**include_demolink_hashes** | Option<**bool**> | Include the demolink hashes | [optional]
**name** | **String** | Name of the ad | 
**priority** | Option<**i32**> | Priority | [optional]
**source_ad_id** | Option<[**models::AdSourceAdId**](Ad_source_ad_id.md)> |  | [optional]
**status** | Option<**String**> | Only ACTIVE and PAUSED are valid during creation. Other statuses can be used for update | [optional]
**tracking_specs** | Option<[**serde_json::Value**](.md)> | With Tracking Specs, you log actions taken by people on your ad. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


