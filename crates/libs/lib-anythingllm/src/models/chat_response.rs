use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
	pub id: String,
	#[serde(rename = "type")]
	pub response_type: ResponseType,
	#[serde(rename = "textResponse")]
	pub text_response: Option<String>,
	pub close: bool,
	pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
	Abort,
	TextResponse,
}
