use crate::prelude::*;
use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::header::HeaderMap;
use serde::Serialize;

pub use crate::config::directus_config;

/// Trait for query parameter types
pub trait Params: Serialize + Default + Debug {}

/// Blanket implementation for any type that implements `Serialize` and `Default`.
impl<T: Serialize + Default + Debug> Params for T {}

#[async_trait]
pub trait Client<T: Resource> {
	fn base_url(&self) -> &'static str;

	fn get_auth_headers(&self) -> HeaderMap;

	fn reqwest_client(&self) -> &reqwest::Client;

	async fn get(&self) -> Result<reqwest::Response>;
}
