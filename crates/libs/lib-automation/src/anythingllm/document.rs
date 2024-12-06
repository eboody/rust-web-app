use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
	pub location: String,
	pub name: Option<String>,
	pub url: String,
	pub title: String,
	#[serde(rename = "docAuthor")]
	pub doc_author: String,
	pub description: String,
	#[serde(rename = "docSource")]
	pub doc_source: String,
	#[serde(rename = "chunkSource")]
	pub chunk_source: String,
	pub published: String,
	#[serde(rename = "wordCount")]
	pub word_count: u32,
	#[serde(rename = "token_count_estimate")]
	pub token_count_estimate: u32,
}
