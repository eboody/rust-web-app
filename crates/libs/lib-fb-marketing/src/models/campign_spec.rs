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

/// CampignSpec : Campaign specifications
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampignSpec {
	/// Name of the campaign
	#[serde(rename = "name", skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Campaign's objective. If it is specified the API will validate that any ads created under the campaign match that objective.
	#[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
	pub objective: Option<Objective>,
	/// This field will help Facebook make optimizations to delivery, pricing, and limits. All ad sets in this campaign must match the buying type.  Possible values are   - AUCTION (default)   - RESERVED (for reach and frequency ads)
	#[serde(rename = "buying_type", skip_serializing_if = "Option::is_none")]
	pub buying_type: Option<BuyingType>,
}

impl CampignSpec {
	/// Campaign specifications
	pub fn new() -> CampignSpec {
		CampignSpec {
			name: None,
			objective: None,
			buying_type: None,
		}
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
