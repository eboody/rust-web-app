use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
	Published,
	Draft,
	Archived,
}
