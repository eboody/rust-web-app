use crate::pages::*;
use crate::prelude::*;

use axum::{routing::get, Router};
use lib_core::model::ModelManager;
use maud::Markup;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

const PUBLIC_DIR: &str = "crates/sites/site_1/src/web-folder";

pub fn main_router(mm: ModelManager) -> Router {
	Router::new()
		.route("/", get(get_slash))
		.nest("/ebooks", ebooks::router(mm.clone()))
		.merge(js_and_css_routes())
		.fallback_service(ServeDir::new(PUBLIC_DIR))
}

async fn get_slash() -> Result<Markup> {
	Ok(layouts::base(layouts::app(html! {
		div hx-get="/ebooks/popup" hx-trigger="load" hx-swap="outerHTML" {}
		//(ebooks::get_menu().await?)
	})))
}

fn js_and_css_routes() -> Router {
	let Assets { main_js, main_css } = get_js_and_css_files();
	let web_folder = std::env::var("SERVICE_WEB_FOLDER").unwrap();
	Router::new()
		.route_service(
			"/css",
			ServeFile::new(format!("{}/{}", web_folder, main_css)),
		)
		.route_service("/js", ServeFile::new(format!("{}/{}", web_folder, main_js)))
}
