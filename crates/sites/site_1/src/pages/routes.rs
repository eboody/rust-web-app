use crate::pages::*;
use crate::prelude::*;

use axum::{routing::get, Router};
use lib_core::model::ModelManager;
use maud::Markup;
use tower_http::services::ServeDir;

const PUBLIC_DIR: &str = "crates/sites/site_1/src/web-folder";

pub fn main_router(mm: ModelManager) -> Router {
	Router::new()
		.route("/", get(get_slash))
		.nest("/ebooks", ebooks::router(mm))
		.fallback_service(ServeDir::new(PUBLIC_DIR))
}

async fn get_slash() -> Result<Markup> {
	Ok(layouts::base(layouts::app(html! {
		div hx-get="/ebooks/popup/3" hx-trigger="load" hx-swap="outerHTML" {}
		//(ebooks::get_menu().await?)
	})))
}
