use ormlite::model::{Join, JoinMeta};
use uuid::Uuid;

#[derive(Debug, ormlite::Model)]
pub struct Ebooks {
	#[ormlite(primary_key)]
	pub id: Uuid,
	pub status: String,
	pub sort: Option<i32>,
	pub user_created: Option<Uuid>,
	pub date_created: Option<String>,
	pub user_updated: Option<Uuid>,
	pub date_updated: Option<String>,
	pub date_published: Option<String>,
}

#[derive(Debug, ormlite::Model)]
pub struct EbooksDirectusUsers {
	#[ormlite(primary_key)]
	pub id: i32,
	pub ebooks_id: Option<Uuid>,
	pub directus_users_id: Option<Uuid>,
}

#[derive(Debug, ormlite::Model)]
pub struct EbooksTags {
	#[ormlite(primary_key)]
	pub id: i32,
	pub ebooks_id: Option<Uuid>,
	pub tags_id: Option<i32>,
}

#[derive(Debug, ormlite::Model)]
pub struct EbooksTranslations {
	#[ormlite(primary_key)]
	pub id: Uuid,
	#[ormlite(join_column = "ebooks_id")]
	pub ebook: Join<Ebooks>,
	pub languages_code: Option<String>,
	pub slug: Option<String>,
	pub summary: Option<String>,
	pub descriptor: Option<String>,
	pub cover_image: Option<Uuid>,
	pub file: Option<Uuid>,
	pub title: Option<String>,
}
