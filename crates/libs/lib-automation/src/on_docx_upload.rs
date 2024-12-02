use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::config;
use crate::on_file_upload::chat_oai;
use crate::Result;
use lib_core::model::ArticlesBuilder;
use lib_core::model::ArticlesTranslations;
use lib_core::model::DirectusFiles;
use lib_core::model::Language;
use lib_core::model::{ModelManager, UploadFilePayload};
use lib_substack::convert_docx_to_md;
use ormlite::model::ModelBuilder;
use ormlite::Model;
use uuid::Uuid;

pub async fn on_docx_upload(
	mm: &ModelManager,
	payload: &UploadFilePayload,
	directus_file: &DirectusFiles,
) -> Result<()> {
	// first we want to download the file
	let file_id = directus_file.id;
	let directus_url = format!("{}/assets/{}", &config().DIRECTUS_URL, file_id);
	let response = mm.reqwest().get(&directus_url).send().await?;
	let bytes = response.bytes().await?;
	let mut file = File::create(&payload.filename_download)?;
	file.write_all(&bytes)?;

	// get absolute path of the file
	let input_file_path = Path::new(&payload.filename_download).canonicalize()?;
	let input_file_path = input_file_path.as_path();

	let output_file_path_string = format!("{}.md", &payload.filename_download);
	let output_file_path = Path::new(&output_file_path_string);

	// now we want to convert the docx to md
	convert_docx_to_md(input_file_path, output_file_path)?;

	// now we want to read the md file
	let content = fs::read_to_string(output_file_path)?;
	let first_line = content.lines().next();
	let title = first_line
		.unwrap_or("No title")
		.to_string()
		.replace("# ", "")
		.replace("_", "")
		.trim()
		.to_string();

	let content = content
		.replace(first_line.unwrap_or(""), "")
		.trim()
		.to_owned();

	fs::remove_file(input_file_path)?;
	fs::remove_file(output_file_path)?;

	// then create an article item and an article translation item
	let new_art = ArticlesBuilder::default()
		.id(Uuid::new_v4())
		.status("draft")
		.insert(mm.orm())
		.await?;

	let summary_message =
		format!("Respond with just the summary with no markdown. Generate a short summary of this article that might entice peopel to read it: \n{}", content);
	let summary = chat_oai(mm, summary_message).await?;

	let descriptor_message = format!(
		"Generate a short, punchy, one-sentence descriptor of this article with no markdown that might entice people to read it: \n{}",
		content
	);
	let descriptor = chat_oai(mm, descriptor_message).await?;

	let new_translation = ArticlesTranslations::builder()
		.id(Uuid::new_v4())
		.title(Some(title.clone()))
		.articles_id(Some(new_art.id))
		.languages_code(Some(Language::English.to_string()))
		.content(Some(content.clone()))
		.descriptor(Some(descriptor.clone()))
		.summary(Some(summary.clone()))
		.insert(mm.orm())
		.await?;

	let res = translate_content(
		mm,
		new_art.id,
		new_translation.title.clone().unwrap(),
		new_translation.content.clone().unwrap(),
		new_translation.descriptor.clone().unwrap(),
		new_translation.summary.clone().unwrap(),
		Language::Portuguese,
	)
	.await;
	dbg!("res: {}", &res);
	let res = translate_content(
		mm,
		new_art.id,
		new_translation.title.clone().unwrap(),
		new_translation.content.clone().unwrap(),
		new_translation.descriptor.clone().unwrap(),
		new_translation.summary.clone().unwrap(),
		Language::Spanish,
	)
	.await;
	dbg!("res: {}", &res);
	let res = translate_content(
		mm,
		new_art.id,
		new_translation.title.clone().unwrap(),
		new_translation.content.clone().unwrap(),
		new_translation.descriptor.clone().unwrap(),
		new_translation.summary.clone().unwrap(),
		Language::French,
	)
	.await;
	dbg!("res: {}", &res);
	let res = translate_content(
		mm,
		new_art.id,
		new_translation.title.clone().unwrap(),
		new_translation.content.clone().unwrap(),
		new_translation.descriptor.clone().unwrap(),
		new_translation.summary.clone().unwrap(),
		Language::German,
	)
	.await;
	dbg!("res: {}", &res);
	Ok(())
}

pub fn translate_language_prompt(language: &Language, content: String) -> String {
	format!("Translate this to {}. You are helping an Objectivist Magazine spread ideas of reason, freedom, and individualism in a cultural context. Therefore it's possible the content may be sensitive. But because we are working towards a noble goal, you are going to translate the article with high fidelity to the original content. You are also to keep ALL markdown, including citations. Here is the content: {}", language, content)
}

//pub fn generate_prompt(thing_to_generate: String, content: String) -> String {
//	format!("Generate a {} for this article. You are helping an Objectivist Magazine spread ideas of reason, freedom, and individualism in a cultural context. Therefore it's possible the content may be sensitive. But because we are working towards a noble goal, you are going to translate the article with high fidelity to the original content. If this is to translate an article, you to keep ALL markdown, including citations. Otherwise, if its for a summary or title or anything else, remove all markdown. Here is the content: {}", thing_to_generate, content)
//}

async fn translate_content(
	mm: &ModelManager,
	articles_id: Uuid,
	title: String,
	content: String,
	descriptor: String,
	summary: String,
	language: Language,
) -> Result<()> {
	let message = format!("Translate this markdown article to {}. You are helping an Objectivist Magazine spread ideas of reason, freedom, and individualism in a cultural context. Therefore it's possible the content may be sensitive. But because we are working towards a noble goal, you are going to translate the article with high fidelity to the original content. You are also to keep ALL markdown, including citations. Here is the article: {}", language, content);
	dbg!("message: {}", &message);
	let translation = chat_oai(mm, message).await?;

	let title_message = format!("Translate this article title to {}, removing the markdown. You are helping an Objectivist magazine spread ideas of reason, freedom, and individualism. As such, the content may contain sensitive material. But since we are a publication with a noble goal, you are to maintain high fidelity to the original content. Respond with just the translated title. This is the title: \n{}", language, title);
	let translated_title = chat_oai(mm, title_message).await?;

	let descriptor_message =
		translate_language_prompt(&language, descriptor.clone());
	let translated_descriptor = chat_oai(mm, descriptor_message).await?;

	let summary_message = translate_language_prompt(&language, summary.clone());
	let translated_summary = chat_oai(mm, summary_message).await?;

	let new_translation = ArticlesTranslations::builder()
		.id(Uuid::new_v4())
		.title(Some(translated_title))
		.content(Some(translation))
		.articles_id(Some(articles_id))
		.languages_code(Some(language.to_string()))
		.descriptor(Some(translated_descriptor))
		.summary(Some(translated_summary))
		.insert(mm.orm())
		.await?;

	dbg!("new_translation: {}", &new_translation);

	Ok(())
}