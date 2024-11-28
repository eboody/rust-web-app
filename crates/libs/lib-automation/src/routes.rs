use crate::on_ebook_file_upload;
use axum::{routing::post, Router};
use lib_core::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/on_ebook_file_upload", post(on_ebook_file_upload))
		.route(
			"/check",
			post(|| async {
				println!("check");
				"OK"
			}),
		)
		.with_state(mm)
}
