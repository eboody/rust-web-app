use crate::pages::ebooks;
use crate::prelude::*;
use axum::extract::State;
use serde::Deserialize;

pub fn router(mm: ModelManager) -> Router {
	Router::new()
		.route("/menu", get(ebooks::get_menu))
		.route("/popup/:id", get(ebooks::get_popup))
		.route("/signup", post(popup_form_filled))
		.with_state(mm)
}

#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
struct Signup {
	email: String,
	first_name: String,
	ebook_name: String,
}

async fn popup_form_filled(State(mm): State<ModelManager>) -> Result<Markup> {
	mm.reqwest()
		.get("https://n8n.eman.network/webhook-test/24478e2d-b29a-43dd-bc71-59ea2efbcf6c")
		.send()
		.await?;

	Ok(html! {(Toast::Success { text: "Check your email for your download link!" })})
}
