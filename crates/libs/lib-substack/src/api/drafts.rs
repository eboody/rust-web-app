use crate::Error;
pub use crate::post::{Audience, ByLine, Type};
use crate::{
    md_to_prosemirror,
    prelude::*,
    prose_mirror::{self, Node},
    transform_endnotes_for_substack, transform_to_substack_format,
};
use lib_utils::retry::RetryableRequest;
use ormlite::types::Uuid;
use regex::Regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Debug, Default)]
pub struct Request {
    pub audience: Audience,
    pub draft_body: Body,
    pub draft_title: String,
    pub draft_subtitle: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub draft_section_id: Option<i64>,
    pub draft_bylines: Vec<ByLine>,
    pub section_chosen: bool,
    pub draft_podcast_duration: Option<String>,
    pub draft_podcast_preview_upload_id: Option<String>,
    pub draft_podcast_upload_id: Option<String>,
    pub draft_podcast_url: String,
    pub draft_video_upload_id: Option<String>,
    pub draft_voiceover_upload_id: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub social_title: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub post_date: Option<OffsetDateTime>,
}

impl Request {
    pub async fn post(&self, client: &reqwest::Client) -> Result<Response> {
        let url = Url::parse(&format!("{}/drafts", &config().API_URL))?;
        Ok(client
            .post(url)
            .headers(config().HEADERS.clone())
            .json(self)
            .retry()
            .send::<Response>()
            .await?)
    }

    pub async fn put(&self, client: &reqwest::Client, draft_id: i64) -> Result<Response> {
        let url = Url::parse(&format!("{}/drafts/{}", &config().API_URL, draft_id))?;
        Ok(client
            .put(url)
            .headers(config().HEADERS.clone())
            .json(self)
            .retry()
            .send::<Response>()
            .await?)
    }

