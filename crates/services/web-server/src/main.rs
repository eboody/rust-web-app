// region:    --- Modules
mod config;
mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
use axum::http::Method;
use config::web_config;

use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolver};
use crate::web::mw_req_stamp::mw_req_stamp_resolver;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{
	http::{HeaderName, HeaderValue},
	middleware,
	routing::get,
	Router,
};
//use lib_core::_dev_utils;
use lib_core::model::ModelManager;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	//_dev_utils::init_dev().await;

	let mm = ModelManager::new().await?;

	// -- Define Routes
	let routes_rpc = web::routes_rpc::routes(mm.clone())
		.route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.route("/health", get(|| async { "OK" }))
		.merge(routes_login::routes(mm.clone()))
		.merge(site_1::router())
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
		.layer(CookieManagerLayer::new())
		.layer(middleware::from_fn(mw_req_stamp_resolver))
		.layer(get_cors_layer())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	// Note: For this block, ok to unwrap.
	let listener = TcpListener::bind("0.0.0.0:3031").await.unwrap();
	info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}

fn get_cors_layer() -> CorsLayer {
	CorsLayer::new()
		.allow_origin(HeaderValue::from_str("https://wp.eman.network").unwrap())
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers([
			HeaderName::from_static("content-type"),
			HeaderName::from_static("hx-request"),
		])
		.allow_credentials(true)
}
