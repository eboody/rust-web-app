use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use uuid::Uuid;

#[serde_as]
#[derive(Clone, Debug, ormlite::Model, Deserialize, Serialize)]
#[ormlite(table = "substack_tags")]
pub struct Tags {
    #[ormlite(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub publication_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub hidden: bool,
}
