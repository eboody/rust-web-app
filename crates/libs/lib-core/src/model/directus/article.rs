use uuid::Uuid;

#[derive(Debug, ormlite::Model)]
pub struct Articles {
	#[ormlite(primary_key)]
	pub id: i32,
	pub status: String,
	pub sort: Option<i32>,
	pub user_created: Option<Uuid>,
	pub date_created: Option<String>,
	pub user_updated: Option<Uuid>,
	pub date_updated: Option<String>,
	pub featured_image: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
pub struct ArticlesDirectusUsers {
	#[ormlite(primary_key)]
	pub id: i32,
	pub articles_id: Option<i32>,
	pub directus_users_id: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
pub struct ArticlesTags {
	#[ormlite(primary_key)]
	pub id: i32,
	pub articles_id: Option<i32>,
	pub tags_id: Option<i32>,
}

#[derive(Debug, ormlite::Model)]
pub struct ArticlesTranslations {
	#[ormlite(primary_key)]
	pub id: i32,
	pub articles_id: Option<i32>,
	pub languages_code: Option<String>,
	pub content: Option<String>,
	pub title: Option<String>,
	pub slug: Option<String>,
	pub summary: Option<String>,
	pub descriptor: Option<String>,
}