    pub async fn get(client: &reqwest::Client, draft_id: i64) -> Result<Response> {
        let url = Url::parse(&format!("{}/drafts/{}", &config().API_URL, draft_id))?;
        Ok(client
            .get(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Response>()
            .await?)
    }

    pub fn new_with_byline(byline_id: i64) -> Self {
        Self {
            audience: Audience::Everyone,
            type_: Type::Newsletter,
            draft_bylines: vec![ByLine {
                id: byline_id,
                is_guest: false,
            }],
            ..Default::default()
        }
    }

    fn remove_em_link(content: &mut [Node]) {
        // Iterate over the nodes in `content`
        for node in content.iter_mut() {
            // Check and modify the marks if they exist
            if let Some(ref mut marks) = node.marks {
                let contains_link_and_em = marks
                    .iter()
                    .any(|m| m.mark_type == prose_mirror::MarkType::Link)
                    && marks
                        .iter()
                        .any(|m| m.mark_type == prose_mirror::MarkType::Em);

                if contains_link_and_em {
                    marks.retain(|m| m.mark_type != prose_mirror::MarkType::Em);
                }
            }

            // Recursively handle nested content
            if let Some(ref mut nested_content) = node.content {
                Self::remove_em_link(nested_content);
            }
        }
    }

    pub async fn export_from_article(
        mm: &ModelManager,
        article: &model::Articles,
        byline_id: i64,
    ) -> Result<Response> {
        let content = article
            .body
            .as_deref()
            .ok_or_else(|| Error::NoArticleContent)?;

        // Remove HTML comments
        let re = regex::Regex::new(r"<!--.*?-->").unwrap();
        let content = re.replace_all(content, "").to_string();

        // Process content and endnotes
        let mut doc = md_to_prosemirror(&content)?;

        // Remove links with emphasis
        Self::remove_em_link(doc.content.as_mut());

        //tracing::debug!("->> {:<12} - doc:\n{:#?}", module_path!(), doc);
        let mut doc: prose_mirror::Node = doc.into();

        transform_to_substack_format(&mut doc);

        if let Some(endnotes) = &article.endnotes {
            let endnotes = md_to_prosemirror(endnotes)?;
            let mut endnotes = transform_endnotes_for_substack(&endnotes.into());
            if let Some(mut c) = doc.content.clone() {
                c.append(&mut endnotes);
                doc.content = Some(c);
            }
        }

        //tracing::debug!("->> {:<12} - section:\n{:#?}", file!(), article);

        let section = article
            .section
            .map(|section_id| async move {
                let section = model::Sections::select()
                    .where_("id = ?")
                    .bind(section_id)
                    .fetch_one(mm.orm())
                    .await
                    .ok()?;

                Some(section)
            })
            .unwrap_or_else(|| {
                panic!(
                    "Failed to fetch section {:#?} for article: {:?}",
                    article.section, article.title
                )
            })
            .await;

        let body = json::to_string(&doc)?.replace(
            "/assets/",
            format!("{}/assets/", &config().DIRECTUS_URL).as_str(),
        );

        let body =
            Regex::new(r"https?://(?:www\.)?theobjectivestandard\.com/(?:[^/\s)]+/)*([^/\s)]+)")
                .unwrap()
                .replace_all(&body, "/p/$1");

        //let body = Regex::new(r"\s*?_?Dowload the .*? of this article\._?")
        //    .unwrap()
        //    .replace_all(&body, "");
        //
        //let body = Regex::new(r"\s*?For the application of these principles.*?\.")
        //    .unwrap()
        //    .replace_all(&body, "");
        let article = model::Articles::select()
            .where_("articles.id = ?")
            .bind(article.id)
            .join(model::Articles::author())
            .fetch_one(mm.orm())
            .await?;

        let subtitle = if let Some(first_name) = &article.author.first_name
            && let Some(last_name) = &article.author.last_name
        {
            Some(format!("By {} {}", first_name, last_name))
        } else if let Some(subtitle) = article.subtitle {
            Some(subtitle)
        } else {
            Some("".to_string())
        };

        let request = Self {
            audience: Audience::Everyone,
            draft_body: Body(body.to_string()),
            draft_title: article.title.clone().ok_or_else(|| Error::NoArticleTitle)?,
            draft_subtitle: subtitle.unwrap_or_default(),
            draft_bylines: vec![ByLine {
                id: byline_id,
                is_guest: false,
            }],
            type_: Type::Newsletter,
            draft_section_id: section.clone().and_then(|s| s.substack_id),
            section_chosen: section.is_some(),
            draft_podcast_url: "".to_string(),
            draft_video_upload_id: None,
            draft_podcast_upload_id: None,
            draft_podcast_duration: None,
            draft_podcast_preview_upload_id: None,
            draft_voiceover_upload_id: None,
            cover_image: None,
            description: None,
            social_title: None,
            post_date: article
                .date_published
                .map(|d| d.with_time(Time::MIDNIGHT).assume_utc()),
        };

        info!(
            "Exporting article to Substack draft: {}",
            request.draft_title
        );

        request.post(mm.reqwest()).await
    }

    pub async fn delete(client: &reqwest::Client, draft_id: i64) -> Result<()> {
        let url = Url::parse(&format!("{}/drafts/{}", &config().API_URL, draft_id))?;
        client
            .delete(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<()>()
            .await?;
        Ok(())
    }

    pub async fn publish(
        client: &reqwest::Client,
        draft_id: i64,
        publish_args: PublishArgs,
    ) -> Result<()> {
        let url = Url::parse(&format!(
            "{}/drafts/{}/publish",
            &config().API_URL,
            draft_id
        ))?;

        let url = url.clone();

        client
            .post(url)
            .body(json::to_string(&publish_args)?)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Response>()
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Debug, Default)]
pub struct PublishArgs {
    pub send: bool,
    pub share_automatically: bool,
}

impl TryFrom<model::Articles> for Request {
    type Error = crate::Error;
    fn try_from(article: model::Articles) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            audience: Audience::Everyone,
            type_: Type::Newsletter,
            draft_body: article.as_ref().try_into()?,
            draft_title: article.title.clone().unwrap_or_default(),
            draft_subtitle: article.subtitle.clone().unwrap_or_default(),
            ..Default::default()
        })
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Body(pub String);

impl TryFrom<&model::Articles> for Body {
    type Error = crate::Error;
    fn try_from(article: &model::Articles) -> Result<Self> {
        if let Some(content) = &article.body {
            let doc = md_to_prosemirror(content)?;
            let mut doc: prose_mirror::Node = doc.into();
            transform_to_substack_format(&mut doc);
            if let Some(endnotes) = &article.endnotes {
                let endnotes = md_to_prosemirror(endnotes)?;
                let mut endnotes = transform_endnotes_for_substack(&endnotes.into());
                doc.content.as_mut().unwrap().append(&mut endnotes);
            }
            Ok(Body(json::to_string(&doc)?))
        } else {
            Err(Error::NoArticleContent)
        }
    }
}

impl TryFrom<model::Articles> for Body {
    type Error = crate::Error;
    fn try_from(article: model::Articles) -> Result<Self> {
        if let Some(content) = &article.body {
            let doc = md_to_prosemirror(content)?;
            let mut doc: prose_mirror::Node = doc.into();
            transform_to_substack_format(&mut doc);
            if let Some(endnotes) = &article.endnotes {
                let endnotes = md_to_prosemirror(endnotes)?;
                let mut endnotes = transform_endnotes_for_substack(&endnotes.into());
                doc.content.as_mut().unwrap().append(&mut endnotes);
            }
            Ok(Body(json::to_string(&doc)?))
        } else {
            Err(Error::NoArticleContent)
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize, ormlite::Model)]
pub struct Response {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: Type,
    pub draft_title: String,
    pub draft_subtitle: String,
    pub audience: Audience,
    pub publication_id: i64,
    pub word_count: Option<i64>,
    pub draft_body: String,
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub draft_created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub draft_updated_at: Option<OffsetDateTime>,
    pub uuid: Uuid,

