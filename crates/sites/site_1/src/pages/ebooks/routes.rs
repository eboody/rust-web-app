use crate::pages::ebooks;
use crate::prelude::*;
use lib_active_campaign::ActiveCampaign;
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

async fn temp_post(Form(signup): Form<Signup>) -> impl IntoResponse {
	let ac = ActiveCampaign::new(
		std::env::var("ACTIVE_CAMPAIGN_URL").unwrap(),
		std::env::var("ACTIVE_CAMPAIGN_API_KEY").unwrap(),
	);

	let contact = ac
		.get_contact(&signup.email)
		.await
		.expect("Failed to get contact.");
	dbg!("{}", &contact);

	"temp post"
}
