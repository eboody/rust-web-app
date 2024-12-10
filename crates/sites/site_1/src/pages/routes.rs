use crate::{pages::*, prelude::*};

use axum::{Router, routing::get};
use layouts::{App, Base};
use lib_core::model::ModelManager;
use maud::Markup;
use tower_http::services::{ServeDir, ServeFile};

pub fn main_router(mm: ModelManager) -> Router {
	Router::new()
		.route("/", get(get_slash))
		.nest("/ebooks", ebooks::router(mm.clone()))
		.merge(js_and_css_routes())
		.fallback_service(ServeDir::new(
			std::env::var("SERVICE_WEB_FOLDER").unwrap(),
		))
}

async fn get_slash() -> Result<Markup> {
	Ok(Base(App(
		html! {
		#pre-ebook-popup hx-get="/ebooks/popup" hx-trigger="showPopup" hx-swap="innerHTML" {}
		(js())
		//(ebooks::get_menu().await?)
	}
	).render()).render())
}

js! {


	onloadAdd((_) => {
		if (!getCookie("popupShown")) {

			me("#pre-ebook-popup").send("showPopup");

			setCookie("popupShown", "true", 30); //expires in 30 days
		} else {
			console.log("Popup already shown, skipping...");
		}
	});
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
