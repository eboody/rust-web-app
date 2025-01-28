use crate::prelude::*;
use lib_substack::{PublicationUser, drafts};

pub async fn get_byline(mm: &ModelManager, user: &model::Users) -> Option<drafts::ByLine> {
    if let Some(first_name) = &user.first_name
        && let Some(last_name) = &user.last_name
    {
        PublicationUser::get_by_name(mm.reqwest(), &format!("{} {}", first_name, last_name))
            .await
            .ok()
            .flatten()
            .map(|u| u.to_byline())
    } else {
        None
    }
}
