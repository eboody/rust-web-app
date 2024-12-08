/*
 * Facebook Marketing API
 *
 * This is a generated connector for [Facebook Marketing API v12.0](https://developers.facebook.com/docs/marketing-apis) OpenAPI specification.  Facebook is an American online social media and social networking service owned by Facebook, Inc.Facebook Marketing  APIs are a collection of Graph API endpoints that can be used to help you advertise on Facebook.
 *
 * The version of the OpenAPI document: v12.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Campaign : Campaign properties
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Campaign {
	/// Ad Labels associated with this campaign
	#[serde(rename = "adlabels", skip_serializing_if = "Option::is_none")]
	pub adlabels: Option<Vec<serde_json::Value>>,
	/// This field will help Facebook make optimizations to delivery, pricing, and limits. All ad sets in this campaign must match the buying type.  Possible values are   - AUCTION (default)   - RESERVED (for reach and frequency ads)
	#[serde(rename = "buying_type", skip_serializing_if = "Option::is_none")]
	pub buying_type: Option<BuyingType>,
	/// Bid strategy for this campaign to suit your specific business goals
	#[serde(rename = "bid_strategy", skip_serializing_if = "Option::is_none")]
	pub bid_strategy: Option<BidStrategy>,
	/// Campaign optimization type
	#[serde(
		rename = "campaign_optimization_type",
		skip_serializing_if = "Option::is_none"
	)]
	pub campaign_optimization_type: Option<CampaignOptimizationType>,
	/// Daily budget of this campaign. All adsets under this campaign will share this budget.
	#[serde(rename = "daily_budget", skip_serializing_if = "Option::is_none")]
	pub daily_budget: Option<i32>,
	/// An execution setting
	#[serde(rename = "execution_options", skip_serializing_if = "Option::is_none")]
	pub execution_options: Option<Vec<ExecutionOptions>>,
	/// To create an iOS 14 campaign, enable SKAdNetwork attribution for this campaign
	#[serde(
		rename = "is_skadnetwork_attribution",
		skip_serializing_if = "Option::is_none"
	)]
	pub is_skadnetwork_attribution: Option<bool>,
	/// Is using l3 schedule
	#[serde(
		rename = "is_using_l3_schedule",
		skip_serializing_if = "Option::is_none"
	)]
	pub is_using_l3_schedule: Option<bool>,
	/// Array of Iterative Split Test Configs created under this campaign
	#[serde(
		rename = "iterative_split_test_configs",
		skip_serializing_if = "Option::is_none"
	)]
	pub iterative_split_test_configs: Option<Vec<String>>,
	/// Lifetime budget of this campaign. All adsets under this campaign will share this budget.  You can either set budget at the campaign level or at the adset level, not both.  
	#[serde(rename = "lifetime_budget", skip_serializing_if = "Option::is_none")]
	pub lifetime_budget: Option<i32>,
	/// Name for this campaign
	#[serde(rename = "name", skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Campaign's objective. If it is specified the API will validate that any ads created under the campaign match that objective.
	#[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
	pub objective: Option<Objective>,
	#[serde(rename = "promoted_object", skip_serializing_if = "Option::is_none")]
	pub promoted_object: Option<Box<models::PromotedObject>>,
	#[serde(rename = "source_campaign_id", skip_serializing_if = "Option::is_none")]
	pub source_campaign_id: Option<Box<models::CampaignSourceCampaignId>>,
	/// Special ad categories
	#[serde(rename = "special_ad_categories")]
	pub special_ad_categories: Vec<SpecialAdCategories>,
	/// Special ad category country
	#[serde(
		rename = "special_ad_category_country",
		skip_serializing_if = "Option::is_none"
	)]
	pub special_ad_category_country: Option<String>,
	/// A spend cap for the campaign, such that it will not spend more than this cap. Defined as integer value of  subunit in your currency with a minimum value of $100 USD (or approximate local equivalent).  
	#[serde(rename = "spend_cap", skip_serializing_if = "Option::is_none")]
	pub spend_cap: Option<i32>,
	/// Start time
	#[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
	pub start_time: Option<String>,
	/// Stop time
	#[serde(rename = "stop_time", skip_serializing_if = "Option::is_none")]
	pub stop_time: Option<String>,
	/// Only ACTIVE and PAUSED are valid during creation. Other statuses can be used for update
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status: Option<Status>,
	#[serde(rename = "topline_id", skip_serializing_if = "Option::is_none")]
	pub topline_id: Option<Box<models::CampaignToplineId>>,
	/// Upstream events
	#[serde(rename = "upstream_events", skip_serializing_if = "Option::is_none")]
	pub upstream_events: Option<serde_json::Value>,
}

impl Campaign {
	/// Campaign properties
	pub fn new(special_ad_categories: Vec<SpecialAdCategories>) -> Campaign {
		Campaign {
			adlabels: None,
			buying_type: None,
			bid_strategy: None,
			campaign_optimization_type: None,
			daily_budget: None,
			execution_options: None,
			is_skadnetwork_attribution: None,
			is_using_l3_schedule: None,
			iterative_split_test_configs: None,
			lifetime_budget: None,
			name: None,
			objective: None,
			promoted_object: None,
			source_campaign_id: None,
			special_ad_categories,
			special_ad_category_country: None,
			spend_cap: None,
			start_time: None,
			stop_time: None,
			status: None,
			topline_id: None,
			upstream_events: None,
		}
	}
	pub fn to_string(&self) -> String {
		serde_json::to_string(self).expect("to_string failed")
	}
}
/// This field will help Facebook make optimizations to delivery, pricing, and limits. All ad sets in this campaign must match the buying type.  Possible values are   - AUCTION (default)   - RESERVED (for reach and frequency ads)
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum BuyingType {
	#[serde(rename = "AUCTION")]
	Auction,
	#[serde(rename = "RESERVED")]
	Reserved,
}

impl Default for BuyingType {
	fn default() -> BuyingType {
		Self::Auction
	}
}
/// Bid strategy for this campaign to suit your specific business goals
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum BidStrategy {
	#[serde(rename = "LOWEST_COST_WITHOUT_CAP")]
	LowestCostWithoutCap,
	#[serde(rename = "LOWEST_COST_WITH_BID_CAP")]
	LowestCostWithBidCap,
	#[serde(rename = "COST_CAP")]
	CostCap,
}

impl Default for BidStrategy {
	fn default() -> BidStrategy {
		Self::LowestCostWithoutCap
	}
}
/// Campaign optimization type
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum CampaignOptimizationType {
	#[serde(rename = "NONE")]
	None,
	#[serde(rename = "ICO_ONLY")]
	IcoOnly,
}

impl Default for CampaignOptimizationType {
	fn default() -> CampaignOptimizationType {
		Self::None
	}
}
/// An execution setting
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ExecutionOptions {
	#[serde(rename = "validate_only")]
	ValidateOnly,
	#[serde(rename = "include_recommendations")]
	IncludeRecommendations,
}

impl Default for ExecutionOptions {
	fn default() -> ExecutionOptions {
		Self::ValidateOnly
	}
}
/// Campaign's objective. If it is specified the API will validate that any ads created under the campaign match that objective.
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Objective {
	#[serde(rename = "APP_INSTALLS")]
	AppInstalls,
	#[serde(rename = "BRAND_AWARENESS")]
	BrandAwareness,
	#[serde(rename = "CONVERSIONS")]
	Conversions,
	#[serde(rename = "EVENT_RESPONSES")]
	EventResponses,
	#[serde(rename = "LEAD_GENERATION")]
	LeadGeneration,
	#[serde(rename = "LINK_CLICKS")]
	LinkClicks,
	#[serde(rename = "LOCAL_AWARENESS")]
	LocalAwareness,
	#[serde(rename = "MESSAGES")]
	Messages,
	#[serde(rename = "OFFER_CLAIMS")]
	OfferClaims,
	#[serde(rename = "PAGE_LIKES")]
	PageLikes,
	#[serde(rename = "POST_ENGAGEMENT")]
	PostEngagement,
	#[serde(rename = "PRODUCT_CATALOG_SALES")]
	ProductCatalogSales,
	#[serde(rename = "REACH")]
	Reach,
	#[serde(rename = "STORE_VISITS")]
	StoreVisits,
	#[serde(rename = "VIDEO_VIEWS")]
	VideoViews,
}

impl Default for Objective {
	fn default() -> Objective {
		Self::AppInstalls
	}
}
/// Special ad categories
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum SpecialAdCategories {
	#[serde(rename = "NONE")]
	None,
	#[serde(rename = "EMPLOYMENT")]
	Employment,
	#[serde(rename = "HOUSING")]
	Housing,
	#[serde(rename = "CREDIT")]
	Credit,
	#[serde(rename = "ISSUES_ELECTIONS_POLITICS")]
	IssuesElectionsPolitics,
	#[serde(rename = "ONLINE_GAMBLING_AND_GAMING")]
	OnlineGamblingAndGaming,
}

impl Default for SpecialAdCategories {
	fn default() -> SpecialAdCategories {
		Self::None
	}
}
/// Only ACTIVE and PAUSED are valid during creation. Other statuses can be used for update
#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Status {
	#[serde(rename = "ACTIVE")]
	Active,
	#[serde(rename = "PAUSED")]
	Paused,
	#[serde(rename = "DELETED")]
	Deleted,
	#[serde(rename = "ARCHIVED")]
	Archived,
}

impl Default for Status {
	fn default() -> Status {
		Self::Active
	}
}
