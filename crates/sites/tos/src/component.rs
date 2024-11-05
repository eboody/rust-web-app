use axum::response::{IntoResponse, Response};
use shtml::Render;

#[derive(Debug, PartialEq, Eq)]
pub struct Component {
	pub html: String,
}

pub type Html = Component;

impl IntoResponse for Html {
	fn into_response(self) -> Response {
		axum::response::Html(self.html).into_response()
	}
}

impl Render for Component {
	fn render_to_string(&self, buffer: &mut String) {
		self.html.clone();
	}
}
