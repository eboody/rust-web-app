use crate::prelude::*;

pub fn base(children: Markup) -> Markup {
	let Assets { main_js, main_css } = get_js_and_css_files();
	html! {
		(DOCTYPE)
		html {
			head {
				title { "SITE1" }
				script type="module" src=(format!("/{}", main_js)) {}
				link rel="stylesheet" href=(format!("/{}", main_css)) {}
				script src="https://unpkg.com/htmx.org@2.0.3" {}
				script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js" {}
				script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js" {}
				script defer src="https://umami.eman.network/script.js" data-website-id="af0eccf4-7f97-4d7a-a075-e54b9f125a49" {}
				link rel="preconnect" href="https://fonts.googleapis.com" {}
				link rel="preconnect" href="https://fonts.gstatic.com" crossorigin {}
				link href="https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap" rel="stylesheet" {}
				link rel="stylesheet" href="https://fonts.cdnfonts.com/css/minion-pro" {}
				meta name="viewport" content="width=device-width, initial-scale=1" {}
				(css())
			}
			body hx-boost="true" {
				div id="app" x-data="app" {
					(children)
				}
			}
		}
	}
}

css! {
	body {
		margin: 0;
		padding: 0;
	}
}
