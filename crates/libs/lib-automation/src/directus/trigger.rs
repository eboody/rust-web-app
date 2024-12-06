use json::Value;
use lib_core::model::{articles, directus::Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(untagged)]
pub enum Event {
	Article(articles::Event),
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Body {
	pub event: Event,
	pub collection: Collection,
	pub keys: Vec<Uuid>,
	pub payload: Value,
}
