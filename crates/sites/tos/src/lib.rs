mod component;

use axum::{routing::get, Router};

use ryde::*;

pub fn router() -> Router {
	Router::new().route("/", get(get_slash))
}

async fn get_slash() -> Html {
	html! {
		<Html>
			<style>
				"
					me {
						background-color: red;
					}
				"
			</style>
			<div>you are here</div>
		</Html>
	}
}

#[allow(non_snake_case)]
fn Html(els: Elements) -> Component {
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
