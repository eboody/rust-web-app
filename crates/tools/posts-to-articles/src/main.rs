use async_recursion::async_recursion;
use lib_automation::directus::tasks;
use lib_core::{
  model::{
    ModelManager,
    directus::{Articles, RelatedArticles, Status, Users, WpPosts},
  },
  prelude::OffsetDateTime,
};
use ormlite::{
  model::{Model, ModelBuilder},
  types::Uuid,
};
use regex::Regex;
use std::collections::HashSet;
use tracing::{info, warn};

// Main entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  setup_logging()?;
  info!("Starting article migration...");

  let mm = ModelManager::new().await?;
  let mut migrator = ArticleMigrator::new(&mm);
  migrator.migrate_all_posts().await?;

  Ok(())
}

struct ArticleMigrator<'a> {
  mm: &'a ModelManager,
  processed_slugs: HashSet<String>,
}

fn take_first_n(n: usize, input: &str) -> String {
  if input.chars().count() <= n {
    input.to_string()
  } else {
    input.chars().take(n).collect()
  }
}

impl<'a> ArticleMigrator<'a> {
  fn new(mm: &'a ModelManager) -> Self {
    Self {
      mm,
      processed_slugs: HashSet::new(),
    }
  }

  async fn migrate_all_posts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let posts = WpPosts::select()
      .where_("endnotes IS NOT NULL")
      .where_("endnotes != ''")
      .where_("author != 'Yaron Brook'")
      .where_("content LIKE '%https://theobjectivestandard.com%/202%'")
      .where_("content NOT LIKE '%https://theobjectivestandard.com%uploads%'")
      .fetch_all(self.mm.orm())
      .await?;

    for post in posts {
      let is_article =
        tasks::is_article(self.mm, &take_first_n(1500, &post.content)).await?;
      if !is_article {
        tracing::warn!("Skipping non-article post: {:?}", post.title);
        continue;
      }

      if let Err(e) = self.process_post(post).await {
        warn!("Failed to process post: {:?}", e);
      }
      std::process::exit(0);
    }

    Ok(())
  }

  #[async_recursion(?Send)]
  async fn process_post(
    &mut self,
    post: WpPosts,
  ) -> Result<Articles, Box<dyn std::error::Error>> {
    // Get or create the article
    let article =
      if let Ok(existing_article) = self.get_existing_article(&post.slug).await {
        existing_article
      } else {
        let new_article = self.create_article(&post).await?;
        tasks::add_tags(self.mm, &new_article).await?;
        tracing::info!("Added tags");
        tasks::add_subtitle(self.mm, &new_article).await?;
        tracing::info!("Added subtitle");
        tasks::select_section(self.mm, &new_article).await?;
        tracing::info!("Selected section");
        tasks::handle_images(self.mm, &new_article).await?;
        tracing::info!("Handled images");
        new_article
      };

    // Only process related articles if we haven't seen this slug before
    if !self.processed_slugs.contains(&post.slug) {
      self.processed_slugs.insert(post.slug.clone());

      // Process related articles
      let related_slugs = self.extract_related_slugs(&format!(
        "{}\n{}",
        post.content,
        post.endnotes.unwrap_or_default()
      ));
      for slug in related_slugs {
        if let Some(related_post) = self.get_wp_post_by_slug(&slug).await? {
          let related_article = self.process_post(related_post).await?;
          self
            .create_article_relationship(&article, &related_article)
            .await?;
        }
      }
    }

    Ok(article)
  }

  async fn create_article_relationship(
    &self,
    article: &Articles,
    related_article: &Articles,
  ) -> Result<(), Box<dyn std::error::Error>> {
    // Check if relationship already exists
    let existing = RelatedArticles::select()
      .where_("articles_id = ? AND related_articles_id = ?")
      .bind(article.id)
      .bind(related_article.id)
      .fetch_optional(self.mm.orm())
      .await?;

    if existing.is_none() {
      // Only create if it doesn't exist
      RelatedArticles::builder()
        .articles_id(article.id)
        .related_articles_id(related_article.id)
        .insert(self.mm.orm())
        .await?;

      info!(
        "Created relationship: {} -> {}",
        article.slug.as_deref().unwrap_or("unknown"),
        related_article.slug.as_deref().unwrap_or("unknown")
      );
    }

    Ok(())
  }

  async fn get_existing_article(
    &self,
    slug: &str,
  ) -> Result<Articles, Box<dyn std::error::Error>> {
    Ok(
      Articles::select()
        .where_("slug = ?")
        .bind(slug)
        .fetch_one(self.mm.orm())
        .await?,
    )
  }

  async fn get_wp_post_by_slug(
    &self,
    slug: &str,
  ) -> Result<Option<WpPosts>, Box<dyn std::error::Error>> {
    Ok(
      WpPosts::select()
        .where_("slug = ?")
        .bind(slug)
        .fetch_optional(self.mm.orm())
        .await?,
    )
  }

  async fn create_article(
    &self,
    post: &WpPosts,
  ) -> Result<Articles, Box<dyn std::error::Error>> {
    let author = self.get_or_create_author(&post.author).await?;
    let (content, endnotes) = self.process_content(post)?;

    let title = Some(
      htmd::convert(&post.title)?
        .replace("_", "")
        .replace(r#"*"#, ""),
    );

    let article = Articles::builder()
      .id(Uuid::new_v4())
      .status(Status::Archived)
      .sort(Some(0))
      .date_created(Some(OffsetDateTime::now_utc()))
      .date_updated(Some(OffsetDateTime::now_utc()))
      .date_published(Some(post.date.date()))
      .body(Some(content))
      .title(title)
      .endnotes(Some(endnotes))
      .descriptor(post.descriptor.clone())
      .slug(Some(post.slug.clone()))
      .author_id(author.id)
      .insert(self.mm.orm())
      .await?;

    info!("Created article: {:?}", article.title);
    Ok(article)
  }

  async fn get_or_create_author(
    &self,
    author_name: &str,
  ) -> Result<Users, Box<dyn std::error::Error>> {
    let users = Users::select().fetch_all(self.mm.orm()).await?;

    if let Some(user) = self.find_matching_user(&users, author_name) {
      return Ok(user);
    }

    let (first_name, last_name) = split_author_name(author_name);

    Ok(
      Users::builder()
        .id(Uuid::new_v4())
        .first_name(Some(first_name))
        .last_name(Some(last_name))
        .insert(self.mm.orm())
        .await?,
    )
  }

  fn find_matching_user(&self, users: &[Users], author_name: &str) -> Option<Users> {
    let (first_name, last_name) = split_author_name(author_name);

    users
      .iter()
      .find(|u| {
        u.first_name.as_deref() == Some(&first_name)
          && u.last_name.as_deref() == Some(&last_name)
      })
      .cloned()
  }

  fn process_content(
    &self,
    post: &WpPosts,
  ) -> Result<(String, String), Box<dyn std::error::Error>> {
    let content = self.clean_html_content(&post.content)?;
    let content_md = htmd::convert(&content)?;

    let endnotes = post.endnotes.clone().unwrap_or_default();
    let endnotes_md = htmd::convert(&endnotes)?;

    let content = self.apply_content_transformations(&content_md)?;
    let endnotes = self.apply_endnotes_transformations(&endnotes_md)?;

    Ok((content, endnotes))
  }

  fn extract_related_slugs(&self, content: &str) -> Vec<String> {
    let re = Regex::new(
      r"https://(?:www\.)?theobjectivestandard.com/(?:(?:(?:issues|special)/\d{4}-[a-z]+/)|(?:\d+|(?:[a-zA-Z|\-]+))/)*(?P<slug>[a-zA-Z|\-]+)[/|)]",
    )
    .expect("Invalid regex pattern");

    re.captures_iter(content)
      .map(|cap| cap[1].to_string())
      .collect()
  }

  // Helper methods for content processing
  fn clean_html_content(
    &self,
    content: &str,
  ) -> Result<String, Box<dyn std::error::Error>> {
    Ok(content.replace("\\r\\n", "<br>").replace("\r\n", "<br>"))
  }

  fn apply_content_transformations(
    &self,
    content: &str,
  ) -> Result<String, Box<dyn std::error::Error>> {
    let content = convert_to_footnotes(content);
    let content = remove_related_section(&content);
    let content = remove_social_elements(&content);
    let content = clean_formatting(&content);

    Ok(content.trim().to_string())
  }

  fn apply_endnotes_transformations(
    &self,
    endnotes: &str,
  ) -> Result<String, Box<dyn std::error::Error>> {
    let endnotes = convert_to_footnotes(endnotes);
    let endnotes = format_footnote_references(&endnotes);
    let endnotes = remove_related_section(&endnotes);
    let endnotes = clean_formatting(&endnotes);

    Ok(endnotes)
  }
}

