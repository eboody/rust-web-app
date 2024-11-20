use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
	English,
	Spanish,
	French,
	German,
	Portuguese,
	Hindi,
	Thai,
}
