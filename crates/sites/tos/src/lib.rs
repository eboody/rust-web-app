mod config;
use config::web_config;
pub use inline_css::css;
pub use ryde::*;
use serde_json::Value;

pub fn router() -> Router {
	Router::new().route("/", get(get_slash))
}

async fn get_slash() -> Html {
	let styles = css! {
		me {
			color: red;
			background-color: #0x0000FF;
		}
	}
	.to_string();

	html! {
		<Page>
			<div>
				<style>{styles}</style>
				you are here
			</div>
		</Page>
	}
}

#[allow(non_snake_case)]
pub fn Page(els: Elements) -> Component {
	let folder = &web_config().WEB_FOLDER;

	let manifest: Value = serde_json::from_str(
		&std::fs::read_to_string(format!("{}/js/.vite/manifest.json", folder))
			.expect("manifest.json not found"),
	)
	.expect("Error parsing manifest.json");

	let main_js = manifest["index.html"]["file"]
		.as_str()
		.expect("index.html not found in manifest");
	let main_css = manifest["index.html"]["css"][0]
		.as_str()
		.expect("index.css not found in manifest");

	html! {
		<!DOCTYPE html>
		<html>
			<head>
				<link rel="stylesheet" href={format!("/js/{}", main_css)}>
				<script type="module" src={format!("/js/{}", main_js)}></script>
				<script src="https://unpkg.com/htmx.org@2.0.3"></script>
				<script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js"></script>
				<script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js"></script>
				<title>The Objective Standard</title>
			</head>
			<body>
				<div id="app" x-data="app">
					<a href="https://vitejs.dev" target="_blank">
						<img x-bind:src="viteLogo" class="logo" alt="Vite logo" />
					</a>
					<a href="https://alpinejs.dev/" target="_blank">
						<img x-bind:src="alpineLogo" class="logo alpine" alt="Alpine logo" />
					</a>
					<h1>Hello Vite + Alpine!</h1>
					<div class="card">
						<button id="counter" type="button" x-on:click="count++" x-text="`count is ${count}`"></button>
					</div>
					<p class="read-the-docs">Click on the Vite and Alpine logos to learn more</p>
				</div>
				{els}
			</body>
		</html>
	}
}
