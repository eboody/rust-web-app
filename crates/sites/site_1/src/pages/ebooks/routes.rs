use crate::pages::ebooks;
use crate::prelude::*;
use lib_active_campaign::{ActiveCampaign, Contact, ContactForCreate};
use reqwest::StatusCode;
use serde::Deserialize;

pub fn router() -> Router {
	Router::new()
		.route("/menu", get(ebooks::get_menu))
		.route("/popup/:id", get(ebooks::get_popup))
		.route("/signup", post(temp_post))
}

#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
struct Signup {
	email: String,
	first_name: String,
}

async fn temp_post(Form(signup): Form<Signup>) -> Result<impl IntoResponse> {
	let ac = ActiveCampaign::new(
		std::env::var("ACTIVE_CAMPAIGN_KEY").unwrap(),
		std::env::var("ACTIVE_CAMPAIGN_URL").unwrap(),
	);

	ac.sync_contact(ContactForCreate {
		email: signup.email,
		first_name: Some(signup.first_name),
		last_name: None,
		phone: None,
		field_values: None,
	})
	.await
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(StatusCode::OK)
}
