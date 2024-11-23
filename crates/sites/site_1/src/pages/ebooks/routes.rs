use crate::pages::ebooks;
use crate::prelude::*;
use axum::extract::State;
use lib_core::model::ModelManager;
use reqwest::StatusCode;
use serde::Deserialize;

pub fn router(mm: ModelManager) -> Router {
	Router::new()
		.route("/menu", get(ebooks::get_menu))
		.route("/popup/:id", get(ebooks::get_popup))
		.route("/signup", post(temp_post))
		.with_state(mm)
}

#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
struct Signup {
	email: String,
	first_name: String,
}

async fn temp_post(State(mm): State<ModelManager>) -> Result<impl IntoResponse> {
	mm.reqwest()
		.get("https://n8n.eman.network/webhook-test/24478e2d-b29a-43dd-bc71-59ea2efbcf6c")
		.send()
		.await?;

	Ok(StatusCode::OK)
}
