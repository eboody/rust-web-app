use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
	pub id: i64,
	#[serde(rename = "type")]
	pub type_: String,
	pub draft_title: String,
	pub draft_subtitle: String,
	pub audience: String,
	pub section_chosen: bool,
	pub publication_id: i64,
	pub word_count: i64,
	pub draft_body: String,
	pub draft_created_at: String,
	pub draft_updated_at: String,
	pub uuid: Uuid,
	// i might need to add more fields here
}
