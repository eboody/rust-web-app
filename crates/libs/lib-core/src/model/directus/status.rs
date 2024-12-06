use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Status {
	Published,
	UnderReview,
	Draft,
	Archived,
}
