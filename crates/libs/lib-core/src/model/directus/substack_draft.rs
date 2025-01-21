use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[ormlite(table = "substack_drafts")]
pub struct SubstackDraft {
    // Primary fields
    pub id: Uuid,
    pub articles_id: Uuid,

    // Fields from API response
    pub substack_draft_id: i64,
    pub substack_uuid: Uuid,
    pub draft_title: String,
    pub draft_subtitle: Option<String>,
    pub draft_type: String,
    pub audience: String,
    pub section_chosen: bool,
    pub publication_id: i64,
    pub word_count: Option<i64>,
    pub draft_body: json::Value,
    pub draft_created_at: OffsetDateTime,
    pub draft_updated_at: OffsetDateTime,

    // Our additional fields
    pub status: String,
    pub sync_status: String,
    pub last_sync_error: Option<String>,
    pub last_synced_at: OffsetDateTime,
    pub section_id: Option<String>,

    // Directus fields
    pub user_created: Option<Uuid>,
    pub date_created: Option<OffsetDateTime>,
    pub user_updated: Option<Uuid>,
    pub date_updated: Option<OffsetDateTime>,

    pub cover_image: Option<String>,
    pub default_comment_sort: Option<String>,
    pub description: Option<String>,
    pub draft_podcast_duration: Option<String>,
    pub draft_podcast_preview_upload_id: Option<String>,
    pub draft_podcast_upload_id: Option<String>,
    pub draft_podcast_url: Option<String>,
    pub draft_section_id: Option<String>,
    pub draft_video_upload_id: Option<String>,
    pub draft_voiceover_upload_id: Option<String>,
    pub editor_v2: Option<bool>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub email_sent_at: Option<OffsetDateTime>,
    pub explicit: Option<bool>,
    pub free_podcast_duration: Option<String>,
    pub free_podcast_url: Option<String>,
    pub has_dismissed_tk_warning: Option<bool>,
    pub hide_from_feed: Option<bool>,
    pub is_metered: Option<bool>,
    pub is_published: Option<bool>,
    pub podcast_duration: Option<String>,
    pub podcast_episode_number: Option<i32>,
    pub podcast_episode_type: Option<String>,
    pub podcast_season_number: Option<i32>,
    pub podcast_url: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub post_date: Option<OffsetDateTime>,
    pub search_engine_description: Option<String>,
    pub search_engine_title: Option<String>,
    pub should_send_email: Option<bool>,
    pub should_send_free_preview: Option<bool>,
    pub show_guest_bios: Option<bool>,
    pub slug: Option<String>,
    pub social_title: Option<String>,
    pub subscriber_set_id: Option<String>,
    pub subtitle: Option<String>,
    pub syndicate_to_section_id: Option<String>,
    pub should_syndicate_to_other_feed: Option<bool>,
    pub title: Option<String>,
    pub write_comment_permissions: Option<String>,
}
