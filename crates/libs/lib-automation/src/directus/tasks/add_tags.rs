use crate::prelude::*;

use model::directus::Articles;

pub async fn add_tags(mm: &ModelManager, article: &Articles) -> Result<()> {
	let message = construct_message(article)?;

	let tags = openai::chat(mm.reqwest(), message)
		.await?
		.split(',')
		.map(|tag| tag.trim().to_owned())
		.collect::<Vec<String>>();

	let json_tags = json::json!(tags);

	article
		.update_partial()
		.tags(Some(json_tags))
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

	let mut message = vec![
		"You are to generate a comma-separated list of tags for an article.",
		"Ask yourself:",
		"Would someone searching for this tag expect to find my article?",
		"Does the tag add value by highlighting a unique aspect of the article?",
		"If the answer is yes, it's a good tag!",
		"Specificity ensures your tags aren't just labels but tools for discovery and organization.",
		"Tagging Article:",
		title_line,
		content_line,
	];

	let author_line = article
		.author
		.first_name
		.as_ref()
		.zip(article.author.last_name.as_ref())
		.map(|(first_name, last_name)| {
			format!("Author: {} {}", first_name, last_name)
		})
		.unwrap_or_default();

	message.push(&author_line);

	Ok(message.join("\n"))
}
