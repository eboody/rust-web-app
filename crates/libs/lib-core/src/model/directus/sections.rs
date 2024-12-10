use ormlite::types::Uuid;

#[derive(Debug, ormlite::Model)]
pub struct Sections {
	#[ormlite(primary_key)]
	pub id: Uuid,
	pub display_string: String,
}
