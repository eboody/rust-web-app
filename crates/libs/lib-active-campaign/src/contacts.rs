use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::ActiveCampaign;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
	email: String,
	first_name: Option<String>,
	last_name: Option<String>,
	phone: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncContactResponse {
	contact: Option<ContactResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContactResponse {
	id: String,
}

impl ActiveCampaign {
	pub async fn get_contact(
		&self,
		email: &str,
	) -> Result<Option<ContactResponse>, Box<dyn Error>> {
		let url = format!("{}/api/3/contacts", self.api_url);

		let response = self
			.client
			.get(&url)
			.header("Api-Token", &self.api_key)
			.query(&[("email", email)])
			.send()
			.await?;

		if response.status().is_success() {
			let response_json: SyncContactResponse = response.json().await?;
			if let Some(contact) = response_json.contact {
				Ok(Some(contact))
			} else {
				println!("Contact not found.");
				Ok(None)
			}
		} else {
			let error_text = response.text().await?;
			Err(format!("Failed to retrieve contact: {}", error_text).into())
		}
	}
	pub async fn sync_contact(
		&self,
		contact: Contact,
	) -> Result<(), Box<dyn Error>> {
		let url = format!("{}/api/3/contact/sync", self.api_url);

		let response = self
			.client
			.post(&url)
			.header("Api-Token", &self.api_key)
			.json(&serde_json::json!({ "contact": contact }))
			.send()
			.await?;

		if response.status().is_success() {
			let response_json: SyncContactResponse = response.json().await?;
			if let Some(contact_response) = response_json.contact {
				println!("Contact synced with ID: {}", contact_response.id);
			} else {
				println!("Contact synced, but no ID returned.");
			}
			Ok(())
		} else {
			let error_text = response.text().await?;
			Err(format!("Failed to sync contact: {}", error_text).into())
		}
	}
}
