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
				link rel="preconnect" href="https://fonts.googleapis.com" {}
				link rel="preconnect" href="https://fonts.gstatic.com" crossorigin {}
				link href="https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap" rel="stylesheet" {}
				link rel="stylesheet" href="https://fonts.cdnfonts.com/css/minion-pro" {}
				(styles())
			}
			body hx-boost="true" {
				div id="app" x-data="app" {
					(children)
				}
			}
		}
	}
}

fn styles() -> Markup {
	html! { style { (css! {
		body {
			font-family: "Poppins", serif;
			margin: 0;
			padding: 0;
		}
	.vh {
			clip: rect(0 0 0 0);
			clip-path: inset(50%);
			block-size: 1px;
			inline-size: 1px;
			overflow: hidden;
			white-space: nowrap;
	}
	})}}
}
