use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalFile {
  pub name: String,
  #[serde(rename = "type")]
  pub file_type: String, // `type` is a reserved keyword in Rust
  pub items: Vec<FileItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileItem {
  pub name: String,
  #[serde(rename = "type")]
  pub file_type: String, // `type` is a reserved keyword in Rust
  pub id: String,
  pub url: String,
  pub title: String,
  pub cached: bool,
}
