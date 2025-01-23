use crate::{Post, prelude::*};
use lib_utils::retry::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub posts: Vec<Post>,
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
}

//use crate::prelude::*;
//use reqwest::Url;
//use serde::{Deserialize, Serialize};
//
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryParams {
    pub offset: u32,
    pub limit: u32,
    #[serde(default = "default_order_by")]
    pub order_by: String,
    #[serde(default = "default_order_direction")]
    pub order_direction: String,
}

fn default_order_by() -> String {
    "draft_created_at".to_string()
}

fn default_order_direction() -> String {
    "desc".to_string()
}

impl QueryParams {
    pub fn new(offset: u32, limit: u32) -> Self {
        Self {
            offset,
            limit,
            ..Default::default()
        }
    }

    pub fn order_by(mut self, field: impl Into<String>) -> Self {
        self.order_by = field.into();
        self
    }

    pub fn order_direction(mut self, direction: impl Into<String>) -> Self {
        self.order_direction = direction.into();
        self
    }
}

impl Response {
    pub async fn fetch(client: &reqwest::Client, params: &QueryParams) -> Result<Self> {
        let url =
            Url::parse_with_params(&format!("{}/post_management/drafts", &config().API_URL), &[
                ("offset", params.offset.to_string()),
                ("limit", params.limit.to_string()),
                ("order_by", params.order_by.clone()),
                ("order_direction", params.order_direction.clone()),
            ])?;

        Ok(client
            .get(url)
            .headers(config().HEADERS.clone())
            .retry()
            .send::<Response>()
            .await?)
    }
}

// Add pagination helper
#[derive(Debug)]
pub struct PaginatedDrafts {
    client: reqwest::Client,
    params: QueryParams,
    total: Option<u32>,
}

impl PaginatedDrafts {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            params: QueryParams::default(),
            total: None,
        }
    }

    pub async fn fetch_page(&mut self) -> Result<Response> {
        let response = Response::fetch(&self.client, &self.params).await?;
        self.total = Some(response.total);
        Ok(response)
    }

    pub fn next_page(&mut self) -> bool {
        if let Some(total) = self.total {
            let next_offset = self.params.offset + self.params.limit;
            if next_offset < total {
                self.params.offset = next_offset;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
