//https://theobjectivestandard.substack.com/api/v1/publication/sections/170375
//PATCH
//payload:
//{
//  "name": "some titlea",
//  "description": "my description",
//  "email_from_name": "",
//  "slug": "some-title",
//  "hide_posts_from_pub_listings": "false",
//  "is_default_on": "false",
//  "logo_url": "https://substack-post-media.s3.amazonaws.com/public/images/edb39314-c55b-445c-96f7-e9fc33b6e9f0_200x200.png",
//  "cover_photo_url": null
//}

//https://theobjectivestandard.substack.com/api/v1/publication/sections/
//POST
//{
//  "name": "section title",
//  "description": "som e descriptionnn",
//  "email_from_name": "",
//  "hide_posts_from_pub_listings": "false",
//  "is_default_on": "true",
//  "port_existing_signups": "true",
//  "logo_url": "https://substack-post-media.s3.amazonaws.com/public/images/b43fb784-ee4b-42e1-b8c2-e66e9550b00d_200x200.png",
//  "cover_photo_url": null
//}

use crate::prelude::*;
use lib_utils::retry::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SectionWrapper {
    pub section: Section,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub email_from_name: String,
    pub slug: Option<String>,
    #[serde(default = "default_false")]
    pub hide_posts_from_pub_listings: bool,
    #[serde(default = "default_false")]
    pub is_default_on: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_existing_signups: Option<bool>,
    pub logo_url: Option<String>,
    pub cover_photo_url: Option<String>,
    #[serde(skip_serializing)]
    pub email_banner_url: Option<String>,
    #[serde(skip_serializing)]
    pub hide_from_navbar: Option<bool>,
    #[serde(skip_serializing)]
    pub hide_intro_subtitle: Option<bool>,
    #[serde(skip_serializing)]
    pub hide_intro_title: Option<bool>,
    #[serde(skip_serializing)]
    pub ignore_publication_email_settings: Option<bool>,
    #[serde(skip_serializing)]
    pub is_live: Option<bool>,
    #[serde(skip_serializing)]
    pub is_podcast: Option<bool>,
    #[serde(skip_serializing)]
    pub port_status: Option<String>,
    #[serde(skip_serializing)]
    pub publication_id: Option<i64>,
    #[serde(skip_serializing)]
    pub sibling_rank: Option<i32>,
    #[serde(skip)]
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(skip)]
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

fn default_false() -> bool {
    false
}

impl Section {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name,
            description,
            email_from_name: String::new(),
            slug: None,
            hide_posts_from_pub_listings: false,
            is_default_on: false,
            port_existing_signups: None,
            logo_url: None,
            cover_photo_url: None,
            email_banner_url: None,
            hide_from_navbar: None,
            hide_intro_subtitle: None,
            hide_intro_title: None,
            ignore_publication_email_settings: None,
            is_live: None,
            is_podcast: None,
            port_status: None,
            publication_id: None,
            sibling_rank: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub async fn create(self, client: &reqwest::Client) -> Result<Section> {
        let url = Url::parse(&format!("{}/publication/sections", &config().API_URL))?;
        let body = client
            .post(url)
            .headers(config().HEADERS.clone())
            .json(&self)
            .retry()
            .send::<SectionWrapper>()
            .await?;

        Ok(body.section)
    }

    pub async fn update(&self, client: &reqwest::Client, section_id: i64) -> Result<Section> {
        let url = Url::parse(&format!(
            "{}/publication/sections/{}",
            &config().API_URL,
            section_id
        ))?;
        Ok(client
            .patch(url)
            .headers(config().HEADERS.clone())
            .json(self)
            .retry()
            .send::<Section>()
            .await?)
    }

    pub async fn delete(&self, client: &reqwest::Client, section_id: i64) -> Result<Section> {
        let url = Url::parse(&format!(
            "{}/publication/sections/{}",
            &config().API_URL,
            section_id
        ))?;

        Ok(client
            .delete(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Section>()
            .await?)
    }

    pub async fn list(client: &reqwest::Client) -> Result<Vec<Section>> {
        let url = Url::parse(&format!("{}/publication/sections", &config().API_URL))?;
        Ok(client
            .get(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Vec<Section>>()
            .await?)
    }

    // Builder methods
    pub fn with_logo(mut self, logo_url: String) -> Self {
        self.logo_url = Some(logo_url);
        self
    }

    pub fn with_cover_photo(mut self, cover_photo_url: String) -> Self {
        self.cover_photo_url = Some(cover_photo_url);
        self
    }

    pub fn with_email_from_name(mut self, email_from_name: String) -> Self {
        self.email_from_name = email_from_name;
        self
    }

    pub fn set_default_on(mut self, is_default: bool) -> Self {
        self.is_default_on = is_default;
        self
    }

    pub fn set_hide_posts_from_pub_listings(mut self, hide: bool) -> Self {
        self.hide_posts_from_pub_listings = hide;
        self
    }

    pub fn with_slug(mut self, slug: String) -> Self {
        self.slug = Some(slug);
        self
    }
}

// Helper methods for working with sections
impl Section {
    pub fn is_active(&self) -> bool {
        self.is_live.unwrap_or(false)
    }

    pub fn has_logo(&self) -> bool {
        self.logo_url.is_some()
    }

    pub fn port_successful(&self) -> bool {
        self.port_status.as_deref() == Some("success")
    }
}
