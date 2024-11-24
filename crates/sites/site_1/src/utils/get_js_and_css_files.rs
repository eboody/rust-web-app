use serde_json::Value;

pub struct Assets {
	pub main_js: String,
	pub main_css: String,
}
pub fn get_js_and_css_files() -> Assets {
	let manifest: Value = serde_json::from_str(
		&std::fs::read_to_string(
			"crates/sites/site_1/src/web-folder/js/.vite/manifest.json",
		)
		.expect("manifest.json not found"),
	)
	.expect("Error parsing manifest.json");

	let main_js = manifest["index.html"]["file"]
		.as_str()
		.expect("index.html not found in manifest");
	let main_css = manifest["index.html"]["css"][0]
		.as_str()
		.expect("index.css not found in manifest");

	Assets {
		main_js: main_js.to_string(),
		main_css: main_css.to_string(),
	}
}
