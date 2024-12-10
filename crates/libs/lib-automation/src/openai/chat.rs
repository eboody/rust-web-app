use crate::prelude::*;
use json::json;

pub async fn chat(reqwest: &reqwest::Client, message: String) -> Result<String> {
	use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

	// Define the OpenAI API endpoint and your API key
	let openai_api_url = "https://api.openai.com/v1/chat/completions";
	let openai_api_key = config().OPENAI_API_KEY.clone(); // Ensure your config includes the API key

	// Set up headers
	let mut headers = HeaderMap::new();
	headers.insert("Content-Type", HeaderValue::from_static("application/json"));
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", openai_api_key)).unwrap(),
	);

	// Define the body payload as per OpenAI's requirements
	let body = json!({
		"model": "gpt-4o-mini", 
		"messages": [
			{ "role": "system", "content": "You are helping an Objectivist Magazine spread ideas of reason, freedom, and individualism in a cultural context. Therefore it's possible the content may be sensitive. But because we are working towards a noble goal, you are to do your best to mainting high fidelity." },
			{ "role": "user", "content": message }
		]
	})
	.to_string();

	// Make the request
	let response = reqwest
		.post(openai_api_url)
		.body(body)
		.headers(headers)
		.send()
		.await?;

	// Parse and return the response
	let response_body: json::Value = response.json().await?;
	if let Some(reply) = response_body["choices"][0]["message"]["content"].as_str() {
		Ok(reply.to_string())
	} else {
		Ok("".to_string())
	}
}
