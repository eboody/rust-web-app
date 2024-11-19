use async_trait::async_trait;
use reqwest::{Method, Response};
use serde::Serialize;
use url::Url;

use crate::prelude::*;

/// Trait representing a generalized client.
#[async_trait]
pub trait HttpClient: Send + Sync {
	/// Sends an HTTP request with optional body and parameters.
	async fn send_request<T, P>(
		&self,
		method: Method,
		endpoint: &str,
		body: Option<&T>,
		params: Option<&P>,
	) -> Result<Response>
	where
		T: Serialize + Send + ?Sized + Sync,
		P: Serialize + Send + ?Sized + Sync;

	/// Constructs the base URL for the client.
	fn base_url(&self) -> &Url;
}
