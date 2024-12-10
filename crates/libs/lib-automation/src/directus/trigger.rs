use json::Value;
use lib_core::model::directus::{Collection, articles};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(untagged)]
pub enum Event {
	Articles(articles::Event),
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Body {
	pub event: Event,
	pub collection: Collection,
	pub keys: Vec<Uuid>,
	pub payload: Value,
}
