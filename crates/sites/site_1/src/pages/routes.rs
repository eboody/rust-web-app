use crate::pages::*;
use crate::prelude::*;

use axum::{routing::get, Router};
use maud::Markup;

pub fn main_router() -> Router {
	Router::new()
		.route("/", get(get_slash))
		.nest("/ebooks", ebooks::router())
}

async fn get_slash() -> Result<Markup> {
	Ok(layouts::base(layouts::app(html! {
		div hx-get="/ebooks/popup/3" hx-trigger="load" hx-swap="outerHTML" {}
		(ebooks::get_menu().await?)
	})))
}
