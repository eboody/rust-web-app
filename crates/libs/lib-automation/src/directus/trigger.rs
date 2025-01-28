use json::Value;
use lib_core::model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Articles(model::Event),
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Request {
    pub body: Body,
    pub headers: Value,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Body {
    pub event: Option<Event>,
    pub collection: model::Collection,
    pub keys: Vec<Uuid>,
    pub payload: Option<Value>,
}
