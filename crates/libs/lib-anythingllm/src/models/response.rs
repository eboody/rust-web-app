use super::document::Document;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
	pub success: bool,
	pub error: Option<String>,
	pub documents: Vec<Document>,
}
