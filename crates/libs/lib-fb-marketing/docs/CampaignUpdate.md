# CampaignUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adlabels** | Option<[**Vec<json::Value>**](json::Value.md)> | Ad Labels associated with this campaign | [optional]
**adset_bid_amounts** | Option<[**json::Value**](.md)> | A map of child adset IDs to their respective bid amounts required in the process of toggling campaign from autobid to manual bid | [optional]
**adset_budgets** | Option<[**models::AdsetBudgets**](AdsetBudgets.md)> |  | [optional]
**bid_strategy** | Option<**String**> | Bid strategy for this campaign to suit your specific business goals | [optional]
**budget_rebalance_flag** | Option<**bool**> | Whether to automatically rebalance budgets daily for all the adsets under this campaign | [optional]
**campaign_optimization_type** | Option<**String**> | Campaign optimization type | [optional]
**daily_budget** | Option<**i32**> | Daily budget of this campaign. All adsets under this campaign will share this budget. | [optional]
**execution_options** | Option<**Vec<String>**> | An execution setting | [optional]
**is_skadnetwork_attribution** | Option<**bool**> | To create an iOS 14 campaign, enable SKAdNetwork attribution for this campaign | [optional]
**is_using_l3_schedule** | Option<**bool**> | Is using l3 schedule | [optional]
**iterative_split_test_configs** | Option<**Vec<String>**> | An Array of Iterative Split Test Configs created under this campaign | [optional]
**lifetime_budget** | Option<**i32**> | Lifetime budget of this campaign. All adsets under this campaign will share this budget.  You can either set budget at the campaign level or at the adset level, not both.   | [optional]
**name** | Option<**String**> | Name for this campaign | [optional]
**objective** | Option<**String**> | Campaign's objective. If it is specified the API will validate that any ads created under the campaign match that objective. | [optional]
**promoted_object** | Option<[**models::PromotedObject**](PromotedObject.md)> |  | [optional]
**smart_promotion_type** | Option<**String**> | Samrt promotion type | [optional]
**special_ad_categories** | **Vec<String>** | Special ad categories | 
**special_ad_category_country** | Option<**String**> | Special ad category country | [optional]
**spend_cap** | Option<**i32**> | A spend cap for the campaign, such that it will not spend more than this cap. Defined as integer value of  subunit in your currency with a minimum value of $100 USD (or approximate local equivalent).   | [optional]
**start_time** | Option<**String**> | Start time | [optional]
**stop_time** | Option<**String**> | Stop time | [optional]
**status** | Option<**String**> | Only ACTIVE and PAUSED are valid during creation. Other statuses can be used for update | [optional]
**upstream_events** | Option<[**json::Value**](.md)> | Upstream events | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


