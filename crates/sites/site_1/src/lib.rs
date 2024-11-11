mod config;
mod error;
mod pages;
mod prelude;
mod utils;

use pages::ebooks;
use prelude::*;

use axum::{routing::get, Router};
use maud::Markup;

pub fn router() -> Router {
	Router::new().route("/", get(get_slash))
}

async fn get_slash() -> Result<Markup> {
	Ok(pages::base(pages::layouts::app_layout(
		ebooks::menu().await?,
	)))
}
