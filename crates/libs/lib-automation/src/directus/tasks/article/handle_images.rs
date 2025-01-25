use axum::body::Bytes;
use lib_utils::retry::RetryableRequest;
use model::directus::{self, Articles, ArticlesFiles, WpPosts};
use regex::Regex;
use reqwest::{Url, multipart};
use serde::Deserialize;
use std::sync::LazyLock;
use tracing::warn;

use crate::prelude::*;

use tracing::error;

struct ImageProcessor<'a> {
    mm: &'a ModelManager,
    article: &'a Articles,
    url: Url,
    title: String,
    index: usize,
    caption: Option<Caption>,
}

impl ImageProcessor<'_> {
    async fn process(&mut self) -> Result<ArticlesFiles> {
        let url = self.normalize_url()?;

        let articles_files = self.check_for_existing_article_image(&url).await;

        if let Ok(articles_files) = articles_files {
            return Ok(articles_files);
        }

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

    async fn check_for_existing_article_image(&self, url: &Url) -> Result<ArticlesFiles> {
        Ok(ArticlesFiles::select()
            .where_("url = ?")
            .bind(url.to_string())
            .fetch_one(self.mm.orm())
            .await?)
    }

    async fn find_existing_file(&self) -> Result<directus::Files> {
        let filename = self.get_filename()?;
        Ok(directus::Files::select()
            .where_("filename_download = ?")
            .bind(filename)
            .fetch_one(self.mm.orm())
            .await?)
    }

    fn get_filename(&self) -> Result<&str> {
        self.url
            .path_segments()
            .ok_or_else(|| Error::NoPathSegments(self.url.to_string()))?
            .last()
            .ok_or_else(|| Error::NoLastPathSegment(self.url.to_string()))
    }

    async fn create_article_file(&self, file_id: Uuid) -> Result<ArticlesFiles> {
        Ok(ArticlesFiles::builder()
            .directus_files_id(file_id)
            .articles_id(self.article.id)
            .caption(self.caption.as_ref().and_then(|c| c.text.clone()))
            .figure(Some(self.index.to_string()))
            .url(Some(self.url.to_string()))
            .insert(self.mm.orm())
            .await?)
    }

    async fn upload_and_create_new_file(&mut self) -> Result<ArticlesFiles> {
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

    async fn upload_file(&mut self) -> Result<directus::api::Files> {
        let url = get_commons_url(self.url.clone())
            .await?
            .expect("Failed to get commons url");

        let url = Url::parse(urlencoding::decode(url.as_str())?.to_string().as_str())?;

        println!("downloading image from url: {}", url);
        let response = reqwest::Client::new()
            .get(url.clone())
            .timeout(std::time::Duration::from_secs(10))
            .header("User-Agent", "Mozilla/5.0 (compatible; MyBot/1.0)")
            .send()
            .await?;

        let image_bytes = response.bytes().await?;
        let form = self.create_upload_form(image_bytes)?;

        self.mm
            .reqwest()
            .post("https://directus.eman.network/files")
            .headers(config().DIRECTUS_HEADERS.clone())
            .multipart(form)
            .send()
            .await
            .map_err(|_| Error::FailedToUploadImage(url.to_string()))?
            .json::<ResponseDataWrapper<directus::api::Files>>()
            .await
            .map(|wrapper| wrapper.data)
            .map_err(|e| e.into())
    }

    fn create_upload_form(&self, image_bytes: Bytes) -> Result<multipart::Form> {
        let filename = self.get_filename()?;
        let new_filename = format!("{} {}", self.title, self.index);

        Ok(multipart::Form::new()
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
            ))
    }

    async fn update_file_metadata(&self, image_file: &directus::api::Files) -> Result<()> {
        directus::Files::select()
            .where_("id = ?")
            .bind(image_file.id)
            .fetch_one(self.mm.orm())
            .await?
            .update_partial()
            .description(self.caption.as_ref().and_then(|c| c.text.clone()))
            .update(self.mm.orm())
            .await?;

        Ok(())
    }

    async fn cleanup_failed_upload(&self, image_file: &directus::api::Files) -> Result<()> {
        self.mm
            .reqwest()
            .delete(format!(
                "https://directus.eman.network/files/{}",
                image_file.id
            ))
            .headers(config().DIRECTUS_HEADERS.clone())
            .retry()
            .send::<()>()
            .await?;

        Ok(())
    }
}

