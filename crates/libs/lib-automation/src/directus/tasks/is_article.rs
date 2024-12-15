use model::directus::Sections;

use crate::prelude::*;

pub async fn is_article(mm: &ModelManager, text: &str) -> Result<bool> {
  let message = construct_message(mm, text).await?;

  let is_article = openai::chat(mm.reqwest(), message).await?;

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
    "You are to return either true or false if this is an article.",
    "Ask yourself:",
    "Would this be something evergreen that I would post on substack as an article or is it a tweet or an administrative post?",
    "Would this article be something that fits into one of these sections?",
    &sections.join(", "),
    "You are to exclusively output JUST true or false and nothing else.",
    "Article Text:",
    text,
  ];

  Ok(message.join("\n"))
}
