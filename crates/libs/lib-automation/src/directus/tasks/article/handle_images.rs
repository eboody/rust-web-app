use axum::body::Bytes;
use model::directus::{self, Articles, ArticlesFiles, WpPosts};
use regex::Regex;
use reqwest::{Url, multipart};
use serde::Deserialize;
use std::sync::LazyLock;
use tracing::warn;

use crate::prelude::*;

use tracing::{debug, error};

struct ImageProcessor<'a> {
  mm: &'a ModelManager,
  article: &'a Articles,
  url: Url,
  title: String,
  index: usize,
  caption: Option<Caption>,
}

impl ImageProcessor<'_> {
  async fn process(&self) -> Result<ArticlesFiles> {
    let url = self.normalize_url()?;
    debug!("Processing image at url: {}", url);

    self.check_for_existing_article_image(&url).await?;

    if let Ok(existing_file) = self.find_existing_file().await {
      return self.create_article_file(existing_file.id).await;
    }

    self.upload_and_create_new_file().await
  }

  fn normalize_url(&self) -> Result<Url> {
    if self.url.has_host() {
      Ok(self.url.clone())
    } else {
      parse_url(self.url.as_str())
    }
  }

  async fn check_for_existing_article_image(&self, url: &Url) -> Result<()> {
    let exists = ArticlesFiles::select()
      .where_("url = ?")
      .bind(url.to_string())
      .fetch_one(self.mm.orm())
      .await
      .is_ok();

    if exists {
      return Err(Error::ArticleImageAlreadyExisted(url.to_string()));
    }
    Ok(())
  }

  async fn find_existing_file(&self) -> Result<directus::Files> {
    let filename = self.get_filename()?;
    Ok(
      directus::Files::select()
        .where_("filename_download = ?")
        .bind(filename)
        .fetch_one(self.mm.orm())
        .await?,
    )
  }

  fn get_filename(&self) -> Result<&str> {
    self
      .url
      .path_segments()
      .ok_or_else(|| Error::NoPathSegments(self.url.to_string()))?
      .last()
      .ok_or_else(|| Error::NoLastPathSegment(self.url.to_string()))
  }

  async fn create_article_file(&self, file_id: Uuid) -> Result<ArticlesFiles> {
    Ok(
      ArticlesFiles::builder()
        .directus_files_id(file_id)
        .articles_id(self.article.id)
        .caption(self.caption.as_ref().and_then(|c| c.text.clone()))
        .figure(Some(self.index.to_string()))
        .url(Some(self.url.to_string()))
        .insert(self.mm.orm())
        .await?,
    )
  }

  async fn upload_and_create_new_file(&self) -> Result<ArticlesFiles> {
    let image_file = self.upload_file().await?;

    self.update_file_metadata(&image_file).await?;

    match self.create_article_file(image_file.id).await {
      Ok(article_file) => Ok(article_file),
      Err(e) => {
        error!("Failed to insert image file: {:?}", e);
        self.cleanup_failed_upload(&image_file).await?;
        Err(e)
      }
    }
  }

  async fn upload_file(&self) -> Result<directus::api::Files> {
    let image_bytes = reqwest::get(self.url.clone()).await?.bytes().await?;
    let form = self.create_upload_form(image_bytes)?;

    self
      .mm
      .reqwest()
      .post("https://directus.eman.network/files")
      .headers(config().DIRECTUS_HEADERS.clone())
      .multipart(form)
      .send()
      .await
      .map_err(|_| Error::FailedToUploadImage(self.url.to_string()))?
      .json::<ResponseDataWrapper<directus::api::Files>>()
      .await
      .map(|wrapper| wrapper.data)
      .map_err(|e| e.into())
  }

  fn create_upload_form(&self, image_bytes: Bytes) -> Result<multipart::Form> {
    let filename = self.get_filename()?;
    let new_filename = format!("{} {}", self.title, self.index);

    Ok(
      multipart::Form::new()
        .part("title", multipart::Part::text(new_filename))
        .part(
          "folder",
          multipart::Part::text(config().ARTICLES_IMAGES_FOLDER_ID.to_string()),
        )
        .part(
          "file",
          multipart::Part::stream(image_bytes)
            .file_name(filename.to_owned())
            .mime_str("image/jpeg")?,
        ),
    )
  }

  async fn update_file_metadata(
    &self,
    image_file: &directus::api::Files,
  ) -> Result<()> {
    directus::Files::select()
      .where_("id = ?")
      .bind(image_file.id)
      .fetch_one(self.mm.orm())
      .await?
      .update_partial()
      .description(self.caption.as_ref().and_then(|c| c.text.clone()))
      .tags(self.article.tags.clone().map(|t| t.to_string()))
      .update(self.mm.orm())
      .await?;

    Ok(())
  }

  async fn cleanup_failed_upload(
    &self,
    image_file: &directus::api::Files,
  ) -> Result<()> {
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

    Ok(())
  }
}