// Utility functions
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
  if tracing_subscriber::fmt()
    .without_time()
    .with_target(false)
    .try_init()
    .is_err()
  {
    warn!("Tracing subscriber already initialized");
  }
  Ok(())
}

fn split_author_name(author_name: &str) -> (String, String) {
  let mut parts: Vec<&str> = author_name.split_whitespace().collect();
  let first_name = parts.remove(0).to_string();
  let last_name = parts.join(" ");
  (first_name, last_name)
}

fn convert_to_footnotes(content: &str) -> String {
  let re = Regex::new(r"\]\(#_(ftn|end)ref(\d+)\)").unwrap();
  re.replace_all(content, "](#_$1$2)").to_string()
}

fn format_footnote_references(content: &str) -> String {
  let re =
    Regex::new(r"\s+\[(\d+)\]\(#_?([a-zA-Z]{2,3})(ref)?-?\d+\)\.?\s*").unwrap();
  re.replace_all(content, "\n\n[$1](#_$2$1) ").to_string()
}

fn remove_related_section(content: &str) -> String {
  let re =
    Regex::new(r"(?s)(?:\s*?\*\*)?Related(?:\sArticles)?:(?:\*\*)?.*").unwrap();
  re.replace_all(content, "").to_string()
}

fn remove_social_elements(content: &str) -> String {
  let re = Regex::new(
        r"(?s)\s*?(_Like this post\?|If you enjoyed this post, consider|\\\[bctt tweet.*?\\\]).*",
    )
    .unwrap();
  re.replace_all(content, "").to_string()
}

fn clean_formatting(content: &str) -> String {
  let content = Regex::new(r"\s*?\\\[/?groups\\_can.*?\\\]")
    .unwrap()
    .replace_all(content, "");

  let content = Regex::new(r"\s*?_Dowload the .*? of this article\._")
    .unwrap()
    .replace_all(&content, "");

  let content = Regex::new(r"\n\s*?\n\s*?\n\s*?\n")
    .unwrap()
    .replace_all(&content, "\n");

  let content = Regex::new(r"\\\[(\w)+\\\]")
    .unwrap()
    .replace_all(&content, "*$1*");

  content.to_string()
}
