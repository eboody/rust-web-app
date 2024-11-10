mod config;
mod error;
use axum::{
	extract::Path,
	http::{HeaderMap, HeaderValue},
	response::{Html, Response},
	routing::{get, post},
	Router,
};
use config::web_config;
use error::Error;
use inline_css::css;
use lib_directus::Ebook;
use maud::{html, Markup, Render, DOCTYPE};
use reqwest::header::AUTHORIZATION;
use serde_json::Value;

pub fn router() -> Router {
	Router::new().route("/", get(get_slash))
}

async fn get_slash() -> Markup {
	html! {
		(page())
	}
}

pub fn page() -> Markup {
	let folder = &web_config().WEB_FOLDER;

	let manifest: Value = serde_json::from_str(
		&std::fs::read_to_string(format!("{}/js/.vite/manifest.json", folder))
			.expect("manifest.json not found"),
	)
	.expect("Error parsing manifest.json");

	let main_js = manifest["index.html"]["file"]
		.as_str()
		.expect("index.html not found in manifest");
	let _main_css = manifest["index.html"]["css"][0]
		.as_str()
		.expect("index.css not found in manifest");

	let styles = css! {
		me {
			color: red;
		}
	};

	html! {
		(DOCTYPE)
		html {
			head {
				title { "SITE1" }
				script type="module" src=(format!("/js/{}", main_js)) {}
				script src="https://unpkg.com/htmx.org@2.0.3" {}
				script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js" {}
				script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js" {}
				link rel="stylesheet" href="https://fonts.cdnfonts.com/css/minion-pro" {}
			}
			body hx-boost="true" {
				div id="app" x-data="app" {
					div id="content" {
						style { (styles) }  // Include styles here
						"Hello, world!"
					}
				}
			}
		}
	}
}