async fn process_image_url(
  mm: &ModelManager,
  article: &Articles,
  url: &Url,
  title: &str,
  _slug: &str,
  index: usize,
  caption: Option<&Caption>,
) -> Result<ArticlesFiles> {
  let processor = ImageProcessor {
    mm,
    article,
    url: url.clone(),
    title: title.to_owned(),
    index,
    caption: caption.cloned(),
  };

  processor.process().await
}

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

  let mut post = WpPosts::select()
    .select("content")
    .where_("slug = ?")
    .bind(slug)
    .fetch_one(mm.orm())
    .await
    .expect("Failed to fetch wppost");

  let captions = parse_captions(&post.content)?;
  post.content = replace_img_blocks(post.content);
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
  let mut index = 1;

  for caption in captions.into_iter() {
    let url: Url;

    if let Some(href_url) = &caption.href {
      url = parse_url(href_url)?;
    } else if let Some(img_url) = &caption.img {
      url = parse_url(img_url)?;
    } else {
      tracing::debug!("->> {:<12} - caption:\n{:#?}", module_path!(), caption);
      continue;
    }

    let article_image_file: Result<ArticlesFiles> =
      process_image_url(mm, article, &url, title, slug, index, Some(&caption)).await;

    replace_caption(
      mm,
      article,
      Some(caption.figure.clone().unwrap_or(index.to_string()).as_str()),
      caption.text.as_deref(),
      &url,
    )
    .await?;

    if let Err(e) = article_image_file {
      if e.to_string().contains("ArticleImageAlreadyExisted") {
        warn!("Article image already existed: {:?}", e);
        continue;
      }
      error!("Failed to process image {}: {:?}", url, e);
      continue;
    }
    index += 1;
  }

  for url in image_urls_that_arent_in_captions.into_iter() {
    let article_image_file: Result<ArticlesFiles> =
      process_image_url(mm, article, &url, title, slug, index, None).await;

    if let Err(e) = article_image_file {
      if e.to_string().contains("ArticleImageAlreadyExisted") {
        warn!("Article image already existed: {:?}", e);
        continue;
      }
      error!("Failed to process image {}: {:?}", url, e);
      continue;
    }
    index += 1;
  }

  Ok(())
}

fn parse_url(raw_url: &str) -> Result<Url> {
  let base = Url::parse("https://theobjectivestandard.com")?;
  let raw_url = urlencoding::decode(raw_url)?.to_string();
  Ok(Url::parse(&raw_url).or_else(|_| base.join(&raw_url))?)
}

fn parse_images(content: &str) -> Result<Vec<Option<Result<Url>>>> {
  let caption_regex = Regex::new(r#"(?x)<img.*src="(?P<url>.*?)".*>"#).unwrap();
  Ok(
    caption_regex
      .captures_iter(content)
      .map(|cap| cap.name("url").map(|m| parse_url(m.as_str())))
      .collect(),
  )
}

static IMG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r#"(?x)<img.*src=".*?".*>"#).expect("Invalid regex pattern")
});

fn replace_img_blocks(content: String) -> String {
  IMG_REGEX.replace_all(&content, "").to_string()
}

fn parse_captions(content: &str) -> Result<Vec<Caption>> {
  let caption_regex = Regex::new(
    r#"(?x)
\[caption.*?\]
(?:<a\s+href="(?P<href>https://(?:www\.)?theobjectivestandard.com.*?)".*?>)?
.*?wp-image-(?P<id>\d+).*?
src="(?P<img>.*?)".*?
(?:>\s*)+
(?:[F|f]ig[\s|\.]+)?
(?:(?P<figure>\d+)[\:|\s|\.]?\s*)?
(?P<caption>.*?)
\[/caption\]
"#,
  )
  .unwrap();

  let captions = caption_regex
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
        text: caption.map(|s| s.to_string()),
      }
    })
    .collect::<Vec<_>>();

  Ok(captions)
}

#[derive(Debug, Clone)]
struct Caption {
  href: Option<String>,
  img: Option<String>,
  figure: Option<String>,
  text: Option<String>,
}

