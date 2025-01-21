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

    let selected_section_string = openai::chat(mm, message).await?;

    //debug!("Got selected section string: {}", selected_section_string);

    if selected_section_string == "Unknown" {
        return Ok(());
    }

    //println!("selected_section_string: {:#?}", selected_section_string);
    let selected_section = Sections::select()
        .where_("display_string = ?")
        .bind(&selected_section_string)
        .fetch_one(mm.orm())
        .await
        .unwrap_or_else(|_| panic!("Failed to fetch section: {}", selected_section_string));

    //debug!("Got selected section: {:?}", selected_section);

    article
        .update_partial()
        .section(Some(selected_section.id))
        .update(mm.orm())
        .await
        .expect("Failed to update article with section");

    //debug!("Updated article with section");

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

    let sections = sections.join(",\n");

    let message = [
        "You are to categorize this article into a section. No markdown, just plain text. DO NOT RETURN AN EMPTY STRING!",
        "The ONLY valid response is EXACTLY one of these and NOTHING else :",
        &sections,
        "\n\n",
        "Again, here is a list of the only valid response. Pick one of these:",
        &sections,
        "\n\n
      Tagging Article:",
        title_line,
        content_line,
    ];

    Ok(message.join("\n"))
}
