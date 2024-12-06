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

/// AdsetBudgets : Adset Budgets
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdsetBudgets {
	/// Adset ID
	#[serde(rename = "adset_id", skip_serializing_if = "Option::is_none")]
	pub adset_id: Option<String>,
	/// Daily budget
	#[serde(rename = "daily_budget", skip_serializing_if = "Option::is_none")]
	pub daily_budget: Option<i32>,
	/// Lifetime budget
	#[serde(rename = "lifetime_budget", skip_serializing_if = "Option::is_none")]
	pub lifetime_budget: Option<i32>,
}

impl AdsetBudgets {
	/// Adset Budgets
	pub fn new() -> AdsetBudgets {
		AdsetBudgets {
			adset_id: None,
			daily_budget: None,
			lifetime_budget: None,
		}
	}
}
