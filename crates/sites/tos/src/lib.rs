pub use inline_css::css;
pub use ryde::*;

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
		<Html>
			<div>
				<style>{styles}</style>
				you are here
			</div>
		</Html>
	}
}

#[allow(non_snake_case)]
pub fn Html(els: Elements) -> Component {
	html! {
		<!DOCTYPE html>
		<html>
			<head>
				<script src="https://unpkg.com/htmx.org@2.0.3"></script>
				<script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js"></script>
				<script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js"></script>
				<title>The Objective Standard</title>
			</head>
			<body>
				{els}
			</body>
		</html>
	}
}
