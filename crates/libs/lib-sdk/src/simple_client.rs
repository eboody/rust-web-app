use crate::prelude::*;
use async_trait::async_trait;
use reqwest::{
	header::{HeaderMap, HeaderValue, CONTENT_TYPE},
	Method, Response,
};
use serde::Serialize;
use url::Url;

use crate::http_client::HttpClient;

pub struct SimpleClient {
	client: reqwest::Client,
	base_url: Url,
}

impl SimpleClient {
	pub fn new(base_url: Url) -> Self {
		let client = reqwest::Client::builder()
			.default_headers(Self::default_headers())
			.build()
			.expect("Failed to build client");

		SimpleClient { client, base_url }
	}

	fn default_headers() -> HeaderMap {
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers
	}
}

#[async_trait]
impl HttpClient for SimpleClient {
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

//To create API services that work with any client, use the HttpClient trait as a bound.
//
//```rust
//pub struct ApiService<C: HttpClient> {
//    client: C,
//}
//
//impl<C: HttpClient> ApiService<C> {
//    pub fn new(client: C) -> Self {
//        Self { client }
//    }
//
//    pub async fn get_all_pages<T: DeserializeOwned>(
//        &self,
//        endpoint: &str,
//    ) -> Result<Vec<T>> {
//        let mut results = Vec::new();
//        let mut next_url = Some(endpoint.to_string());
//
//        while let Some(url) = next_url {
//            let response = self
//                .client
//                .send_request::<(), ()>(Method::GET, &url, None, None)
//                .await?;
//            let paginated_response: PaginatedResponse<T> = response.json().await?;
//            results.extend(paginated_response.results);
//            next_url = paginated_response.next;
//        }
//
//        Ok(results)
//    }
//}
//```
