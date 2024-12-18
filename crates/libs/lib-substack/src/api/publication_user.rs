use crate::prelude::*;
use reqwest::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub handle: String,
  pub photo_url: String,
  pub bio: String,
  pub email: String,
  #[serde(with = "time::serde::rfc3339")]
  pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationUser {
  pub id: i64,
  pub publication_id: i64,
  pub user_id: i64,
  pub public: bool,
  #[serde(with = "time::serde::rfc3339")]
  pub created_at: OffsetDateTime,
  #[serde(with = "time::serde::rfc3339")]
  pub updated_at: OffsetDateTime,
  pub public_rank: i32,
  pub name: Option<String>,
  pub bio: Option<String>,
  pub photo_url: Option<String>,
  pub role: Role,
  pub is_primary: bool,
  pub show_badge: Option<bool>,
  pub is_in_notes_feed: bool,
  pub twitter_screen_name: Option<String>,
  pub email: Option<String>,
  pub user: User,
}

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Role within a Substack publication, determining user permissions and visibility
#[derive(
  Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString,
)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  /// Full publication access, including all settings
  Admin,

  /// Able to publish and edit posts, but cannot access publication settings
  Contributor,

  /// Can be listed as an author on posts, but cannot publish or access dashboard
  Byline,

  /// Publicly listed as a contributor to this publication
  Public,

  /// Not listed as a contributor to this publication
  Private,
}

impl Role {
  /// Returns true if the role has publishing permissions
  pub fn can_publish(&self) -> bool {
    matches!(self, Self::Admin | Self::Contributor)
  }

  /// Returns true if the role has access to publication settings
  pub fn can_access_settings(&self) -> bool {
    matches!(self, Self::Admin)
  }

  /// Returns true if the role is publicly visible
  pub fn is_public(&self) -> bool {
    matches!(self, Self::Admin | Self::Contributor | Self::Public)
  }

  /// Returns true if the role can be listed as an author
  pub fn can_be_author(&self) -> bool {
    matches!(self, Self::Admin | Self::Contributor | Self::Byline)
  }
}

impl PublicationUser {
  pub async fn list(client: &reqwest::Client) -> Result<Vec<Self>> {
    let url = Url::parse(&format!("{}/publication_user", &config().API_URL))?;

    Ok(
      client
        .get(url)
        .headers(config().HEADERS.clone())
        .send()
        .await?
        .json::<Vec<Self>>()
        .await?,
    )
  }

  pub async fn get_by_user_id(
    client: &reqwest::Client,
    user_id: i64,
  ) -> Result<Option<Self>> {
    let users = Self::list(client).await?;
    Ok(users.into_iter().find(|u| u.user_id == user_id))
  }

  pub fn is_admin(&self) -> bool {
    self.role == Role::Admin
  }
}

// Optional: Add a type for filtering/querying users
#[derive(Debug, Default)]
pub struct PublicationUserQuery {
  pub role: Option<Role>,
  pub is_public: Option<bool>,
  pub is_primary: Option<bool>,
}

impl PublicationUserQuery {
  pub async fn execute(
    self,
    client: &reqwest::Client,
  ) -> Result<Vec<PublicationUser>> {
    let users = PublicationUser::list(client).await?;

    Ok(
      users
        .into_iter()
        .filter(|user| {
          self.role.as_ref().map(|r| r == &user.role).unwrap_or(true)
            && self.is_public.map(|p| p == user.public).unwrap_or(true)
            && self
              .is_primary
              .map(|p| p == user.is_primary)
              .unwrap_or(true)
        })
        .collect(),
    )
  }
}

// Example usage:
// let admins = PublicationUserQuery {
//     role: Some("admin".to_string()),
//     is_public: Some(true),
//     ..Default::default()
// }.execute(client).await?;
