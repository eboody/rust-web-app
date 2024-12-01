#[derive(Debug, ormlite::Model)]
pub struct Tags {
	#[ormlite(primary_key)]
	pub id: i32,
	pub value: Option<String>,
}

#[derive(Debug, ormlite::Model)]
pub struct TagsTranslations {
	#[ormlite(primary_key)]
	pub id: i32,
	pub tags_id: Option<i32>,
	pub languages_code: Option<String>,
}
