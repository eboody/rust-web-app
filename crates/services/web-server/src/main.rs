mod config;
mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
use axum::http::{Method, StatusCode, Uri};

use axum::response::{IntoResponse, Response};
use axum::{http::HeaderValue, routing::get, Router};

use lib_automation::prelude::Uuid;
use lib_core::model::ModelManager;
use log::log_request;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let mm = ModelManager::new().await?;

	let routes_all = Router::new()
		.route("/health", get(|| async { "OK" }))
		.merge(site_1::main_router(mm.clone()))
		.merge(lib_automation::routes(mm.clone()))
		.layer(axum::middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new())
		.layer(get_cors_layer());

	let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
	info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}

pub fn get_cors_layer() -> CorsLayer {
	CorsLayer::new()
		.allow_origin(HeaderValue::from_static("*"))
		.allow_methods([Method::PATCH, Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers(Any)
}

async fn main_response_mapper(
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<Error>();
	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ = log_request(uuid, req_method, uri, service_error).await;

	println!();
	service_error
		.map(|se| {
			(StatusCode::INTERNAL_SERVER_ERROR, se.to_string()).into_response()
		})
		.unwrap_or(res)
}
