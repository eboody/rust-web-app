use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use strum::Display;

#[derive(Debug, Serialize)]
pub struct ChatRequest {
	pub message: String,
	pub mode: ChatMode,
	pub session_id: String,
}

#[derive(Debug, Serialize)]
pub enum ChatMode {
	#[serde(rename = "chat")]
	Chat,
	#[serde(rename = "query")]
	Query,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
	pub id: String,
	pub r#type: ResponseType,
	#[serde(rename = "textResponse")]
	pub text_response: Option<String>,
	pub sources: Vec<ChatSource>,
	pub close: bool,
	pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSource {
	pub title: String,
	pub chunk: String,
}

#[derive(Debug, Deserialize)]
pub enum ResponseType {
	#[serde(rename = "abort")]
	Abort,
	#[serde(rename = "textResponse")]
	TextResponse,
}

pub struct AnythingLLMClient {
	client: Client,
	base_url: String,
	api_key: String,
}

#[derive(Debug, Serialize)]
pub struct CreateThreadRequest {
	pub user_id: Option<i32>,
	pub name: String,
	pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct ThreadResponse {
	pub thread: ThreadDetails,
	pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadDetails {
	pub id: i32,
	pub name: String,
	pub slug: String,
	pub user_id: Option<i32>,
	pub workspace_id: i32,
}

impl AnythingLLMClient {
	pub fn new(base_url: &str, api_key: &str) -> Self {
		AnythingLLMClient {
			client: Client::new(),
			base_url: base_url.to_string(),
			api_key: api_key.to_string(),
		}
	}

	pub async fn create_thread(
		&self,
		workspace_slug: &str,
		request: CreateThreadRequest,
	) -> Result<ThreadResponse, Box<dyn Error>> {
		let url = format!(
			"{}/v1/workspace/{}/thread/new",
			self.base_url, workspace_slug
		);

		let response = self
			.client
			.post(&url)
			.header("accept", "application/json")
			.header("Authorization", format!("Bearer {}", self.api_key))
			.header("Content-Type", "application/json")
			.json(&request)
			.send()
			.await?;

		if response.status().is_success() {
			let thread_response = response.json::<ThreadResponse>().await?;
			Ok(thread_response)
		} else {
			let error_message = response.text().await?;
			Err(Box::new(std::io::Error::new(
				std::io::ErrorKind::Other,
				error_message,
			)))
		}
	}
	pub async fn delete_thread(
		&self,
		workspace_slug: &str,
		thread_slug: &str,
	) -> Result<ThreadResponse, Box<dyn Error>> {
		let url = format!(
			"{}/v1/workspace/{}/thread/{}",
			self.base_url, workspace_slug, thread_slug
		);

		let response = self
			.client
			.delete(&url)
			.header("accept", "application/json")
			.header("Authorization", format!("Bearer {}", self.api_key))
			.header("Content-Type", "application/json")
			.send()
			.await?;

		if response.status().is_success() {
			let thread_response = response.json::<ThreadResponse>().await?;
			Ok(thread_response)
		} else {
			let error_message = response.text().await?;
			Err(Box::new(std::io::Error::new(
				std::io::ErrorKind::Other,
				error_message,
			)))
		}
	}
	pub async fn send_chat(
		&self,
		workspace_slug: &str,
		thread_slug: &str,
		request: ChatRequest,
	) -> Result<ChatResponse, Box<dyn Error>> {
		let url = format!(
			"{}/v1/workspace/{}/thread/{}/chat",
			self.base_url, workspace_slug, thread_slug
		);

		let response = self
			.client
			.post(&url)
			.header("accept", "application/json")
			.header("Authorization", format!("Bearer {}", self.api_key))
			.header("Content-Type", "application/json")
			.json(&request)
			.send()
			.await?;

		if response.status().is_success() {
			let chat_response = response.json::<ChatResponse>().await?;
			Ok(chat_response)
		} else {
			let error_message = response.text().await?;
			Err(Box::new(std::io::Error::new(
				std::io::ErrorKind::Other,
				error_message,
			)))
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum Language {
	English,
	German,
	French,
	Portuguese,
	Spanish,
	Hindi,
	Thai,
}

pub async fn translate(
	text: String,
	language: Language,
) -> Result<String, Box<dyn Error>> {
	let client = AnythingLLMClient::new(
		"https://anythingllm.eman.network/api",
		&std::env::var("ANYTHINGLLM_API_KEY")?,
	);

	let chat_request = ChatRequest {
		message: format!("Please translate this into {}:\n{}", language, text),
		mode: ChatMode::Chat,
		session_id: "api-translation".to_string(),
	};
	dbg!("{}", serde_json::to_string(&chat_request).unwrap());

	let metadata_workspace_slug = "article-metadata-translator";

	let new_thread_params = CreateThreadRequest {
		user_id: None,
		name: metadata_workspace_slug.to_string(),
		slug: metadata_workspace_slug.to_string(),
	};

	let new_thread_response = client
		.create_thread(metadata_workspace_slug, new_thread_params)
		.await?;

	match client
		.send_chat(
			metadata_workspace_slug,
			&new_thread_response.thread.slug,
			chat_request,
		)
		.await
	{
		Ok(response) => {
			client
				.delete_thread(
					metadata_workspace_slug,
					&new_thread_response.thread.slug,
				)
				.await?;
			dbg!("{}", &response);
			if let Some(text_response) = response.text_response {
				println!("Translated text: {}", text_response);
				Ok(text_response)
			} else {
				Err(Box::new(std::io::Error::new(
					std::io::ErrorKind::Other,
					"Translation failed",
				)))
			}
		}
		Err(e) => {
			client
				.delete_thread(
					metadata_workspace_slug,
					&new_thread_response.thread.slug,
				)
				.await?;
			eprintln!("Error: {}", e);
			Err(e)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_translate() {
		let text = "Hello, how are you?".to_string();
		let language = Language::German;

		let translated_text = translate(text, language).await.unwrap();
		dbg!("{}", &translated_text);
	}
}