    pub cover_image: Option<String>,
    pub default_comment_sort: Option<String>,
    pub description: Option<String>,
    pub draft_podcast_duration: Option<String>,
    pub draft_podcast_preview_upload_id: Option<String>,
    pub draft_podcast_upload_id: Option<String>,
    pub draft_podcast_url: Option<String>,
    pub draft_section_id: Option<i64>,
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
    pub search_engine_description: Option<String>,
    pub search_engine_title: Option<String>,
    pub section_id: Option<i64>,
    pub should_send_email: Option<bool>,
    pub should_send_free_preview: Option<bool>,
    pub show_guest_bios: Option<bool>,
    pub slug: Option<String>,
    pub social_title: Option<String>,
    pub subscriber_set_id: Option<String>,
    pub subtitle: Option<String>,
    pub syndicate_to_section_id: Option<i64>,
    pub should_syndicate_to_other_feed: Option<bool>,
    pub title: Option<String>,
    pub write_comment_permissions: Option<String>,
    pub section_chosen: Option<bool>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub post_date: Option<OffsetDateTime>,
}
impl Response {
    //pub async fn put(&self, client: &reqwest::Client) -> Result<Response> {
    //    let url = Url::parse(&format!("{}/drafts/{}", &config().API_URL))?;
    //    Ok(client
    //        .put(url)
    //        .headers(config().HEADERS.clone())
    //        .json(self)
    //        .retry()
    //        .send::<Response>()
    //        .await?)
    //}
}

impl Response {
    pub fn into_substack_draft(self, articles_id: Uuid) -> model::SubstackDraft {
        model::SubstackDraft {
            id: Uuid::new_v4(),
            articles_id: Some(articles_id),
            substack_draft_id: self.id,
            substack_uuid: self.uuid,
            draft_title: self.draft_title,
            draft_subtitle: Some(self.draft_subtitle),
            draft_type: self.type_.to_string(),
            audience: self.audience.to_string(),
            section_chosen: self.section_chosen,
            publication_id: self.publication_id,
            word_count: self.word_count,
            draft_body: json::from_str(&self.draft_body)
                .expect("Failed to deserialize draft body in Response into SubstackDraft"),
            draft_created_at: self.draft_created_at,
            draft_updated_at: self.draft_updated_at,
            status: "draft".to_string(),
            sync_status: "synced".to_string(),
            last_sync_error: None,
            last_synced_at: OffsetDateTime::now_utc(),
            section_id: self.section_id,
            user_created: None,
            date_created: None,
            user_updated: None,
            date_updated: None,
            cover_image: self.cover_image,
            default_comment_sort: self.default_comment_sort,
            description: self.description,
            draft_podcast_duration: self.draft_podcast_duration,
            draft_podcast_preview_upload_id: self.draft_podcast_preview_upload_id,
            draft_podcast_upload_id: self.draft_podcast_upload_id,
            draft_podcast_url: self.draft_podcast_url,
            draft_section_id: self.draft_section_id,
            draft_video_upload_id: self.draft_video_upload_id,
            draft_voiceover_upload_id: self.draft_voiceover_upload_id,
            editor_v2: self.editor_v2,
            email_sent_at: self.email_sent_at,
            explicit: self.explicit,
            free_podcast_duration: self.free_podcast_duration,
            free_podcast_url: self.free_podcast_url,
            has_dismissed_tk_warning: self.has_dismissed_tk_warning,
            hide_from_feed: self.hide_from_feed,
            is_metered: self.is_metered,
            is_published: self.is_published,
            podcast_duration: self.podcast_duration,
            podcast_episode_number: self.podcast_episode_number,
            podcast_episode_type: self.podcast_episode_type,
            podcast_season_number: self.podcast_season_number,
            podcast_url: self.podcast_url,
            search_engine_description: self.search_engine_description,
            search_engine_title: self.search_engine_title,
            should_send_email: self.should_send_email,
            should_send_free_preview: self.should_send_free_preview,
            show_guest_bios: self.show_guest_bios,
            slug: self.slug,
            social_title: self.social_title,
            subscriber_set_id: self.subscriber_set_id,
            subtitle: self.subtitle,
            syndicate_to_section_id: self.syndicate_to_section_id,
            should_syndicate_to_other_feed: self.should_syndicate_to_other_feed,
            title: self.title,
            write_comment_permissions: self.write_comment_permissions,

            post_date: self.post_date,
        }
    }
}
impl From<model::SubstackDraft> for Response {
    fn from(draft: model::SubstackDraft) -> Self {
        Response {
            id: draft.substack_draft_id,
            type_: draft.draft_type.parse().unwrap_or_default(),
            draft_title: draft.draft_title,
            draft_subtitle: draft.draft_subtitle.unwrap_or_default(),
            audience: draft.audience.parse().unwrap_or_default(),
            section_chosen: draft.section_chosen,
            publication_id: draft.publication_id,
            word_count: draft.word_count,
            draft_body: json::to_string(&draft.draft_body).expect(
                "Failed to serialize draft body in From<model::SubstackDraft> for Response",
            ),
            draft_created_at: draft.draft_created_at,
            draft_updated_at: draft.draft_updated_at,
            uuid: draft.substack_uuid,
            cover_image: draft.cover_image,
            default_comment_sort: draft.default_comment_sort,
            description: draft.description,
            draft_podcast_duration: draft.draft_podcast_duration,
            draft_podcast_preview_upload_id: draft.draft_podcast_preview_upload_id,
            draft_podcast_upload_id: draft.draft_podcast_upload_id,
            draft_podcast_url: draft.draft_podcast_url,
            draft_section_id: draft.draft_section_id,
            draft_video_upload_id: draft.draft_video_upload_id,
            draft_voiceover_upload_id: draft.draft_voiceover_upload_id,
            editor_v2: draft.editor_v2,
            email_sent_at: draft.email_sent_at,
            explicit: draft.explicit,
            free_podcast_duration: draft.free_podcast_duration,
            free_podcast_url: draft.free_podcast_url,
            has_dismissed_tk_warning: draft.has_dismissed_tk_warning,
            hide_from_feed: draft.hide_from_feed,
            is_metered: draft.is_metered,
            is_published: draft.is_published,
            podcast_duration: draft.podcast_duration,
            podcast_episode_number: draft.podcast_episode_number,
            podcast_episode_type: draft.podcast_episode_type,
            podcast_season_number: draft.podcast_season_number,
            podcast_url: draft.podcast_url,
            search_engine_description: draft.search_engine_description,
            search_engine_title: draft.search_engine_title,
            section_id: draft.section_id,
            should_send_email: draft.should_send_email,
            should_send_free_preview: draft.should_send_free_preview,
            show_guest_bios: draft.show_guest_bios,
            slug: draft.slug,
            social_title: draft.social_title,
            subscriber_set_id: draft.subscriber_set_id,
            subtitle: draft.subtitle,
            syndicate_to_section_id: draft.syndicate_to_section_id,
            should_syndicate_to_other_feed: draft.should_syndicate_to_other_feed,
            title: draft.title,
            write_comment_permissions: draft.write_comment_permissions,
            post_date: draft.post_date,
        }
    }
}
