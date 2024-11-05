mod component;

use axum::{routing::get, Router};

use ryde::*;

pub fn router() -> Router {
	Router::new().route("/", get(get_slash))
}

async fn get_slash() -> Html {
	html! {
		<Html>
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
				<title>ryde with rust</title>
			</head>
			<body>
				{els}
			</body>
		</html>
	}
}