#[bon::builder]
pub async fn process_image_url(
    mm: &ModelManager,
    article: &Articles,
    url: &Url,
    title: &str,
    index: usize,
    caption: Option<&Caption>,
) -> Result<ArticlesFiles> {
    let mut processor = ImageProcessor {
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

    println!("looping through captions");
    for caption in captions.into_iter() {
        let url: Url;

        if let Some(href_url) = &caption.href {
            url = parse_url(href_url)?;
        } else if let Some(img_url) = &caption.img {
            url = parse_url(img_url)?;
        } else {
            //tracing::debug!("->> {:<12} - caption:\n{:#?}", module_path!(), caption);
            continue;
        }

        let article_image_file: Result<ArticlesFiles> = process_image_url()
            .mm(mm)
            .article(article)
            .url(&url)
            .title(title)
            .index(index)
            .caption(&caption)
            .call()
            .await;

        let Ok(article_image_file) = article_image_file else {
            let e = article_image_file.unwrap_err();

            if e.to_string().contains("ArticleImageAlreadyExisted") {
                warn!("Article image already existed: {:?}", e);
            } else {
                error!("Failed to process image {}: {:?}", url, e);
            }

            continue;
        };

        replace_caption(
            mm,
            article,
            Some(caption.figure.clone().unwrap_or(index.to_string()).as_str()),
            caption.text.as_deref(),
            &url,
            article_image_file.directus_files_id,
        )
        .await?;

        index += 1;
    }

    println!("looping through image_urls_that_arent_in_captions");
    for url in image_urls_that_arent_in_captions.into_iter() {
        println!("processing image at url: {}", url);
        let article_image_file: Result<ArticlesFiles> = process_image_url()
            .mm(mm)
            .article(article)
            .url(&url)
            .title(title)
            .index(index)
            .call()
            .await;

        if let Err(e) = article_image_file {
            if e.to_string().contains("ArticleImageAlreadyExisted") {
                warn!("Article image already existed: {:?}", e);
                continue;
            }
            error!("Failed to process image {}: {:?}", url, e);
            continue;
        }

        let article_image_file = article_image_file?;

        println!("replacing img tag");
        replace_img_tag(mm, article, &url, article_image_file.directus_files_id).await?;

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
    Ok(caption_regex
        .captures_iter(content)
        .map(|cap| cap.name("url").map(|m| parse_url(m.as_str())))
        .collect())
}

static IMG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?x)\[caption.*?](<img.*src=".*?".*>).*?\[/caption]"#)
        .expect("Invalid regex pattern")
});

