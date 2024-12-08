# Rust API client for openapi

This is a generated connector for [Facebook Marketing API v12.0](https://developers.facebook.com/docs/marketing-apis) OpenAPI specification. 
Facebook is an American online social media and social networking service owned by Facebook, Inc.Facebook Marketing  APIs are a collection of Graph API endpoints that can be used to help you advertise on Facebook.



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v12.0
- Package version: v12.0
- Generator version: 7.10.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://graph.facebook.com/v12.0*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AdApi* | [**create_ad**](docs/AdApi.md#create_ad) | **POST** /act_{ad_account_id}/ads | 
*AdApi* | [**delete_ad**](docs/AdApi.md#delete_ad) | **DELETE** /{ad_id} | 
*AdApi* | [**get_ad**](docs/AdApi.md#get_ad) | **GET** /{ad_id} | 
*AdApi* | [**get_ads**](docs/AdApi.md#get_ads) | **GET** /act_{ad_account_id}/ads | 
*AdApi* | [**update_ad**](docs/AdApi.md#update_ad) | **POST** /{ad_id} | 
*AdSetApi* | [**create_ad_set**](docs/AdSetApi.md#create_ad_set) | **POST** /act_{ad_account_id}/adsets | 
*AdSetApi* | [**delete_ad_set**](docs/AdSetApi.md#delete_ad_set) | **DELETE** /{ad_set_id} | 
*AdSetApi* | [**get_ad_set**](docs/AdSetApi.md#get_ad_set) | **GET** /{ad_set_id} | 
*AdSetApi* | [**get_ad_sets**](docs/AdSetApi.md#get_ad_sets) | **GET** /act_{ad_account_id}/adsets | 
*AdSetApi* | [**update_ad_set**](docs/AdSetApi.md#update_ad_set) | **POST** /{ad_set_id} | 
*CampaignApi* | [**create_campaign**](docs/CampaignApi.md#create_campaign) | **POST** /act_{ad_account_id}/campaigns | 
*CampaignApi* | [**delete_campaign**](docs/CampaignApi.md#delete_campaign) | **DELETE** /{campaign_id} | 
*CampaignApi* | [**dissociate_campaign**](docs/CampaignApi.md#dissociate_campaign) | **DELETE** /act_{ad_account_id}/campaigns | 
*CampaignApi* | [**get_campaigns**](docs/CampaignApi.md#get_campaigns) | **GET** /act_{ad_account_id}/campaigns | 
*CampaignApi* | [**update_campaign**](docs/CampaignApi.md#update_campaign) | **POST** /{campaign_id} | 


## Documentation For Models

 - [Ad](docs/Ad.md)
 - [AdList](docs/AdList.md)
 - [AdResponse](docs/AdResponse.md)
 - [AdSet](docs/AdSet.md)
 - [AdSetCampaignId](docs/AdSetCampaignId.md)
 - [AdSetList](docs/AdSetList.md)
 - [AdSetResponse](docs/AdSetResponse.md)
 - [AdSetSchedule](docs/AdSetSchedule.md)
 - [AdSetSourceAdsetId](docs/AdSetSourceAdsetId.md)
 - [AdSetUpdate](docs/AdSetUpdate.md)
 - [AdSetUpdateCreativeSequenceInner](docs/AdSetUpdateCreativeSequenceInner.md)
 - [AdSetUpdateRbPredictionId](docs/AdSetUpdateRbPredictionId.md)
 - [AdSetUpdateRfPredictionId](docs/AdSetUpdateRfPredictionId.md)
 - [AdSourceAdId](docs/AdSourceAdId.md)
 - [AdUpdate](docs/AdUpdate.md)
 - [AdUpdateDraftAdgroupId](docs/AdUpdateDraftAdgroupId.md)
 - [AdsetBudgets](docs/AdsetBudgets.md)
 - [AttributionSpec](docs/AttributionSpec.md)
 - [Campaign](docs/Campaign.md)
 - [CampaignDissociateResponse](docs/CampaignDissociateResponse.md)
 - [CampaignList](docs/CampaignList.md)
 - [CampaignResponse](docs/CampaignResponse.md)
 - [CampaignSourceCampaignId](docs/CampaignSourceCampaignId.md)
 - [CampaignToplineId](docs/CampaignToplineId.md)
 - [CampaignUpdate](docs/CampaignUpdate.md)
 - [CampignSpec](docs/CampignSpec.md)
 - [ContextualBundlingSpec](docs/ContextualBundlingSpec.md)
 - [ListSummary](docs/ListSummary.md)
 - [OmnichannelObject](docs/OmnichannelObject.md)
 - [Paging](docs/Paging.md)
 - [PagingCursors](docs/PagingCursors.md)
 - [PromotedObject](docs/PromotedObject.md)
 - [PromotedObjectEventId](docs/PromotedObjectEventId.md)
 - [PromotedObjectFundraiserCampaignId](docs/PromotedObjectFundraiserCampaignId.md)
 - [PromotedObjectInstagramProfileId](docs/PromotedObjectInstagramProfileId.md)
 - [PromotedObjectOfferId](docs/PromotedObjectOfferId.md)
 - [PromotedObjectOfflineConversionDataSetId](docs/PromotedObjectOfflineConversionDataSetId.md)
 - [PromotedObjectPixelId](docs/PromotedObjectPixelId.md)
 - [PromotedObjectProductCatalogId](docs/PromotedObjectProductCatalogId.md)
 - [PromotedObjectProductItemId](docs/PromotedObjectProductItemId.md)
 - [PromotedObjectProductSetId](docs/PromotedObjectProductSetId.md)
 - [TimeRange](docs/TimeRange.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



