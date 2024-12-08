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

/// PromotedObject : The object this campaign is promoting across all its ads
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromotedObject {
    /// The ID of a Facebook Application. Usually related to mobile or canvas games being promoted on Facebook for installs or engagement
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<i32>,
    #[serde(rename = "pixel_id", skip_serializing_if = "Option::is_none")]
    pub pixel_id: Option<Box<models::PromotedObjectPixelId>>,
    /// The event from an App Event of a mobile app, not in the standard event list
    #[serde(rename = "custom_event_type", skip_serializing_if = "Option::is_none")]
    pub custom_event_type: Option<CustomEventType>,
    /// The uri of the mobile / digital store where an application can be bought / downloaded
    #[serde(rename = "object_store_url", skip_serializing_if = "Option::is_none")]
    pub object_store_url: Option<String>,
    #[serde(rename = "offer_id", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<Box<models::PromotedObjectOfferId>>,
    /// The ID of a Facebook Page
    #[serde(rename = "page_id", skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    #[serde(rename = "product_catalog_id", skip_serializing_if = "Option::is_none")]
    pub product_catalog_id: Option<Box<models::PromotedObjectProductCatalogId>>,
    #[serde(rename = "product_item_id", skip_serializing_if = "Option::is_none")]
    pub product_item_id: Option<Box<models::PromotedObjectProductItemId>>,
    #[serde(rename = "instagram_profile_id", skip_serializing_if = "Option::is_none")]
    pub instagram_profile_id: Option<Box<models::PromotedObjectInstagramProfileId>>,
    #[serde(rename = "product_set_id", skip_serializing_if = "Option::is_none")]
    pub product_set_id: Option<Box<models::PromotedObjectProductSetId>>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<Box<models::PromotedObjectEventId>>,
    #[serde(rename = "offline_conversion_data_set_id", skip_serializing_if = "Option::is_none")]
    pub offline_conversion_data_set_id: Option<Box<models::PromotedObjectOfflineConversionDataSetId>>,
    #[serde(rename = "fundraiser_campaign_id", skip_serializing_if = "Option::is_none")]
    pub fundraiser_campaign_id: Option<Box<models::PromotedObjectFundraiserCampaignId>>,
    /// The event from an App Event of a mobile app, not in the standard event list.
    #[serde(rename = "custom_event_str", skip_serializing_if = "Option::is_none")]
    pub custom_event_str: Option<String>,
    #[serde(rename = "omnichannel_object", skip_serializing_if = "Option::is_none")]
    pub omnichannel_object: Option<Box<models::OmnichannelObject>>,
    /// Post conversions
    #[serde(rename = "post_conversions", skip_serializing_if = "Option::is_none")]
    pub post_conversions: Option<Vec<Vec<serde_json::Value>>>,
}

impl PromotedObject {
    /// The object this campaign is promoting across all its ads
    pub fn new() -> PromotedObject {
        PromotedObject {
            application_id: None,
            pixel_id: None,
            custom_event_type: None,
            object_store_url: None,
            offer_id: None,
            page_id: None,
            product_catalog_id: None,
            product_item_id: None,
            instagram_profile_id: None,
            product_set_id: None,
            event_id: None,
            offline_conversion_data_set_id: None,
            fundraiser_campaign_id: None,
            custom_event_str: None,
            omnichannel_object: None,
            post_conversions: None,
        }
    }
}
/// The event from an App Event of a mobile app, not in the standard event list
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CustomEventType {
    #[serde(rename = "RATE")]
    Rate,
    #[serde(rename = "TUTORIAL_COMPLETION")]
    TutorialCompletion,
    #[serde(rename = "CONTACT")]
    Contact,
    #[serde(rename = "CUSTOMIZE_PRODUCT")]
    CustomizeProduct,
    #[serde(rename = "DONATE")]
    Donate,
    #[serde(rename = "FIND_LOCATION")]
    FindLocation,
    #[serde(rename = "SCHEDULE")]
    Schedule,
    #[serde(rename = "START_TRIAL")]
    StartTrial,
    #[serde(rename = "SUBMIT_APPLICATION")]
    SubmitApplication,
    #[serde(rename = "SUBSCRIBE")]
    Subscribe,
    #[serde(rename = "ADD_TO_CART")]
    AddToCart,
    #[serde(rename = "ADD_TO_WISHLIST")]
    AddToWishlist,
    #[serde(rename = "INITIATED_CHECKOUT")]
    InitiatedCheckout,
    #[serde(rename = "ADD_PAYMENT_INFO")]
    AddPaymentInfo,
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "LEAD")]
    Lead,
    #[serde(rename = "COMPLETE_REGISTRATION")]
    CompleteRegistration,
    #[serde(rename = "CONTENT_VIEW")]
    ContentView,
    #[serde(rename = "SEARCH")]
    Search,
    #[serde(rename = "SERVICE_BOOKING_REQUEST")]
    ServiceBookingRequest,
    #[serde(rename = "MESSAGING_CONVERSATION_STARTED_7D")]
    MessagingConversationStarted7D,
    #[serde(rename = "LEVEL_ACHIEVED")]
    LevelAchieved,
    #[serde(rename = "ACHIEVEMENT_UNLOCKED")]
    AchievementUnlocked,
    #[serde(rename = "SPENT_CREDITS")]
    SpentCredits,
    #[serde(rename = "LISTING_INTERACTION")]
    ListingInteraction,
    #[serde(rename = "D2_RETENTION")]
    D2Retention,
    #[serde(rename = "D7_RETENTION")]
    D7Retention,
    #[serde(rename = "OTHER")]
    Other,
}

impl Default for CustomEventType {
    fn default() -> CustomEventType {
        Self::Rate
    }
}

