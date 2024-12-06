use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
	pub offset: u32,
	pub limit: u32,
	pub order_by: String,
	pub order_direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub posts: Vec<Post>,
	pub offset: u32,
	pub limit: u32,
	pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
	pub id: u64,
	pub uuid: String,
	pub editor_v2: bool,
	pub publication_id: u64,
	pub r#type: String,
	pub post_date: Option<String>,
	pub draft_created_at: String,
	pub email_sent_at: Option<String>,
	pub is_published: bool,
	pub title: Option<String>,
	pub draft_title: String,
	pub draft_updated_at: String,
	pub draft_video_upload_id: Option<u64>,
	pub audience: String,
	pub slug: Option<String>,
	pub should_send_email: bool,
	pub write_comment_permissions: String,
	pub default_comment_sort: Option<String>,
	pub section_id: Option<u64>,
	pub cover_image: Option<String>,
	pub should_send_free_preview: bool,
	pub video_upload_id: Option<u64>,
	pub is_metered: bool,
	pub section_slug: Option<String>,
	pub section_name: Option<String>,
	pub draft_section_name: Option<String>,
	pub is_section_pinned: bool,
	pub reactions: Reactions,
	pub reaction: Option<String>,
	pub top_exclusions: Vec<String>,
	pub pins: Vec<String>,
	pub published_bylines: Vec<Byline>,
	pub draft_bylines: Vec<Byline>,
	pub reaction_count: u32,
	pub comment_count: u32,
	pub child_comment_count: u32,
	pub bylines: Vec<Byline>,
	pub stats: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reactions {
	#[serde(rename = "❤")]
	pub heart: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Byline {
	pub id: u64,
	pub name: String,
	pub handle: String,
	pub previous_name: Option<String>,
	pub photo_url: String,
	pub bio: String,
	pub profile_set_up_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
	pub views: u32,
	pub opens: u32,
	pub opened: u32,
	pub open_rate: u32,
	pub clicked: u32,
	pub clicks: u32,
	pub sent: u32,
	pub delivered: u32,
	pub downloads: u32,
	pub downloads_day7: u32,
	pub downloads_day30: u32,
	pub downloads_day90: u32,
	pub podcast_preview_downloads: u32,
	pub podcast_preview_downloads_day30: u32,
	pub video_viewers: u32,
	pub video_views: u32,
	pub video_minutes_watched: u32,
	pub signups_within_1_day: u32,
	pub disables_within_1_day: u32,
	pub subscriptions_within_1_day: u32,
	pub unsubscribes_within_1_day: u32,
	pub signups: u32,
	pub subscribes: u32,
	pub shares: u32,
	pub estimated_value: u32,
}