//async fn process_image_url(
//  mm: &ModelManager,
//  article: &Articles,
//  url: &Url,
//  title: &str,
//  _slug: &str,
//  index: usize,
//  caption: Option<&Caption>,
//) -> Result<ArticlesFiles> {
//  debug!("index: {}", index);
//  let url = if url.has_host() {
//    url.clone()
//  } else {
//    parse_url(url.as_str())?
//  };
//  debug!("url: {}", url);
//  let article_image_with_same_info = ArticlesFiles::select()
//    .where_("url = ?")
//    .bind(url.to_string())
//    .fetch_one(mm.orm())
//    .await;
//
//  let articles_image_already_existed = article_image_with_same_info.is_ok();
//
//  if articles_image_already_existed {
//    return Err(Error::ArticleImageAlreadyExisted(url.to_string()));
//  }
//
//  let iteration_of_article_image = index.to_string();
//
//  let path_segments = url
//    .path_segments()
//    .ok_or(Error::NoPathSegments(url.to_string()))
//    .expect("Failed to get path segments");
//
//  let file_name = path_segments
//    .last()
//    .ok_or(Error::NoLastPathSegment(url.to_string()))
//    .expect("Failed to get last path segment");
//
//  if let Ok(existing_file) = directus::Files::select()
//    .where_("filename_download = ?")
//    .bind(file_name)
//    .fetch_one(mm.orm())
//    .await
//  {
//    Ok(
//      ArticlesFiles::builder()
//        .directus_files_id(existing_file.id)
//        .articles_id(article.id)
//        .caption(caption.and_then(|c| c.text.clone()))
//        .figure(Some(iteration_of_article_image))
//        .url(Some(url.to_string()))
//        .insert(mm.orm())
//        .await
//        .expect("Failed to insert article file"),
//    )
//  } else {
//    //let file_extension = file_name
//    //  .split('.')
//    //  .last()
//    //  .ok_or(Error::NoFileExtension(file_name.to_string()))?;
//
//    let image_bytes = reqwest::get(url.clone()).await?.bytes().await?;
//
//    let new_file_name = format!("{} {}", title, iteration_of_article_image);
//    debug!("new_file_name: {}", new_file_name);
//
//    let file_name_clone = file_name.to_owned();
//
//    let directus_upload_form = multipart::Form::new()
//      .part("title", multipart::Part::text(new_file_name))
//      .part(
//        "folder",
//        multipart::Part::text(config().ARTICLES_IMAGES_FOLDER_ID.to_string()),
//      )
//      .part(
//        "file",
//        multipart::Part::stream(image_bytes)
//          .file_name(file_name_clone)
//          .mime_str("image/jpeg")?,
//      );
//
//    let image_file = mm
//      .reqwest()
//      .post("https://directus.eman.network/files")
//      .headers(config().DIRECTUS_HEADERS.clone())
//      .multipart(directus_upload_form)
//      .send()
//      .await
//      .map_err(|_| Error::FailedToUploadImage(url.clone().to_string()))?
//      .json::<ResponseDataWrapper<directus::api::Files>>()
//      .await
//      .expect("Failed to parse response")
//      .data;
//
//    directus::Files::select()
//      .where_("id = ?")
//      .bind(image_file.id)
//      .fetch_one(mm.orm())
//      .await?
//      .update_partial()
//      .description(caption.as_ref().and_then(|c| c.text.clone()))
//      .tags(article.tags.clone().map(|t| t.to_string()))
//      .update(mm.orm())
//      .await
//      .expect("Failed to update file");
//
//    let res = ArticlesFiles::builder()
//      .directus_files_id(image_file.id)
//      .articles_id(article.id)
//      .caption(caption.and_then(|c| c.text.clone()))
//      .figure(Some(iteration_of_article_image))
//      .url(Some(url.to_string()))
//      .insert(mm.orm())
//      .await;
//
//    let article_files_item_failed_to_insert = res.is_err();
//
//    if article_files_item_failed_to_insert {
//      error!("Failed to insert image file: {:?}", res);
//      mm.reqwest()
//        .delete(format!(
//          "https://directus.eman.network/files/{}",
//          image_file.id
//        ))
//        .headers(config().DIRECTUS_HEADERS.clone())
//        .send()
//        .await?;
//    }
//    Ok(res?)
//  }
//}

async fn replace_caption(
  mm: &ModelManager,
  article: &Articles,
  figure: Option<&str>,
  caption_text: Option<&str>,
  url: &Url,
) -> Result<()> {
  //tracing::debug!("->> {:<12} - article:\n{:#?}", file!(), article);
  //tracing::debug!("->> {:<12} - figure:\n{:#?}", file!(), figure);
  //tracing::debug!("->> {:<12} - caption_text:\n{:#?}", file!(), caption_text);

  let article = Articles::select()
    .where_("id = ?")
    .bind(article.id)
    .fetch_one(mm.orm())
    .await?;

  let regex_str = format!(
    r#"(?xms)(\\\[caption.*?{}.*?\\\[/caption\\\])"#,
    regex::escape(url.as_str()),
  );

  let caption_regex = Regex::new(&regex_str).unwrap();

  if let Some(article_body) = &article.body
    && let Some(figure) = figure
    && let Some(caption_text) = caption_text
  {
    let new_caption = format!(
      "![Fig. {}: {}]({})",
      figure,
      caption_text.replace("<em>", "").replace("</em>", ""),
      url
    );
    let article_body = caption_regex.replace(article_body, new_caption);
    article
      .update_partial()
      .body(Some(article_body.to_string()))
      .update(mm.orm())
      .await?;
  }

  Ok(())
}
