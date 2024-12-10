use axum::http::{HeaderMap, HeaderValue};
use lib_utils::envs::get_env;
use reqwest::header::AUTHORIZATION;
use std::sync::OnceLock;
use uuid::Uuid;

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
	pub DIRECTUS_HEADERS: HeaderMap,
	pub DIRECTUS_HEADERS_JSON: HeaderMap,
	pub DIRECTUS_TOKEN: String,
	pub ANYTHINGLLM_URL: String,
	pub ANYTHING_HEADERS: HeaderMap,
	pub ANYTHING_HEADERS_JSON: HeaderMap,
	pub ANYTHINGLLM_KEY: String,
	pub OPENAI_API_KEY: String,

	pub ARTICLES_IMAGES_FOLDER_ID: Uuid,
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

		let mut anything_headers_json = anything_headers.clone();
		anything_headers_json
			.insert("Content-Type", HeaderValue::from_static("application/json"));

		let mut directus_headers_json = directus_auth_headers.clone();
		directus_headers_json
			.insert("Content-Type", HeaderValue::from_static("application/json"));

		Ok(AutomationConfig {
			DIRECTUS_URL: get_env("DIRECTUS_URL")?,
			DIRECTUS_DB: get_env("DIRECTUS_DB")?,
			DIRECTUS_HEADERS: directus_auth_headers,
			DIRECTUS_HEADERS_JSON: directus_headers_json,
			DIRECTUS_TOKEN: get_env("DIRECTUS_TOKEN")?,
			ANYTHINGLLM_URL: get_env("ANYTHINGLLM_URL")?,
			ANYTHING_HEADERS: anything_headers,
			ANYTHING_HEADERS_JSON: anything_headers_json,
			ANYTHINGLLM_KEY: get_env("ANYTHINGLLM_KEY")?,
			OPENAI_API_KEY: get_env("OPENAI_API_KEY")?,
			ARTICLES_IMAGES_FOLDER_ID: Uuid::parse_str(&get_env(
				"ARTICLES_IMAGES_FOLDER_ID",
			)?)
			.unwrap(),
		})
	}
}
