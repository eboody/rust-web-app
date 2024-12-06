use lib_core::model::directus::{Articles, Posts, Users};
use ormlite::{
	model::{Model, ModelBuilder},
	postgres::PgPool,
	types::{time::OffsetDateTime, Uuid},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let orm = ormlite::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&std::env::var("DIRECTUS_DB").unwrap())
		.await
		.unwrap();

	let posts = Posts::select().fetch_all(&orm).await.unwrap();
	for post in posts {
		create_article(orm.clone(), post).await?;
	}
	Ok(())
}

fn convert_to_footnotes(content: &str) -> String {
	let re = regex::Regex::new(r"\]\(#_(ftn|end)ref(\d+)\)").unwrap();
	re.replace_all(content, "](#_$1$2)").to_string()
}

async fn create_article(
	orm: PgPool,
	post: Posts,
) -> Result<(), Box<dyn std::error::Error>> {
	let html_content = post
		.content
		.replace("\\r\\n", "<br>")
		.replace("\r\n", "<br>");
	let content_markdown =
		htmd::convert(&html_content).expect("Failed to convert HTML to MD");
	let endnotes_markdown =
		htmd::convert(&post.endnotes.clone().unwrap_or_default())
			.expect("Failed to convert HTML to MD");

	let content = convert_to_footnotes(&content_markdown);
	let endnotes = convert_to_footnotes(&endnotes_markdown);

	let re = regex::Regex::new(r"\s+\[(\d+)\]\(#_?(ftn|end|edn)(ref)?-?\d+\)\.?\s*")
		.unwrap();
	let endnotes = re.replace_all(&endnotes, "\n\n[$1](#_$2$1) ").to_string();

	let re = regex::Regex::new(r"\s*?\*\*Related:\*\*.*").unwrap();
	let endnotes = re.replace_all(&endnotes, "").to_string();

	let re = regex::Regex::new(
		r"\s*?(_Like this post\?|If you enjoyed this post, consider|\\\[bctt tweet.*?\\\]).*",
	)
	.unwrap();
	let content = re.replace_all(&content, "").to_string();

	let re = regex::Regex::new(r"\s*?\\\[/?groups\\_can.*?\\\]").unwrap();
	let endnotes = re.replace_all(&endnotes, "").to_string();
	let content = re.replace_all(&content, "").to_string();

	let re = regex::Regex::new(r"\s*?_Dowload the .*? of this article\._").unwrap();
	let content = re.replace_all(&content, "").to_string();

	let re = regex::Regex::new(r"\n\s*?\n\s*?\n\s*?\n").unwrap();
	let content = re.replace_all(&content, '\n'.to_string()).to_string();

	let re = regex::Regex::new(r"\\\[(\w)\\\]").unwrap();
	let content = re.replace_all(&content, "$1").to_string();

	let content = content.trim().to_string();

	let title = htmd::convert(&post.title).expect("Failed to convert HTML to MD");

	let users = Users::select().fetch_all(&orm).await.unwrap();

	let author = if let Some(user) = users
		.iter()
		.filter(|u| u.first_name.is_some() && u.last_name.is_some())
		.find(|u| {
			let split = post.author.split(" ").collect::<Vec<&str>>();
			let first_name = split[0];
			let last_name = split[1..].join(" ");
			u.first_name.as_ref().unwrap() == first_name
				&& u.last_name.as_ref().unwrap() == &last_name
		}) {
		user.clone()
	} else {
		dbg!("User does not exist");
		println!("Author: {}", post.author);
		let mut split = post.author.split(" ").collect::<Vec<&str>>();

		let first_name = split[0];
		split.remove(0);
		let last_name = split.join(" ");

		Users::builder()
			.id(Uuid::new_v4())
			.first_name(Some(first_name.to_string()))
			.last_name(Some(last_name.to_string()))
			.insert(&orm)
			.await?
	};

	let new_article = Articles::builder()
		.id(Uuid::new_v4())
		.status("draft".to_string())
		.sort(Some(0))
		.user_created(None)
		.date_created(Some(OffsetDateTime::now_utc()))
		.user_updated(None)
		.date_updated(Some(OffsetDateTime::now_utc()))
		.featured_image(None)
		.date_published(Some(post.date.date()))
		.content(Some(content))
		.title(Some(title))
		.endnotes(Some(endnotes))
		.summary(None)
		.descriptor(post.descriptor)
		.slug(Some(post.slug))
		.author(author.id)
		.insert(&orm)
		.await?;

	dbg!("new_article: {}", &new_article);

	Ok(())
}
