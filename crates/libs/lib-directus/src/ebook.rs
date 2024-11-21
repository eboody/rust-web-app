use lib_sdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ebook {
	pub availability: String,
	pub name: String,
	pub cover_image: String,
	pub date_created: String,
	pub date_updated: Option<String>,
	pub file: String,
	pub id: u32,
	pub sort: Option<serde_json::Value>,
	pub status: String,
	pub user_updated: Option<String>,
	pub sub_text: Option<String>,
	pub times_hovered_over: u32,
	pub times_downloaded: u32,
	pub hovers: Option<serde_json::Value>,
}

impl Ebook {
	pub fn get_thumbnail(&self, width: u32) -> String {
		format!(
			"https://directus.eman.network/assets/{}$thumbnail={width}",
			self.cover_image
		)
	}
	pub fn get_cover_image(&self) -> String {
		format!("https://directus.eman.network/assets/{}", self.cover_image)
	}

	pub fn get_file_download(&self) -> String {
		format!("https://directus.eman.network/assets/{}", self.file)
	}
}

pub struct EbookQueryParams {
	pub availability: Option<String>,
	pub name: Option<String>,
	pub cover_image: Option<String>,
	pub date_created: Option<String>,
	pub date_updated: Option<String>,
	pub file: Option<String>,
	pub id: Option<u32>,
	pub sort: Option<serde_json::Value>,
	pub status: Option<String>,
	pub user_updated: Option<String>,
	pub sub_text: Option<String>,
	pub times_hovered_over: Option<u32>,
	pub times_downloaded: Option<u32>,
	pub hovers: Option<serde_json::Value>,
}

impl Resource for Ebook {
	type Id = String;

	fn endpoint() -> &'static str {
		"/items/ebooks"
	}
}

impl_service!(
	ServiceName: EbookService,
	Resource: Ebook,
	ForCreate: (),
	ForUpdate: (),
	QueryParams: EbookQueryParams,
	IdType: String,
	Traits: [
		GetService
	]
);
