use crate::prelude::*;

use model::directus::{Articles, Sections};

pub async fn select_section(mm: &ModelManager, article: &Articles) -> Result<()> {
  let mut sections = Sections::select()
    .fetch_all(mm.orm())
    .await?
    .into_iter()
    .map(|s| s.display_string)
    .collect::<Vec<_>>();
  sections.push("Unknown".to_string());

  let message = construct_message(article, sections)?;

  let selected_section_string = openai::chat(mm.reqwest(), message).await?;

  if selected_section_string == "Unknown" {
    return Ok(());
  }

  let selected_section = Sections::select()
    .where_("display_string = ?")
    .bind(&selected_section_string)
    .fetch_one(mm.orm())
    .await?;

  article
    .update_partial()
    .section(Some(selected_section.id))
    .update(mm.orm())
    .await?;

  Ok(())
}

fn construct_message(article: &Articles, sections: Vec<String>) -> Result<String> {
  let title = article
    .title
    .as_ref()
    .ok_or(Error::NoTitleInArticle(article.id))?;
  let content = article
    .body
    .as_ref()
    .ok_or(Error::NoContentInArticle(article.id))?;

  let title_line = &format!("Title: {}", title);
  let content_line = &format!("Content: {}", content);

  let sections = sections.join(", ");

  let message = [
    "You are to categorize this article into a section.",
    "The only valid response is one of these:",
    &sections,
    "Tagging Article:",
    title_line,
    content_line,
  ];

  Ok(message.join("\n"))
}
