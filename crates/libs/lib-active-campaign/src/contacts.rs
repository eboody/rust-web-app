use serde::{Deserialize, Serialize};
use std::error::Error;

use serde_json_debugging::DebugDeserialize;

use crate::ActiveCampaign;
use lib_utils::time::Rfc3339;

use serde_with::{serde_as, DisplayFromStr};
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use url::Url;

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");
time::serde::format_description!(
	ac_date_time,
	PrimitiveDateTime,
	"[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactForCreateBody {
	pub contact: ContactForCreate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactForCreate {
	pub email: String,
	#[serde(rename = "firstName")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_name: Option<String>,

	#[serde(rename = "lastName")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_name: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone: Option<String>,

	#[serde(rename = "fieldValues")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub field_values: Option<Vec<FieldValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldValue {
	pub field_id: i32,
	pub value: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
	#[serde_as(as = "Rfc3339")]
	pub cdate: OffsetDateTime,
	pub email: String,
	pub phone: Option<String>,
	#[serde(rename = "firstName")]
	pub first_name: Option<String>,
	#[serde(rename = "lastName")]
	pub last_name: Option<String>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub orgid: Option<u64>,
	pub orgname: Option<String>,
	pub segmentio_id: Option<String>,
	#[serde_as(as = "DisplayFromStr")]
	pub bounced_hard: u32,
	#[serde_as(as = "DisplayFromStr")]
	pub bounced_soft: u32,
	#[serde_as(as = "Option<Rfc3339>")]
	pub bounced_date: Option<OffsetDateTime>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub ip: Option<u32>,
	pub ua: Option<String>,
	pub hash: Option<String>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub socialdata_lastcheck: Option<OffsetDateTime>,
	pub email_local: Option<String>,
	pub email_domain: Option<String>,
	#[serde_as(as = "DisplayFromStr")]
	pub sentcnt: u32,
	#[serde(with = "one_true_date::option")]
	pub rating_tstamp: Option<Date>,
	#[serde_as(as = "DisplayFromStr")]
	pub gravatar: u32,
	#[serde_as(as = "DisplayFromStr")]
	pub deleted: u32,
	#[serde_as(as = "DisplayFromStr")]
	pub anonymized: u32,
	#[serde_as(as = "Option<Rfc3339>")]
	pub adate: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub udate: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub edate: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub deleted_at: Option<OffsetDateTime>,
	#[serde(with = "ac_date_time::option")]
	pub created_utc_timestamp: Option<PrimitiveDateTime>,
	#[serde(with = "ac_date_time::option")]
	pub updated_utc_timestamp: Option<PrimitiveDateTime>,
	#[serde(with = "ac_date_time::option")]
	pub created_timestamp: Option<PrimitiveDateTime>,
	#[serde(with = "ac_date_time::option")]
	pub updated_timestamp: Option<PrimitiveDateTime>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub created_by: Option<u64>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub updated_by: Option<u64>,
	#[serde_as(as = "DisplayFromStr")]
	pub mpp_tracking: u32,
	#[serde(with = "ac_date_time::option")]
	pub last_click_date: Option<PrimitiveDateTime>,
	#[serde(with = "ac_date_time::option")]
	pub last_open_date: Option<PrimitiveDateTime>,
	#[serde(rename = "accountContacts")]
	pub account_contacts: Vec<String>,
	pub links: ContactLinks,
	#[serde_as(as = "DisplayFromStr")]
	pub id: u64,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub organization: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactLinks {
	pub bounce_logs: Url,
	pub contact_automations: Url,

	pub contact_data: Url,

	pub contact_goals: Url,

	pub contact_lists: Url,

	pub contact_logs: Url,

	pub contact_tags: Url,

	pub contact_deals: Url,

	pub deals: Url,

	pub field_values: Url,

	pub geo_ips: Url,

	pub notes: Url,

	pub organization: Url,

	pub plus_append: Url,

	pub tracking_logs: Url,

	pub score_values: Url,

	pub automation_entry_counts: Url,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct ContactResponse {
	contact: Contact,
}

pub type SyncContactResponse = ContactResponse;

impl ActiveCampaign {
	pub async fn get_contact(
		&self,
		email: &str,
	) -> Result<ContactResponse, Box<dyn Error>> {
		let url = format!("{}/api/3/contacts", self.api_url);

		let response = self
			.client
			.get(&url)
			.header("Api-Token", &self.api_key)
			.query(&[("email", email)])
			.send()
			.await?;

		if response.status().is_success() {
			let contact = response.text().await?;
			dbg!("{}", &contact);

			todo!();
		} else {
			let error_text = response.text().await?;
			Err(format!("Failed to retrieve contact: {}", error_text).into())
		}
	}
	pub async fn sync_contact(
		&self,
		contact: ContactForCreate,
	) -> Result<SyncContactResponse, Box<dyn Error>> {
		let url = format!("{}/api/3/contact/sync", self.api_url);

		let body = &serde_json::to_value(ContactForCreateBody { contact });
		if let Err(e) = &body {
			dbg!("{}", e);
			todo!();
		} else {
			dbg!("{}", &body);
		}

		let response = self
			.client
			.post(&url)
			.header("Api-Token", &self.api_key)
			.json(body.as_ref().unwrap())
			.send()
			.await;

		if let Err(e) = &response {
			dbg!("{}", e);
			todo!();
		} else {
			dbg!("{}", &response);
		}

		let response = response?;

		if response.status().is_success() {
			let sync_contact_response: DebugDeserialize<SyncContactResponse> =
				response.json().await?;
			dbg!("{}", &sync_contact_response);
			todo!();
			//dbg!("{}", &sync_contact_response);
			//
			//println!(
			//	"Contact synced with ID: {}",
			//	sync_contact_response.contact.id
			//);
			//
			//Ok(sync_contact_response)
		} else {
			let error_text = response.text().await?;
			Err(format!("Failed to sync contact: {}", error_text).into())
		}
	}
}
