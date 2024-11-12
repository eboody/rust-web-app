use crate::pages;
use crate::pages::layouts;
use crate::prelude::*;

use axum::{routing::get, Router};
use maud::Markup;

pub fn router() -> Router {
	Router::new()
		.route("/", get(get_slash))
		.nest("/ebooks", pages::ebooks::router())
}

async fn get_slash() -> Result<Markup> {
	Ok(pages::base(layouts::app_layout(html! {
		div hx-get="/ebooks/popup/3" hx-trigger="load" hx-swap="outerHTML" {}
		div hx-get="/ebooks/menu" hx-trigger="load" hx-swap="outerHTML" {}
	})))
}