fn replace_img_blocks(content: String) -> String {
    let mut interim_content: String = String::from(&content);
    IMG_REGEX.captures_iter(&content).for_each(|cap| {
        //tracing::debug!("->> {:<12} - cap:\n{:#?}", module_path!(), cap);
        let img = cap.get(1).unwrap().as_str();
        interim_content = content.replace(img, "");
    });
    interim_content
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

    let wiki_caption_regex = Regex::new(r#"<a\s+href="(?P<href>http[s]?://commons\.wikimedia\.org/wiki/File:.*?)".*?>(?P<caption>.*?)</a>"#).unwrap();

    let captions = caption_regex
        .captures_iter(content)
        .map(|cap| {
            let mut href = cap.name("href").map(|m| m.as_str());
            let img = cap.name("img").map(|m| m.as_str());
            let figure = cap.name("figure").map(|m| m.as_str());
            let mut caption_text = None;

            if let Some(c) = cap.name("caption") {
                if let Some(wiki_cap) = wiki_caption_regex.captures(c.as_str()) {
                    href = wiki_cap.name("href").map(|h| h.as_str());
                    caption_text = wiki_cap.name("caption").map(|c| c.as_str());
                } else {
                    caption_text = Some(c.as_str());
                }
            }

            Caption {
                href: href.map(|s| s.to_string()),
                img: img.map(|s| s.to_string()),
                figure: figure.map(|s| s.to_string()),
                text: caption_text.map(|s| s.to_string()),
            }
        })
        .collect::<Vec<_>>();

    Ok(captions)
}

#[derive(Debug, Clone)]
pub struct Caption {
    href: Option<String>,
    img: Option<String>,
    figure: Option<String>,
    text: Option<String>,
}

pub async fn get_commons_url(url: Url) -> Result<Option<Url>> {
    // Check if wikimedia File: url
    if !url.path().contains("/wiki/File:") {
        return Ok(Some(url));
    }

    // Get page HTML
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.text().await?;

    // Extract commons URL using regex
    let re = Regex::new(
        r#"<a href="(https://upload\.wikimedia\.org/wikipedia/commons/[^"]+)"[^>]+class="internal""#,
    )?;

    Ok(re
        .captures(&response)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .map(|s| Url::parse(&s).expect("Failed to parse URL")))
}

async fn replace_caption(
    mm: &ModelManager,
    article: &Articles,
    _figure: Option<&str>,
    caption_text: Option<&str>,
    url: &Url,
    image_id: Uuid,
) -> Result<()> {
    let article = Articles::select()
        .where_("id = ?")
        .bind(article.id)
        .fetch_one(mm.orm())
        .await?;

    //tracing::debug!("->> {:<12} - article:\n{:#?}", file!(), article);
    //tracing::debug!("->> {:<12} - figure:\n{:#?}", file!(), figure);
    //tracing::debug!("->> {:<12} - caption_text:\n{:#?}", file!(), caption_text);

    let regex_str = format!(r#"(?x)(\\\[caption.*?{}.*?\\\[/caption\\\])"#, url.as_str(),);

    //tracing::debug!("->> {:<12} - regex_str:\n{:#?}", module_path!(), regex_str);

    let a_tag_regex = Regex::new(r#"(?x)<a.*?>"#).unwrap();

    let caption_regex = Regex::new(&regex_str).unwrap();

    if let Some(article_body) = &article.body
        && let Some(caption_text) = caption_text
    {
        let caption_text = a_tag_regex.replace(caption_text, "");
        let new_caption = format!(
            "![{}](/assets/{})",
            caption_text
                .replace("<em>", "")
                .replace("</em>", "")
                .replace("</a>", "")
                .trim(),
            image_id,
        );
        //tracing::debug!(
        //    "->> {:<12} - new_caption:\n{:#?}",
        //    module_path!(),
        //    new_caption
        //);

        let article_body = caption_regex.replace(article_body, new_caption.clone());

        article
            .update_partial()
            .body(Some(article_body.to_string()))
            .update(mm.orm())
            .await?;
    }

    Ok(())
}

async fn replace_img_tag(
    mm: &ModelManager,
    article: &Articles,
    url: &Url,
    image_id: Uuid,
) -> Result<()> {
    let article = Articles::select()
        .where_("id = ?")
        .bind(article.id)
        .fetch_one(mm.orm())
        .await?;

    let regex_str = format!(r#"(?x)\!\[.*?\]\({}\)"#, url.as_str(),);

    let caption_regex = Regex::new(&regex_str).unwrap();

    if let Some(article_body) = &article.body {
        let new_caption = format!("![](/assets/{})", image_id,);

        //tracing::debug!(
        //    "->> {:<12} - new_caption:\n{:#?}",
        //    module_path!(),
        //    new_caption
        //);

        let article_body = caption_regex.replace(article_body, new_caption.clone());

        article
            .update_partial()
            .body(Some(article_body.to_string()))
            .update(mm.orm())
            .await?;
    }

    Ok(())
}

use csv::ReaderBuilder;
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CsvRecord {
    post_id: String,
    article_title: String,
    article_slug: String,
    featured_image_id: Option<String>,
    featured_image_url: Option<String>,
}

pub async fn process_csv_and_update_featured_images(mm: &ModelManager) -> Result<()> {
    let file = File::open("/home/eran/code/rust-web-app/export.csv")?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    for record in reader.deserialize::<CsvRecord>() {
        let record: CsvRecord = match record {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!("Skipping invalid record: {}", e);
                continue;
            }
        };

        if record.featured_image_url == Some("\\N".to_string()) {
            continue;
        }

        tracing::debug!("->> {:<12} - record:\n{:#?}", module_path!(), record);

        // Process each row
        if let Err(e) = process_record(mm, &record).await {
            tracing::error!(
                "Failed to process record for slug {}: {:?}",
                record.article_slug,
                e
            );
        }
    }

    Ok(())
}

async fn process_record(mm: &ModelManager, record: &CsvRecord) -> Result<()> {
    let article = Articles::select()
        .where_("slug = ?")
        .bind(&record.article_slug)
        .fetch_one(mm.orm())
        .await?;

    if let Some(image_url) = &record.featured_image_url {
        let url = Url::parse(image_url)?;

        let downloaded_image = process_image_url()
            .mm(mm)
            .article(&article)
            .url(&url)
            .title(&record.article_title)
            .index(0)
            .call()
            .await?;

        article
            .update_partial()
            .featured_image(Some(downloaded_image.directus_files_id))
            .update(mm.orm())
            .await?;
    } else {
        info!(
            "No featured image URL for article '{}', skipping.",
            record.article_slug
        );
    }

    Ok(())
}
