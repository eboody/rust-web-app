use crate::prelude::*;
use async_trait::async_trait;
use reqwest::{
	header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
	Method, Response,
};
use serde::Serialize;
use url::Url;

use crate::http_client::HttpClient;

pub struct TokenClient {
	client: reqwest::Client,
	access_token: String,
	base_url: Url,
}

impl TokenClient {
	pub fn new(base_url: Url, access_token: String) -> Self {
		let client = reqwest::Client::builder()
			.default_headers(Self::default_headers(&access_token))
			.build()
			.expect("Failed to build client");

		TokenClient {
			client,
			access_token,
			base_url,
		}
	}

	fn default_headers(access_token: &str) -> HeaderMap {
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert(
			AUTHORIZATION,
			HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
		);
		headers
	}
}

#[async_trait]
impl HttpClient for TokenClient {
	async fn send_request<T, P>(
		&self,
		method: Method,
		endpoint: &str,
		body: Option<&T>,
		params: Option<&P>,
	) -> Result<Response>
	where
		T: Serialize + ?Sized + Send + Sync,
		P: Serialize + ?Sized + Send + Sync,
	{
		let mut url = self.base_url.join(endpoint)?;

		if let Some(params) = params {
			let query = serde_urlencoded::to_string(params)?;
			url.set_query(Some(&query));
		}

		let mut request_builder = self.client.request(method, url);

		if let Some(body) = body {
			request_builder = request_builder.json(body);
		}

		let response = request_builder.send().await?;
		Ok(response)
	}

	fn base_url(&self) -> &Url {
		&self.base_url
	}
}
