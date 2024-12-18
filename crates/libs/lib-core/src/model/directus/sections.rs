use ormlite::types::Uuid;

#[derive(Debug, ormlite::Model, Clone)]
pub struct Sections {
  #[ormlite(primary_key)]
  pub id: Uuid,
  pub display_string: String,
  pub description: String,
  pub substack_id: Option<i64>,
}
