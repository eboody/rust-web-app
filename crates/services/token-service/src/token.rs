use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct TokenResponse {
	data: TokenData,
}

#[derive(Deserialize)]
#[allow(unused)]
struct TokenData {
	access_token: String,
	expires: u64,
	refresh_token: String,
}

#[derive(Clone)]
pub struct TokenClient {
	client: ReqwestClient,
	email: String,
	password: String,
	token_url: String,
}

impl TokenClient {
	pub fn new() -> Self {
		let email = env::var("DIRECTUS_EMAIL").expect("DIRECTUS_EMAIL must be set");
		let password =
			env::var("DIRECTUS_PASSWORD").expect("DIRECTUS_PASSWORD must be set");
		let api_url =
			env::var("DIRECTUS_API_URL").expect("DIRECTUS_API_URL must be set");
		let token_url = format!("{api_url}/auth/login");

		TokenClient {
			client: ReqwestClient::new(),
			email,
			password,
			token_url,
		}
	}

	pub async fn fetch_token(&self) -> Result<String, reqwest::Error> {
		let credentials = json::json!({
			"email": self.email,
			"password": self.password,
		});

		let response = self
			.client
			.post(&self.token_url)
			.json(&credentials)
			.send()
			.await?
			.json::<TokenResponse>()
			.await?;

		Ok(response.data.access_token)
	}
}
