mod config;
use config::web_config;
pub use ryde::*;
use serde_json::Value;

pub fn router() -> Router {
	Router::new().route("/", get(get_slash))
}

async fn get_slash() -> Html {
	html! {
		<Page>
			<div>
				you are here
				<style>r#"
					me {
						color: red;
						background-color: #0000FF;
					}
				"#</style>
			</div>
			<div>
				I change color every second.
				<script>r#"
					me().on("click", async ev => {
						let el = me(ev) // Save target because async will lose it.
						me(el).styles({ "transition": "background 1s" })
						await sleep(1000)
						me(el).styles({ "background": "red" })
						await sleep(1000)
						me(el).styles({ "background": "green" })
						await sleep(1000)
						me(el).styles({ "background": "blue" })
						await sleep(1000)
						me(el).styles({ "background": "none" })
						await sleep(1000)
						me(el).remove()
					})
				"#</script>
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
			<body hx-boost=true>
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
					{els}
				</div>
			</body>
		</html>
	}
}
