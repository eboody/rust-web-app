use crate::pages::ebooks;
use crate::prelude::*;
use axum::extract::State;
use serde::Deserialize;
use serde_json::json;

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

async fn popup_form_filled(
	State(mm): State<ModelManager>,
	Form(form): Form<Signup>,
) -> Result<Markup> {
	let res = mm
		.reqwest()
		.post(
			"https://n8n.eman.network/webhook/78b2e1c3-ceac-43d9-85c7-3870cf37710a",
		)
		.body(
			json!({
				"email": form.email,
				"first_name": form.first_name,
				"ebook_name": form.ebook_name,
			})
			.to_string(),
		)
		.header("Content-Type", "application/json")
		.send()
		.await;

	if let Err(e) = res {
		Ok(Toast::Error {
			text: format!("Error: {}", e),
		}
		.render())
	} else {
		Ok(Toast::Success {
			text: "Check your email for your download link!".into(),
		}
		.render())
	}
}
