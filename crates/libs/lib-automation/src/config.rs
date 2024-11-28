use axum::http::{HeaderMap, HeaderValue};
use lib_utils::envs::get_env;
use reqwest::header::AUTHORIZATION;
use std::sync::OnceLock;

pub fn config() -> &'static AutomationConfig {
	static INSTANCE: OnceLock<AutomationConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		AutomationConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct AutomationConfig {
	pub DIRECTUS_URL: String,
	pub DIRECTUS_DB: String,
	pub DIRECTUS_AUTH_HEADERS: HeaderMap,
	pub DIRECTUS_TOKEN: String,
	pub ANYTHINGLLM_URL: String,
	pub ANYTHING_AUTH: HeaderMap,
	pub ANYTHINGLLM_KEY: String,
}

impl AutomationConfig {
	fn load_from_env() -> lib_utils::envs::Result<AutomationConfig> {
		let mut directus_auth_headers = HeaderMap::new();
		let token = get_env("DIRECTUS_TOKEN")?;

		directus_auth_headers.insert(
			AUTHORIZATION,
			HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
		);

		let mut anything_headers = HeaderMap::new();

		anything_headers.insert(
			AUTHORIZATION,
			HeaderValue::from_str(&format!(
				"Bearer {}",
				get_env("ANYTHINGLLM_KEY")?
			))
			.unwrap(),
		);

		Ok(AutomationConfig {
			DIRECTUS_URL: get_env("DIRECTUS_URL")?,
			DIRECTUS_DB: get_env("DIRECTUS_DB")?,
			DIRECTUS_AUTH_HEADERS: directus_auth_headers,
			DIRECTUS_TOKEN: get_env("DIRECTUS_TOKEN")?,
			ANYTHINGLLM_URL: get_env("ANYTHINGLLM_URL")?,
			ANYTHING_AUTH: anything_headers,
			ANYTHINGLLM_KEY: get_env("ANYTHINGLLM_KEY")?,
		})
	}
}
