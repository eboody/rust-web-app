use crate::prelude::*;

use model::Articles;

pub async fn add_subtitle(mm: &ModelManager, article: &Articles) -> Result<()> {
    let message = construct_message(article)?;

    let generated_subtitle = openai::chat(mm, message).await?;

    article
        .update_partial()
        .subtitle(Some(generated_subtitle))
        .update(mm.orm())
        .await?;

    Ok(())
}

fn construct_message(article: &Articles) -> Result<String> {
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

    let message = [
        "You are to generate just a subtitle for this article.",
        "Your response shouldn't be 'Subtitle: ...' but just the subtitle itself.",
        "Article:",
        title_line,
        content_line,
    ];

    Ok(message.join("\n"))
}
