use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum Event {
	Item(ItemEvent),
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum ItemEvent {
	Create,
	Update,
	Delete,
}
