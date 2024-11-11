use crate::config::web_config;
use crate::prelude::*;

pub fn base(children: Markup) -> Markup {
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
				style { (css! {
					body {
						font-family: "Minion Pro", serif;
						margin: 0;
						padding: 0;
					}
				}) }
			}
			body hx-boost="true" {
				div id="app" x-data="app" {
					(children)
				}
			}
		}
	}
}
