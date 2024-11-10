use reqwest::{
	header::{HeaderMap, HeaderValue, AUTHORIZATION},
	Error,
};
use serde::{Deserialize, Serialize};
use serde_json_debugging::DebugDeserialize;

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
	pub hovers: Option<serde_json::Value>,
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
pub struct ResponseVecData {
	data: Vec<Ebook>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseData {
	pub data: Ebook,
}

impl ResponseVecData {
	pub fn to_ebooks(self) -> Vec<Ebook> {
		self.data
	}
}

pub async fn get_ebooks() -> Result<Vec<Ebook>, Error> {
	let url = "https://directus.eman.network/items/eBooks";
	let token = std::env::var("DIRECTUS_TOKEN").unwrap();
	let client = reqwest::Client::new();
	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
	);

	// Send the GET request
	let response = client.get(url).headers(headers).send().await?;
	let res: DebugDeserialize<ResponseVecData> = response.json().await?;
	dbg!("{}", &res);
	let ebooks = res.0.to_ebooks();

	Ok(ebooks)
}
