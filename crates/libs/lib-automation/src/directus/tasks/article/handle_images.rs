use model::directus::{self, Articles, ArticlesFiles, WpPosts};
use regex::Regex;
use reqwest::{Url, multipart};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
struct ResponseDataWrapper<T> {
  data: T,
}

#[derive(Debug)]
struct Caption {
  href: Option<String>,
  img: Option<String>,
  figure: Option<String>,
  text: Option<String>,
}

struct ImageProcessor<'a> {
  mm: &'a ModelManager,
  article: &'a Articles,
  title: String,
  slug: String,
}

impl<'a> ImageProcessor<'a> {
  pub fn new(mm: &'a ModelManager, article: &'a Articles) -> Result<Self> {
    let title = article
      .title
      .as_ref()
      .ok_or(Error::NoTitleInArticle(article.id))?
      .clone();
    let slug = article
      .slug
      .as_ref()
      .ok_or(Error::NoSlugInArticle(article.id))?
      .clone();

    Ok(Self {
      mm,
      article,
      title,
      slug,
    })
  }

  async fn transform_content(&self, content: &str, file_id: Uuid) -> Result<String> {
    let regex = Regex::new(
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
    )?;

    let transformed = regex.replace_all(content, |caps: &regex::Captures| {
      format!(
        "![{}](/assets/{})",
        caps.name("caption").map_or("", |m| m.as_str().trim()),
        file_id
      )
    });

    Ok(transformed.into_owned())
  }

  pub async fn process(&self) -> Result<()> {
    let post = WpPosts::select()
      .select("content")
      .where_("slug = ?")
      .bind(&self.slug)
      .fetch_one(self.mm.orm())
      .await?;

    let content = &post.content;
    let captions = parse_captions(content)?;
    let uncaptioned_images = self.get_uncaptioned_images(content, &captions)?;

    for (index, url) in uncaptioned_images.iter().enumerate() {
      let index = index + 1;
      if let Err(e) = self.process_image(url, index, None).await {
        error!("Failed to process image {}: {:?}", url, e);
      }
    }

    for (index, caption) in captions.iter().enumerate() {
      let index = index + 1;
      tracing::debug!("->> {:<12} - caption:\n{:#?}", file!(), caption);

      let url = if let Some(href_url) = &caption.href {
        parse_url(href_url)?
      } else if let Some(img_url) = &caption.img {
        parse_url(img_url)?
      } else {
        continue;
      };

      let result = self.process_image(&url, index, Some(caption)).await;

      if let Ok(article_file) = result {
        // Transform the content with the new file ID
        let transformed_content = self
          .transform_content(&post.content, article_file.directus_files_id)
          .await?;

        // Update the article with transformed content
        Articles::select()
          .where_("id = ?")
          .bind(self.article.id)
          .fetch_one(self.mm.orm())
          .await?
          .update_partial()
          .body(Some(transformed_content))
          .update(self.mm.orm())
          .await?;
      } else if let Err(e) = result {
        error!("Failed to process image {}: {:?}", url, e);
      }
    }

    Ok(())
  }

  async fn process_image(
    &self,
    url: &Url,
    index: usize,
    caption: Option<&Caption>,
  ) -> Result<ArticlesFiles> {
    // Check if image already exists
    if let Ok(existing) = ArticlesFiles::select()
      .where_("url = ?")
      .bind(url.to_string())
      .fetch_one(self.mm.orm())
      .await
    {
      return Ok(existing);
    }

    let figure = caption
      .and_then(|c| c.figure.clone())
      .unwrap_or_else(|| index.to_string());

    let file_name = get_filename_from_url(url)?;

    // Check for existing file in Directus
    if let Ok(existing_file) = directus::Files::select()
      .where_("filename_download = ?")
      .bind(&file_name)
      .fetch_one(self.mm.orm())
      .await
    {
      return self
        .create_article_file(existing_file.id, &figure, caption, url)
        .await;
    }

    // Upload new file
    let image_bytes = reqwest::get(url.clone()).await?.bytes().await?;
    let new_file_name = format!("{} {}", self.title, figure);

    let form = multipart::Form::new()
      .part("title", multipart::Part::text(new_file_name))
      .part(
        "folder",
        multipart::Part::text(config().ARTICLES_IMAGES_FOLDER_ID.to_string()),
      )
      .part(
        "file",
        multipart::Part::stream(image_bytes)
          .file_name(file_name)
          .mime_str("image/jpeg")?,
      );

    let image_file = self
      .mm
      .reqwest()
      .post("https://directus.eman.network/files")
      .headers(config().DIRECTUS_HEADERS.clone())
      .multipart(form)
      .send()
      .await?
      .json::<ResponseDataWrapper<directus::api::Files>>()
      .await?
      .data;

    // Update file metadata
    directus::Files::select()
      .where_("id = ?")
      .bind(image_file.id)
      .fetch_one(self.mm.orm())
      .await?
      .update_partial()
      .description(caption.and_then(|c| c.text.clone()))
      .tags(self.article.tags.clone().map(|t| t.to_string()))
      .update(self.mm.orm())
      .await?;

    // Create ArticlesFiles entry
    let res = self
      .create_article_file(image_file.id, &figure, caption, url)
      .await;

    if res.is_err() {
      self
        .mm
        .reqwest()
        .delete(format!(
          "https://directus.eman.network/files/{}",
          image_file.id
        ))
        .headers(config().DIRECTUS_HEADERS.clone())
        .send()
        .await?;
    }

    res
  }

