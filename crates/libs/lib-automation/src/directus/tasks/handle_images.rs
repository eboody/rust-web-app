use model::directus::{self, Articles, ArticlesFiles, Posts};
use regex::Regex;
use reqwest::{Url, multipart};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
struct ResponseDataWrapper<T> {
	data: T,
}

pub async fn handle_images(mm: &ModelManager, article: &Articles) -> Result<()> {
	let (title, slug) = (
		article
			.title
			.as_ref()
			.ok_or(Error::NoTitleInArticle(article.id))?,
		article
			.slug
			.as_ref()
			.ok_or(Error::NoSlugInArticle(article.id))?,
	);

	let post = Posts::select()
		.select("content")
		.where_("title = ?")
		.bind(title)
		.where_("slug = ?")
		.bind(slug)
		.fetch_one(mm.orm())
		.await?;

	let captions = parse_captions(&post.content)?;
	let image_urls = parse_images(&post.content)?;

	//get image_urls that arent in captions
	let image_urls_that_arent_in_captions = image_urls
		.into_iter()
		.flatten()
		.flatten()
		.filter(|url| {
			!captions
				.iter()
				.any(|caption| caption.img.as_deref() == Some(url.as_str()))
		})
		.collect::<Vec<_>>();

	for (index, url) in image_urls_that_arent_in_captions.into_iter().enumerate() {
		let article_image_file: Result<ArticlesFiles> =
			process_image_url(mm, article, &url, title, slug, index, None).await;

		if let Err(e) = article_image_file {
			error!("Failed to process image {}: {:?}", url, e);
		}
	}

	for (index, caption) in captions.into_iter().enumerate() {
		let url: Url;

		if let Some(href_url) = &caption.href {
			url = Url::parse(href_url)?;
		} else if let Some(img_url) = &caption.img {
			url = Url::parse(img_url)?;
		} else {
			continue;
		}

		let article_image_file: Result<ArticlesFiles> =
			process_image_url(mm, article, &url, title, slug, index, Some(caption))
				.await;

		debug!(
			"->> {:<12} - article_image_file: {:#?}",
			file!(),
			article_image_file
		);

		if let Err(e) = article_image_file {
			error!("Failed to process image {}: {:?}", url, e);
		}
	}

	Ok(())
}

fn parse_images(content: &str) -> Result<Vec<Option<Result<Url>>>> {
	let caption_regex = Regex::new(
		r#"(?x)
<img.*
src="(?P<url>.*?)"
.*>"#,
	)
	.unwrap();

	Ok(caption_regex
		.captures_iter(content)
		.map(|cap| {
			let href = cap
				.name("url")
				.map(|m| Url::parse(m.as_str()).map_err(|e| e.into()));
			href
		})
		.collect::<Vec<_>>())
}

fn parse_captions(content: &str) -> Result<Vec<Caption>> {
	let caption_regex = Regex::new(
		r#"(?x)
\[caption.*?
(?:href="(?P<href>.*?)")?.*?
(?:wp-image-(.*?)[\s|"]).*?
(?:src="(?P<img>.*?)").*?
(?:>\s)+
  (?:[F|f]ig[\s|\.]+)?
  (?:(?P<figure>\d+)[\:|\s|\.]?\s+?)?
  (?P<caption>.*?)
\[\/caption\]"#,
	)
	.unwrap();

	Ok(caption_regex
		.captures_iter(content)
		.map(|cap| {
			let href = cap.name("href").map(|m| m.as_str());
			let img = cap.name("img").map(|m| m.as_str());
			let figure = cap.name("figure").map(|m| m.as_str());
			let caption = cap.name("caption").map(|m| m.as_str());
			Caption {
				href: href.map(|s| s.to_string()),
				img: img.map(|s| s.to_string()),
				figure: figure.map(|s| s.to_string()),
				caption: caption.map(|s| s.to_string()),
			}
		})
		.collect::<Vec<_>>())
}

#[derive(Debug)]
struct Caption {
	href: Option<String>,
	img: Option<String>,
	figure: Option<String>,
	caption: Option<String>,
}

async fn process_image_url(
	mm: &ModelManager,
	article: &Articles,
	url: &Url,
	title: &str,
	slug: &str,
	index: usize,
	caption: Option<Caption>,
) -> Result<ArticlesFiles> {
	let article_image_with_same_info = ArticlesFiles::select()
		.where_("url = ?")
		.bind(url.to_string())
		.where_("articles_id = ?")
		.bind(article.id)
		.fetch_one(mm.orm())
		.await;

	let articles_image_already_existed = article_image_with_same_info.is_ok();

	if articles_image_already_existed {
		return Ok(article_image_with_same_info?);
	}

	let iteration_of_article_image = caption
		.as_ref()
		.and_then(|c| c.figure.clone())
		.unwrap_or(index.to_string());

	let path_segments = url
		.path_segments()
		.ok_or(Error::NoPathSegments(url.to_string()))?;
	let file_name = path_segments
		.last()
		.ok_or(Error::NoLastPathSegment(url.to_string()))?;
	let file_extension = file_name
		.split('.')
		.last()
		.ok_or(Error::NoFileExtension(file_name.to_string()))?;

	let image_bytes = reqwest::get(url.clone()).await?.bytes().await?;

	let new_file_name = format!("{} {}", title, iteration_of_article_image);

	let directus_upload_form = multipart::Form::new()
		.part("title", multipart::Part::text(new_file_name))
		.part(
			"folder",
			multipart::Part::text(config().ARTICLES_IMAGES_FOLDER_ID.to_string()),
		)
		.part(
			"file",
			multipart::Part::stream(image_bytes)
				.file_name(format!(
					"{}-{}.{}",
					slug, iteration_of_article_image, file_extension
				))
				.mime_str("image/jpeg")?,
		);

	let image_file = mm
		.reqwest()
		.post("https://directus.eman.network/files")
		.headers(config().DIRECTUS_HEADERS.clone())
		.multipart(directus_upload_form)
		.send()
		.await
		.map_err(|_| Error::FailedToUploadImage(url.clone()))?
		.json::<ResponseDataWrapper<directus::api::Files>>()
		.await?
		.data;

	directus::Files::select()
		.where_("id = ?")
		.bind(image_file.id)
		.fetch_one(mm.orm())
		.await?
		.update_partial()
		.description(caption.as_ref().and_then(|c| c.caption.clone()))
		.tags(article.tags.clone())
		.update(mm.orm())
		.await?;

	let res = ArticlesFiles::builder()
		.directus_files_id(image_file.id)
		.articles_id(article.id)
		.caption(caption.and_then(|c| c.caption))
		.figure(Some(iteration_of_article_image))
		.url(Some(url.to_string()))
		.insert(mm.orm())
		.await;

	let article_files_item_failed_to_insert = res.is_err();

	if article_files_item_failed_to_insert {
		error!("Failed to insert image file: {:?}", res);
		mm.reqwest()
			.delete(format!(
				"https://directus.eman.network/files/{}",
				image_file.id
			))
			.headers(config().DIRECTUS_HEADERS.clone())
			.send()
			.await?;
	}

	Ok(res?)
}
