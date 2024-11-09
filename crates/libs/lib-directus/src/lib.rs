use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ebook {
	#[serde(rename = "Availability")]
	pub availability: String,
	#[serde(rename = "Name")]
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
}

impl Ebook {
	pub fn get_cover_image(&self) -> String {
		format!("https://directus.eman.network/assets/{}", self.cover_image)
	}

	pub fn get_file_download(&self) -> String {
		format!("https://directus.eman.network/assets/{}", self.file)
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseData {
	data: Vec<Ebook>,
}

impl ResponseData {
	pub fn to_ebooks(self) -> Vec<Ebook> {
		self.data
	}
}

pub async fn get_ebooks() -> Result<Vec<Ebook>, Error> {
	let url = "https://directus.eman.network/items/eBooks";

	// Send the GET request
	let response = reqwest::get(url).await?;
	let res: ResponseData = response.json().await?;
	let ebooks = res.to_ebooks();

	Ok(ebooks)
}