  async fn create_article_file(
    &self,
    file_id: Uuid,
    figure: &str,
    caption: Option<&Caption>,
    url: &Url,
  ) -> Result<ArticlesFiles> {
    Ok(
      ArticlesFiles::builder()
        .directus_files_id(file_id)
        .articles_id(self.article.id)
        .caption(caption.and_then(|c| c.text.clone()))
        .figure(Some(figure.to_string()))
        .url(Some(url.to_string()))
        .insert(self.mm.orm())
        .await?,
    )
  }

  fn get_uncaptioned_images(
    &self,
    content: &str,
    captions: &[Caption],
  ) -> Result<Vec<Url>> {
    let regex = Regex::new(r#"(?x)<img.*src="(?P<url>.*?)".*>"#)?;

    Ok(
      regex
        .captures_iter(content)
        .filter_map(|cap| cap.name("url"))
        .map(|m| parse_url(m.as_str()))
        .filter_map(Result::ok)
        .filter(|url| {
          !captions
            .iter()
            .any(|caption| caption.img.as_deref() == Some(url.as_str()))
        })
        .collect(),
    )
  }

  async fn replace_caption(&self, caption: &Caption, url: &Url) -> Result<()> {
    tracing::debug!("->> {:<12} - article:\n{:#?}", file!(), self.article);
    tracing::debug!("->> {:<12} - figure:\n{:#?}", file!(), caption.figure);
    tracing::debug!("->> {:<12} - caption_text:\n{:#?}", file!(), caption.text);

    let article = Articles::select()
      .where_("id = ?")
      .bind(self.article.id)
      .fetch_one(self.mm.orm())
      .await?;

    if let (Some(body), Some(figure), Some(caption_text)) = (
      &article.body,
      caption.figure.as_deref(),
      caption.text.as_deref(),
    ) {
      let regex_str = format!(
        r#"(?xm)(\\\[caption.*?{}.*?\\\[/caption\\\])"#,
        regex::escape(url.as_str()),
      );
      let caption_regex = Regex::new(&regex_str)?;
      let new_caption = format!("![Fig. {}: {}]({})", figure, caption_text, url);
      let updated_body = caption_regex.replace(body, new_caption);

      article
        .update_partial()
        .body(Some(updated_body.to_string()))
        .update(self.mm.orm())
        .await?;
    }

    Ok(())
  }
}

fn parse_captions(content: &str) -> Result<Vec<Caption>> {
  let regex = Regex::new(
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
  )?;

  Ok(
    regex
      .captures_iter(content)
      .map(|cap| Caption {
        href: cap.name("href").map(|m| m.as_str().to_string()),
        img: cap.name("img").map(|m| m.as_str().to_string()),
        figure: cap.name("figure").map(|m| m.as_str().to_string()),
        text: cap.name("caption").map(|m| m.as_str().to_string()),
      })
      .collect(),
  )
}

fn parse_url(raw_url: &str) -> Result<Url> {
  let base = Url::parse("https://theobjectivestandard.com")?;
  Ok(Url::parse(raw_url).or_else(|_| base.join(raw_url))?)
}

fn get_filename_from_url(url: &Url) -> Result<String> {
  let path_segments = url
    .path_segments()
    .ok_or(Error::NoPathSegments(url.to_string()))?;

  path_segments
    .last()
    .map(String::from)
    .ok_or_else(|| Error::NoLastPathSegment(url.to_string()))
}

pub async fn handle_images(mm: &ModelManager, article: &Articles) -> Result<()> {
  let processor = ImageProcessor::new(mm, article)?;
  processor.process().await
}
