use crate::prelude::*;

use model::Articles;

pub async fn add_tags(mm: &ModelManager, article: &Articles) -> Result<()> {
    let message = construct_message(mm, article).await?;

    let tags = openai::chat(mm, message)
        .await?
        .split(',')
        .map(|tag| tag.trim().to_owned())
        .collect::<Vec<String>>();

    article
        .update_partial()
        .tags(json::to_value(&tags).ok())
        .update(mm.orm())
        .await?;

    Ok(())
}

async fn construct_message(mm: &ModelManager, article: &Articles) -> Result<String> {
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

    let mut message = vec![
        "You are to generate a comma-separated list of tags for an article.",
        "Ask yourself:",
        "Would someone searching for this tag expect to find my article?",
        "Does the tag add value by highlighting a unique aspect of the article?",
        "You are to exclusively output JUST the tags and nothing else.",
        "If the answer is yes, it's a good tag!",
        "Specificity ensures your tags aren't just labels but tools for discovery and organization.",
        "Tagging Article:",
        title_line,
        content_line,
    ];

    let article_with_author = Articles::select()
        .select("author.first_name, author.last_name")
        .join(Articles::author())
        .where_("articles.id = ?")
        .bind(article.id)
        .fetch_one(mm.orm())
        .await?;

    let author_line = article_with_author
        .author
        .first_name
        .as_ref()
        .zip(article_with_author.author.last_name.as_ref())
        .map(|(first_name, last_name)| format!("Author: {} {}", first_name, last_name))
        .unwrap_or_default();

    message.push(&author_line);

    Ok(message.join("\n"))
}
