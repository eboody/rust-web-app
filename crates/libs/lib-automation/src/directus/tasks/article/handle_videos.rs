#![allow(unused)]
use model::directus::{self, Articles, ArticlesFiles, VecString, WpPosts};
use regex::Regex;
use reqwest::{Url, multipart};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
struct ResponseDataWrapper<T> {
    data: T,
}

pub async fn handle_videos(mm: &ModelManager, article: &Articles) -> Result<()> {
    let (title, slug, body) = (
        article
            .title
            .as_ref()
            .ok_or(Error::NoTitleInArticle(article.id))?,
        article
            .slug
            .as_ref()
            .ok_or(Error::NoSlugInArticle(article.id))?,
        article
            .body
            .as_ref()
            .ok_or(Error::NoContentInArticle(article.id))?,
    );

    let post = WpPosts::select()
        .select("content")
        .where_("title = ?")
        .bind(title)
        .where_("slug = ?")
        .bind(slug)
        .fetch_one(mm.orm())
        .await?;

    let urls = parse_video_blocks(body);
    //let image_urls = parse_images(&post.content)?;

    //get image_urls that arent in captions
    //let image_urls_that_arent_in_captions = image_urls
    //  .into_iter()
    //  .flatten()
    //  .flatten()
    //  .filter(|url| {
    //    !captions
    //      .iter()
    //      .any(|caption| caption.img.as_deref() == Some(url.as_str()))
    //  })
    //  .collect::<Vec<_>>();

    for (index, url) in urls.into_iter().enumerate() {
        let article_image_file: Result<ArticlesFiles> =
            process_video_url(mm, article, &url, title, slug, index).await;

        if let Ok(article_image_file) = &article_image_file {
            let file_title = directus::Files::select()
                .select("title")
                .where_("id = ?")
                .bind(article_image_file.directus_files_id)
                .fetch_one(mm.orm())
                .await?
                .title;

            if let Some(file_title) = file_title {
                replace_video_block(mm, article, &file_title, &url).await?;
            }
        }

        if let Err(e) = article_image_file {
            error!("Failed to process image {}: {:?}", url, e);
        }
    }

    //for (index, caption) in captions.into_iter().enumerate() {
    //  let url: Url;
    //
    //  if let Some(href_url) = &caption.href {
    //    url = Url::parse(href_url)?;
    //  } else if let Some(img_url) = &caption.img {
    //    url = Url::parse(img_url)?;
    //  } else {
    //    continue;
    //  }
    //
    //  let article_image_file: Result<ArticlesFiles> =
    //    process_video_url(mm, article, &url, title, slug, index, Some(&caption)).await;
    //
    //  replace_caption(
    //    mm,
    //    article,
    //    caption.figure.as_deref(),
    //    caption.text.as_deref(),
    //    &url,
    //  )
    //  .await?;
    //
    //  debug!(
    //    "->> {:<12} - article_image_file: {:#?}",
    //    file!(),
    //    article_image_file
    //  );
    //
    //  if let Err(e) = article_image_file {
    //    error!("Failed to process image {}: {:?}", url, e);
    //  }
    //}

    Ok(())
}

//fn parse_images(content: &str) -> Result<Vec<Option<Result<Url>>>> {
//  let caption_regex = Regex::new(
//    r#"(?x)
//<img.*
//src="(?P<url>.*?)"
//.*>"#,
//  )
//  .unwrap();
//
//  Ok(
//    caption_regex
//      .captures_iter(content)
//      .map(|cap| {
//        let href = cap
//          .name("url")
//          .map(|m| Url::parse(m.as_str()).map_err(|e| e.into()));
//        href
//      })
//      .collect::<Vec<_>>(),
//  )
//}

fn parse_video_blocks(content: &str) -> Vec<Url> {
    let caption_regex = Regex::new(
        r#"(?x)
\\\[video.*?
mp4="(?P<url>.*)".*
\\\[/video\\\]
"#,
    )
    .unwrap();

    let urls = caption_regex
        .captures_iter(content)
        .map(|cap| {
            let url = cap.name("url").map(|m| m.as_str());
            Url::parse(url.unwrap()).map_err(|e| e.into())
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    urls
}

async fn process_video_url(
    mm: &ModelManager,
    article: &Articles,
    url: &Url,
    title: &str,
    slug: &str,
    index: usize,
) -> Result<ArticlesFiles> {
    let article_video_with_same_url = ArticlesFiles::select()
        .where_("url = ?")
        .bind(url.to_string())
        .where_("articles_id = ?")
        .bind(article.id)
        .fetch_one(mm.orm())
        .await;

    let articles_video_already_exists = article_video_with_same_url.is_ok();

    if articles_video_already_exists {
        return Ok(article_video_with_same_url?);
    }

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

    let new_file_name = format!("{} Video {}", title, index);

    let directus_upload_form = multipart::Form::new()
        .part("title", multipart::Part::text(new_file_name))
        .part(
            "folder",
            multipart::Part::text(config().ARTICLES_VIDEOS_FOLDER_ID.to_string()),
        )
        .part(
            "file",
            multipart::Part::stream(image_bytes)
                .file_name(format!("{}-{}.{}", slug, index, file_extension))
                .mime_str("video/mp4")?,
        );

    let video_file = mm
        .reqwest()
        .post("https://directus.eman.network/files")
        .headers(config().DIRECTUS_HEADERS.clone())
        .multipart(directus_upload_form)
        .send()
        .await
        .map_err(|_| Error::FailedToUploadImage(url.clone().to_string()))?
        .json::<ResponseDataWrapper<directus::api::Files>>()
        .await?
        .data;

    directus::Files::select()
        .where_("id = ?")
        .bind(video_file.id)
        .fetch_one(mm.orm())
        .await?
        .update_partial()
        .tags(article.tags.clone())
        .update(mm.orm())
        .await?;

    let res = ArticlesFiles::builder()
        .directus_files_id(video_file.id)
        .articles_id(article.id)
        .figure(Some(index.to_string()))
        .url(Some(url.to_string()))
        .insert(mm.orm())
        .await;

    let article_files_item_failed_to_insert = res.is_err();

    if article_files_item_failed_to_insert {
        error!("Failed to insert image file: {:?}", res);
        mm.reqwest()
            .delete(format!(
                "https://directus.eman.network/files/{}",
                video_file.id
            ))
            .headers(config().DIRECTUS_HEADERS.clone())
            .send()
            .await?;
    }

    Ok(res?)
}

async fn replace_video_block(
    mm: &ModelManager,
    article: &Articles,
    title: &str,
    url: &Url,
) -> Result<()> {
    let article = Articles::select()
        .where_("id = ?")
        .bind(article.id)
        .fetch_one(mm.orm())
        .await?;
    let regex_str = format!(
        r#"(?xm)(\\\[video.*?{}.*?\\\[/video\\\])"#,
        regex::escape(url.as_str()),
    );
    let caption_regex = Regex::new(&regex_str).unwrap();

    if let Some(article_body) = &article.body {
        let new_caption = format!("![{}]({})", title, url);
        let article_body = caption_regex.replace(article_body, new_caption);
        article
            .update_partial()
            .body(Some(article_body.to_string()))
            .update(mm.orm())
            .await?;
    }

    Ok(())
}
