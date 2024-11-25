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
