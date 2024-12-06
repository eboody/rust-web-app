# AdSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adlabels** | Option<[**Vec<json::Value>**](json::Value.md)> | Specifies list of labels to be associated with this object. This field is optional | [optional]
**adset_schedule** | Option<[**Vec<models::AdSetSchedule>**](AdSetSchedule.md)> | Ad set schedule, representing a delivery schedule for a single day | [optional]
**attribution_spec** | Option<[**Vec<models::AttributionSpec>**](AttributionSpec.md)> | Array of attribution specs | [optional]
**bid_amount** | Option<**i32**> | Bid cap or target cost for this ad set. The bid cap used in a lowest cost bid strategy is defined as the maximum bid you want to pay for a result based on your optimization_goal. | [optional]
**bid_strategy** | Option<**String**> | Bid strategy for this campaign to suit your specific business goals | [optional]
**billing_event** | Option<**String**> | The billing event | [optional]
**campaign_id** | Option<[**models::AdSetCampaignId**](AdSet_campaign_id.md)> |  | [optional]
**campaign_spec** | Option<[**models::CampignSpec**](CampignSpec.md)> |  | [optional]
**contextual_bundling_spec** | Option<[**models::ContextualBundlingSpec**](ContextualBundlingSpec.md)> |  | [optional]
**creative_sequence** | Option<[**Vec<models::AdSetUpdateCreativeSequenceInner>**](AdSetUpdate_creative_sequence_inner.md)> | Order of the adgroup sequence to be shown to users | [optional]
**daily_budget** | Option<**i32**> | The daily budget defined in your account currency, allowed only for ad sets with a duration (difference between end_time and start_time) longer than 24 hours. | [optional]
**daily_imps** | Option<**i32**> | Daily impressions. Available only for campaigns with buying_type=FIXED_CPM. | [optional]
**daily_min_spend_target** | Option<**i32**> | Daily minimum spend target of the ad set defined in your account currency. To use this field, daily budget must be specified in the Campaign. | [optional]
**daily_spend_cap** | Option<**i32**> | Daily spend cap of the ad set defined in your account currency. To use this field, daily budget must be specified in the Campaign. | [optional]
**destination_type** | Option<**String**> | The billing event | [optional]
**end_time** | Option<**String**> | End time, required when lifetime_budget is specified | [optional]
**execution_options** | Option<**Vec<String>**> | An execution setting | [optional]
**existing_customer_budget_percentage** | Option<**i32**> | Existing customer budget percentage | [optional]
**is_dynamic_creative** | Option<**bool**> | Indicates the ad set must only be used for dynamic creatives. Dynamic creative ads can be created in this ad set. Defaults to false | [optional][default to false]
**lifetime_budget** | Option<**i32**> | Lifetime budget of this campaign. All adsets under this campaign will share this budget.  You can either set budget at the campaign level or at the adset level, not both.   | [optional]
**lifetime_imps** | Option<**i32**> | Lifetime impressions. Available only for campaigns with buying_type=FIXED_CPM | [optional]
**lifetime_min_spend_target** | Option<**i32**> | Lifetime minimum spend target of the ad set defined in your account currency. To use this field, lifetime budget must be specified in the Campaign. | [optional]
**lifetime_spend_cap** | Option<**i32**> | Lifetime spend cap of the ad set defined in your account currency. To use this field, lifetime budget must be specified in the Campaign. | [optional]
**multi_optimization_goal_weight** | Option<**String**> | Multi optimization goal weight | [optional]
**name** | **String** | Ad set name, max length of 400 characters. | 
**optimization_goal** | Option<**String**> | What the ad set is optimizing for. | [optional]
**optimization_sub_event** | Option<**String**> | What the ad set is optimizing for. | [optional]
**pacing_type** | Option<**Vec<String>**> | Defines the pacing type, standard by default or using ad scheduling | [optional]
**rf_prediction_id** | Option<[**models::AdSetUpdateRfPredictionId**](AdSetUpdate_rf_prediction_id.md)> |  | [optional]
**source_adset_id** | Option<[**models::AdSetSourceAdsetId**](AdSet_source_adset_id.md)> |  | [optional]
**promoted_object** | Option<[**models::PromotedObject**](PromotedObject.md)> |  | [optional]
**start_time** | Option<**String**> | The start time of the set | [optional]
**status** | Option<**String**> | Only ACTIVE and PAUSED are valid during creation. Other statuses can be used for update | [optional]
**targeting** | Option<[**json::Value**](.md)> | An ad set's targeting structure. \"countries\" is required. | [optional]
**time_based_ad_rotation_id_blocks** | Option<[**Vec<Vec<i32>>**](Vec.md)> | Specify ad creative that displays at custom date ranges in a campaign as an array. A list of Adgroup IDs | [optional]
**time_based_ad_rotation_intervals** | Option<**Vec<i32>**> | Date range when specific ad creative displays during a campaign. Provide date ranges in an array of UNIX timestamps where each timestamp represents the start time for each date range. | [optional]
**time_start** | Option<**String**> | Time start | [optional]
**time_stop** | Option<**String**> | Time stop | [optional]
**tune_for_category** | Option<**String**> | Tune for category | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


