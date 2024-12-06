use lib_core::model::directus::{Collection, Event, Status};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
	pub path: Option<String>,
	pub query: Option<HashMap<String, String>>, // Adjust as necessary if query params have complex types
	pub body: Body,
	pub method: String,
	pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body {
	pub collection: Collection,
	pub keys: Vec<Uuid>,
	pub event: Option<Event>,
	pub payload: Option<Payload>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
	pub status: Option<Status>,
}
