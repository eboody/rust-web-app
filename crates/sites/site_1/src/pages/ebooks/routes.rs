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
	let n8n_url = std::env::var("EBOOK_N8N_URL").unwrap_or(
		"https://n8n.eman.network/webhook-test/03cf713c-7a5a-457d-b58c-807744de76a6"
			.to_owned(),
	);

	let res = mm
		.reqwest()
		.post(n8n_url)
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
