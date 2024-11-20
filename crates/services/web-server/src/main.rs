// region:    --- Modules
mod config;
mod error;
mod web;

pub use self::error::{Error, Result};
use axum::http::Method;
use config::web_config;

use crate::web::routes_static;
use axum::{http::HeaderValue, routing::get, Router};

use lib_core::model::ModelManager;
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

	let _mm = ModelManager::new().await?;

	let routes_all = Router::new()
		.route("/health", get(|| async { "OK" }))
		.merge(site_1::main_router())
		.layer(CookieManagerLayer::new())
		.layer(get_cors_layer())
		.fallback_service(routes_static::serve_dir());

	let listener = TcpListener::bind("0.0.0.0:3031").await.unwrap();
	info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}

fn get_cors_layer() -> CorsLayer {
	CorsLayer::new()
		.allow_origin(HeaderValue::from_static("*"))
		.allow_methods([Method::PATCH, Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers(Any)
}
