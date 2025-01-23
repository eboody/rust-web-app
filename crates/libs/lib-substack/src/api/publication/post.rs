use crate::{Tag, TagAssociation, post, prelude::*};
use lib_utils::retry::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: post::Type,
    pub draft_title: String,
    pub draft_subtitle: String,
    pub audience: post::Audience,
    pub section_chosen: bool,
    pub publication_id: i64,
    pub word_count: Option<i64>,
    pub draft_body: json::Value,
    #[serde(with = "time::serde::rfc3339")]
    pub draft_created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub draft_updated_at: OffsetDateTime,
    pub uuid: Uuid,

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
    pub section_id: Option<String>,
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

    pub reaction_count: i32,
    pub comment_count: i32,
    pub child_comment_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reactions {
    #[serde(rename = "❤")]
    pub heart: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Byline {
    pub id: i64,
    pub name: String,
    pub handle: String,
    pub previous_name: Option<String>,
    pub photo_url: String,
    pub bio: String,
    pub profile_set_up_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub views: i32,
    pub opens: i32,
    pub opened: i32,
    pub open_rate: i32,
    pub clicked: i32,
    pub clicks: i32,
    pub sent: i32,
    pub delivered: i32,
    pub downloads: i32,
    pub downloads_day7: i32,
    pub downloads_day30: i32,
    pub downloads_day90: i32,
    pub podcast_preview_downloads: i32,
    pub podcast_preview_downloads_day30: i32,
    pub video_viewers: i32,
    pub video_views: i32,
    pub video_minutes_watched: i32,
    pub signups_within_1_day: i32,
    pub disables_within_1_day: i32,
    pub subscriptions_within_1_day: i32,
    pub unsubscribes_within_1_day: i32,
    pub signups: i32,
    pub subscribes: i32,
    pub shares: i32,
    pub estimated_value: i32,
}
// Implement additional methods for Post
impl Post {
    pub fn is_draft(&self) -> bool {
        self.is_published.is_some_and(|v| !v)
    }

    pub fn has_section(&self) -> bool {
        self.section_id.is_some()
    }

    pub async fn update_section(
        &self,
        client: &reqwest::Client,
        section_id: Option<i64>,
    ) -> Result<Post> {
        let url = Url::parse(&format!(
            "{}/post_management/drafts/{}",
            &config().API_URL,
            self.id
        ))?;

        Ok(client
            .patch(url)
            .headers(config().HEADERS.clone())
            .json(&json!({ "section_id": section_id }))
            .retry()
            .send::<Post>()
            .await?)
    }

    pub fn total_engagement(&self) -> i32 {
        self.reaction_count + self.comment_count + self.child_comment_count
    }
}

impl Post {
    pub async fn get_tags(&self, client: &reqwest::Client) -> Result<Vec<Tag>> {
        let url = Url::parse(&format!("{}/post/{}/tag", &config().API_URL, self.id))?;

        Ok(client
            .get(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Vec<Tag>>()
            .await?)
    }

    pub async fn add_tag(&self, client: &reqwest::Client, tag: &Tag) -> Result<TagAssociation> {
        tag.add_to_post(client, self.id).await
    }
}
