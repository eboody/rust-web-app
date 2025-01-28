use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default, ormlite::Enum)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    #[default]
    Everyone,
    Founding,
    OnlyPaid,
    OnlyFree,
}
