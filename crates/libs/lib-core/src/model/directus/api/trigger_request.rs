use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerRequest {
	pub path: String,
	pub query: HashMap<String, String>, // Adjust as necessary if query params have complex types
	pub body: RequestBody,
	pub method: String,
	pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestBody {
	pub collection: String,
	pub keys: Vec<String>,
}
