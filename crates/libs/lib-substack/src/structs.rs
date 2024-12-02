use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DraftRequest {
	pub audience: String,
	pub draft_body: String,
	pub draft_bylines: Vec<DraftByline>,
	pub draft_podcast_duration: Option<String>,
	pub draft_podcast_preview_upload_id: Option<String>,
	pub draft_podcast_upload_id: Option<String>,
	pub draft_podcast_url: String,
	pub draft_section_id: Option<String>,
	pub draft_subtitle: String,
	pub draft_title: String,
	pub draft_video_upload_id: Option<String>,
	pub draft_voiceover_upload_id: Option<String>,
	pub section_chosen: bool,
	pub r#type: String,
}

#[derive(Serialize, Debug)]
pub struct DraftByline {
	pub id: u64,
	pub is_guest: bool,
}

#[derive(Deserialize, Debug)]
pub struct DraftResponse {
	pub id: u64,
	#[serde(rename = "type")]
	pub type_: String,
	pub draft_title: String,
	pub draft_subtitle: String,
	pub audience: String,
	pub section_chosen: bool,
	pub publication_id: u64,
	pub word_count: u64,
	pub draft_body: String,
	pub draft_created_at: String,
	pub draft_updated_at: String,
	pub uuid: String,
	// i might need to add more fields here
}

#[derive(Debug, Deserialize)]
pub struct Document {
	#[serde(rename = "type")]
	pub doc_type: NodeType,
	pub content: Vec<Content>,
}

impl From<Document> for Content {
	fn from(doc: Document) -> Self {
		Content {
			type_: doc.doc_type,
			content: Some(doc.content),
			..Default::default()
		}
	}
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Content {
	#[serde(rename = "type")]
	pub type_: NodeType,
	pub attrs: Option<Attributes>,
	pub content: Option<Vec<Content>>,
	pub marks: Option<Vec<Mark>>,
	pub text: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Attributes {
	pub level: Option<u8>,
	pub href: Option<String>,
	pub title: Option<String>,
	pub number: Option<u32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Mark {
	#[serde(rename = "type")]
	pub mark_type: MarkType,
	pub attrs: Option<Attributes>,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize, Display, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
	Doc,
	Heading,
	#[default]
	Paragraph,
	Text,
	Blockquote,
	Link,
	#[serde(rename = "footnoteAnchor")]
	FootnoteAnchor,
	Footnote,
	HardBreak,
	OrderedList,
	ListItem,
	BulletList,
	Other(String),
}

#[derive(Debug, Deserialize, Clone, Serialize, Display, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MarkType {
	Em,
	Strong,
	Link,
	Other(String),
}
