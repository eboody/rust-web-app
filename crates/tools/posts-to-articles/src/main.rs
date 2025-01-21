use async_recursion::async_recursion;
use lib_automation::directus::tasks;
use lib_core::{
    model::{
        ModelManager,
        directus::{Articles, Issues, RelatedArticles, Status, Users, WpPosts},
    },
    prelude::{Deserialize, OffsetDateTime},
};
use ormlite::{
    model::{Model, ModelBuilder},
    types::Uuid,
};
use regex::Regex;
use sqlmo::query::Direction;
use std::collections::HashSet;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

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
        //let chats_without_hashes = Chats::select()
        //  .where_("message_hash IS NULL")
        //  .fetch_all(self.mm.orm())
        //  .await?;
        //println!("chats_without_hashes: {:#?}", chats_without_hashes.len());
        //
        //for chat in chats_without_hashes {
        //  println!("added hash to chat");
        //  let mut hasher = Sha256::new();
        //  hasher.update(&chat.message);
        //  let message_hash = format!("{:x}", hasher.finalize());
        //
        //  chat
        //    .update_partial()
        //    .message_hash(Some(message_hash))
        //    .update(self.mm.orm())
        //    .await?;
        //}

        let posts = WpPosts::select()
            //.where_("endnotes IS NOT NULL")
            //.where_("endnotes != ''")
            //.where_("author != 'Yaron Brook'")
            //.where_("content LIKE '%https://theobjectivestandard.com%/202%'")
            //.where_("content NOT LIKE '%https://theobjectivestandard.com%uploads%'")
            //.where_("title = 'Frank Lloyd Wright Masterpiece Opens to Public for the First Time'")
            //.where_("title LIKE '%Eames: The Architect and the Painter%'")
            .order_by("date", Direction::Desc)
            .fetch_all(self.mm.orm())
            .await?;

        for post in posts {
            if let Ok(_existing_article) = self.get_existing_article(&post.slug).await {
                continue;
            }

            let issue_string = get_issue(post.title.clone());

            let mut issue: Option<Issues> = None;

            if let Some(issue_string) = issue_string {
                let split_string = issue_string.split(" ").collect::<Vec<&str>>();
                let season = split_string[0].to_owned().to_lowercase();
                let year = split_string[1].to_owned();

                let existing_issue = Issues::select()
                    .where_("season = ? AND year = ?")
                    .bind(season.clone())
                    .bind(year.clone())
                    .fetch_one(self.mm.orm())
                    .await;

                if let Ok(existing_issue) = existing_issue {
                    issue = Some(existing_issue);
                } else {
                    issue = Some(
                        Issues::builder()
                            .id(Uuid::new_v4())
                            .season(season.clone())
                            .year(year.clone())
                            .insert(self.mm.orm())
                            .await?,
                    );
                }
            }

            let is_article = tasks::is_article(self.mm, &take_first_n(1500, &post.content)).await?;

            if !is_article {
                tracing::warn!("Skipping non-article post: {:?}", post.title);
                continue;
            }

            if let Err(e) = self.process_post(post, issue).await {
                warn!("Failed to process post: {:?}", e);
            }
            //std::process::exit(0);
        }

        Ok(())
    }

    #[async_recursion(?Send)]
    async fn process_post(
        &mut self,
        post: WpPosts,
        issue: Option<Issues>,
    ) -> Result<Articles, Box<dyn std::error::Error>> {
        // get or create the article
        let article = if let Ok(existing_article) = self.get_existing_article(&post.slug).await {
            existing_article
        } else {
            let new_article = self.create_article(&post, issue.clone()).await?;
            tasks::add_tags(self.mm, &new_article)
                .await
                .expect("Failed to add tags");
            tracing::info!("Added tags");
            tasks::add_subtitle(self.mm, &new_article)
                .await
                .expect("Failed to add subtitle");
            tracing::info!("Added subtitle");
            tasks::select_section(self.mm, &new_article)
                .await
                .expect("Failed to select section");
            tracing::info!("Selected section");
            tasks::handle_images(self.mm, &new_article)
                .await
                .expect("Failed to handle images");
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
                if let Some(related_post) = self
                    .get_wp_post_by_slug(&slug)
                    .await
                    .expect("Failed to fetch related post")
                {
                    let related_article = self.process_post(related_post, issue.clone()).await?;
                    self.create_article_relationship(&article, &related_article)
                        .await
                        .expect("Failed to create relationship");
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
        Ok(Articles::select()
            .where_("slug = ?")
            .bind(slug)
            .fetch_one(self.mm.orm())
            .await?)
    }

    async fn get_wp_post_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<WpPosts>, Box<dyn std::error::Error>> {
        Ok(WpPosts::select()
            .where_("slug = ?")
            .bind(slug)
            .fetch_optional(self.mm.orm())
            .await?)
    }

    async fn create_article(
        &self,
        post: &WpPosts,
        issue: Option<Issues>,
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
            .issue(issue.map(|i| i.id))
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

        Ok(Users::builder()
            .id(Uuid::new_v4())
            .first_name(Some(first_name))
            .last_name(Some(last_name))
            .insert(self.mm.orm())
            .await?)
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

    fn clean_html_content(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(content
            .replace("\r\n", "<br><br><br><br>")
            .replace("\n", "<br>"))
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
        if !endnotes.is_empty() {}
        let endnotes = convert_to_footnotes(endnotes);
        let endnotes = format_footnote_references(&endnotes);
        let endnotes = remove_related_section(&endnotes);
        let endnotes = clean_formatting(&endnotes);

        Ok(endnotes)
    }
}

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    if tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
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
    let re = Regex::new(r"\]\(#_([a-zA-Z]+)ref(\d+)\)").unwrap();
    re.replace_all(content, "](#_$1$2)").to_string()
}

fn format_footnote_references(content: &str) -> String {
    let re = Regex::new(r"\s+\[(\d+)\]\(#_?([a-zA-Z]+)(ref)?-?\d+\)\.?\s*").unwrap();
    re.replace_all(content, "\n\n[$1](#_$2$1) ").to_string()
}

fn remove_related_section(content: &str) -> String {
    let re = Regex::new(r"(?s)(?:\s*?\*\*)?Related(?:\sArticles)?:(?:\*\*)?.*").unwrap();
    re.replace_all(content, "").to_string()
}

fn remove_social_elements(content: &str) -> String {
    let re = Regex::new(
        r"(?s)\s*?(_Like this post\?|If you enjoyed this post, consider|\\\[bctt tweet.*?\\\]).*",
    )
    .unwrap();
    re.replace_all(content, "\n").to_string()
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

#[derive(Deserialize, Debug)]
struct Issue {
    issue_title: String,
    article_titles: Vec<String>,
}

fn get_issue(post_title: String) -> Option<String> {
    let module_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let json_data =
        std::fs::read_to_string(module_dir + "/src/issues.json").expect("Unable to read file");
    let issues: Vec<Issue> = json::from_str(&json_data).unwrap();

    for issue in issues {
        if issue.article_titles.contains(&post_title) {
            return Some(issue.issue_title);
        }
    }

    None
}
