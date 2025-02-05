use model::Sections;

use crate::prelude::*;

pub async fn is_article(mm: &ModelManager, text: &str) -> Result<bool> {
    let message = construct_message(mm, text).await?;

    let is_article = openai::chat(mm, message).await?;

    let is_article = is_article.eq_ignore_ascii_case("true");

    Ok(is_article)
}

async fn construct_message(mm: &ModelManager, text: &str) -> Result<String> {
    let sections = Sections::select()
        .fetch_all(mm.orm())
        .await?
        .into_iter()
        .map(|s| s.display_string)
        .collect::<Vec<_>>();

    let message = [
        "You're going to get a *portion* of a post.",
        "Using just this portion of a post, you are to return either true or false if the whole post is likey to be an article or opinion piece.",
        "Ask yourself:",
        "Would this be something evergreen that I would post on substack as an article or review or is it a tweet or an administrative post?",
        "Would this article, review, or opinion piece be something that fits into one of these sections?",
        &sections.join(", "),
        "You are to exclusively output JUST true or false and nothing else.",
        "Article Text:",
        text,
    ];

    Ok(message.join("\n"))
}
