mod client;
mod ebook;
pub use client::DirectusClient;
pub use ebook::*;

//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct ResponseVecData {
//	data: Vec<Ebook>,
//}
//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct ResponseData {
//	pub data: Ebook,
//}
//
//impl ResponseVecData {
//	pub fn to_ebooks(self) -> Vec<Ebook> {
//		self.data
//	}
//}

//pub async fn get_ebooks() -> Result<Vec<Ebook>, Error> {
//	let url = "https://directus.eman.network/items/eBooks";
//	let token = reqwest::get("https://tos-token-service.eman.network/token")
//		.await?
//		.text()
//		.await?;
//
//	let client = reqwest::Client::new();
//	let mut headers = HeaderMap::new();
//	headers.insert(
//		AUTHORIZATION,
//		HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
//	);
//
//	let response = client.get(url).headers(headers).send().await?;
//	let res: ResponseVecData = response.json().await?;
//	let ebooks = res.to_ebooks();
//
//	Ok(ebooks)
//}
//
//pub async fn get_ebook(id: u32) -> Result<Ebook, Error> {
//	let url = format!("https://directus.eman.network/items/eBooks/{}", id);
//	let token = reqwest::get("https://tos-token-service.eman.network/token")
//		.await?
//		.text()
//		.await?;
//
//	let client = reqwest::Client::new();
//	let mut headers = HeaderMap::new();
//	headers.insert(
//		AUTHORIZATION,
//		HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
//	);
//
//	let response = client.get(url).headers(headers).send().await?;
//	let res: ResponseData = response.json().await?;
//	let ebook = res.data;
//
//	Ok(ebook)
//}
