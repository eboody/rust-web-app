use reqwest::{Client, StatusCode};

use crate::prelude::*;
mod macros;

#[allow(unused)]
pub async fn get_image(Path(image_id): Path<String>) -> Result<Response> {
	let cover_image_url =
		format!("https://directus.eman.network/assets/{}", image_id);

	// Fetch the DIRECTUS_TOKEN from environment variables
	let token = std::env::var("DIRECTUS_TOKEN")
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let client = Client::new();

	// Fetch the image from Directus
	let image_response = client
		.get(&cover_image_url)
		.bearer_auth(&token)
		.send()
		.await
		.map_err(|_| StatusCode::BAD_GATEWAY)?;

	if !image_response.status().is_success() {
		// Return a 404 if the image is not found
		return Err(Error::HttpStatusCode(StatusCode::NOT_FOUND));
	}

	// Get the content type from the Directus response headers
	let content_type = image_response
		.headers()
		.get(reqwest::header::CONTENT_TYPE)
		.cloned()
		.unwrap_or_else(|| HeaderValue::from_static("application/octet-stream"));

	// Read the image bytes
	let image_bytes = image_response
		.bytes()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	// Build the response to send back to the client
	let response = Response::builder()
		.status(StatusCode::OK)
		.header(axum::http::header::CONTENT_TYPE, content_type)
		.body(image_bytes)
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(response.into_body().into_response())
}
