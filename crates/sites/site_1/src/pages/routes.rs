use crate::pages::*;
use crate::prelude::*;

use axum::{routing::get, Router};
use lib_core::model::ModelManager;
use maud::Markup;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

const PUBLIC_DIR: &str = "crates/sites/site_1/src/web-folder";

pub fn main_router(mm: ModelManager) -> Router {
	let Assets { main_js, main_css } = get_js_and_css_files();
	dbg!("main_css: {}", &main_css);
	dbg!("main_js: {}", &main_js);
	Router::new()
		.route("/", get(get_slash))
		.route_service(
			"/css",
			ServeFile::new(format!(
				"crates/sites/site_1/src/web-folder/js/{}",
				main_css
			)),
		)
		.route_service(
			"/js",
			ServeFile::new(format!(
				"crates/sites/site_1/src/web-folder/js/{}",
				main_js
			)),
		)
		.nest("/ebooks", ebooks::router(mm.clone()))
		.fallback_service(ServeDir::new(PUBLIC_DIR))
}

async fn get_slash() -> Result<Markup> {
	Ok(layouts::base(layouts::app(html! {
		div hx-get="/ebooks/popup/ffa4e238-daaf-4e00-9f4f-3944d1ff329f" hx-trigger="load" hx-swap="outerHTML" {}
		//(ebooks::get_menu().await?)
	})))
}
