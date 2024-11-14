mod contacts;

use reqwest::Client;

pub struct ActiveCampaign {
	api_key: String,
	api_url: String,
	client: Client,
}

impl ActiveCampaign {
	pub fn new(api_key: String, api_url: String) -> Self {
		ActiveCampaign {
			api_key,
			api_url,
			client: Client::new(),
		}
	}
}
