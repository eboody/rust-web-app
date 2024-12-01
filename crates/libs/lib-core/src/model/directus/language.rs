use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum Language {
	#[display("en")]
	English,
	#[display("es")]
	Spanish,
	#[display("fr")]
	French,
	#[display("de")]
	German,
	#[display("pt")]
	Portuguese,
	#[display("hi")]
	Hindi,
	#[display("th")]
	Thai,
}

#[derive(Debug, ormlite::Model)]
pub struct Languages {
	#[ormlite(primary_key)]
	pub code: String,
	pub direction: Option<String>,
	pub name: Option<String>,
}
