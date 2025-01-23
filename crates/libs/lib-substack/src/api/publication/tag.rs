use crate::prelude::*;
use die_exit::die;
use lib_core::model::directus;
use lib_utils::retry::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: Uuid,
    pub publication_id: i64,
    pub name: String,
    pub slug: String,
    pub hidden: bool,
}

impl From<Tag> for directus::Tags {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
            publication_id: tag.publication_id,
            hidden: tag.hidden,
        }
    }
}

impl From<directus::Tags> for Tag {
    fn from(value: directus::Tags) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
            publication_id: value.publication_id,
            hidden: value.hidden,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagAssociation {
    pub id: Uuid,
    pub publication_id: i64,
    pub post_id: i64,
    pub post_tag_id: String,
}

impl Tag {
    pub async fn create(client: &reqwest::Client, name: String) -> Result<Self> {
        let url = Url::parse(&format!("{}/publication/post-tag", &config().API_URL))?;

        // Send the request
        let response = client
            .post(url)
            .headers(config().HEADERS.clone())
            .json(&json!({ "name": name }))
            .retry()
            .send::<Tag>()
            .await?;

        Ok(response)
    }

    pub async fn list(client: &reqwest::Client) -> Result<Vec<Tag>> {
        let url = Url::parse(&format!("{}/publication/post-tag", &config().API_URL))?;

        let response = client
            .get(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Vec<Tag>>()
            .await?;

        Ok(response)
    }

    pub async fn get(client: &reqwest::Client, id: Uuid) -> Result<Tag> {
        let url = Url::parse(&format!(
            "{}/publication/post-tag/{}",
            &config().API_URL,
            id
        ))?;

        let response = client
            .get(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Tag>()
            .await?;

        Ok(response)
    }

    pub async fn add_to_post(
        &self,
        client: &reqwest::Client,
        post_id: i64,
    ) -> Result<TagAssociation> {
        let url = Url::parse(&format!(
            "{}/post/{}/tag/{}",
            &config().API_URL,
            post_id,
            self.id
        ))?;

        // Send the request
        let response = client
            .post(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<TagAssociation>()
            .await?;

        Ok(response)
    }
}
