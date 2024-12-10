//use ormlite::types::Uuid;
//use serde::{Deserialize, Serialize};
//
//#[derive(Debug, Serialize, Clone, Deserialize)]
//pub enum Event {
//	ItemUpdate(ItemUpdate),
//}
//
//#[derive(Debug, Serialize, Clone, Deserialize)]
//pub struct ItemUpdate {
//	pub event: ItemEvent,
//	pub collection: String,
//	pub keys: Vec<Uuid>,
//	pub payload: json::Value,
//}
//
//#[derive(Debug, Serialize, Clone, Deserialize)]
//pub enum ItemEvent {
//	#[serde(rename = "articles.items.create")]
//	Create,
//	#[serde(rename = "articles.items.update")]
//	Update,
//	#[serde(rename = "articles.items.delete")]
//	Delete,
//}
