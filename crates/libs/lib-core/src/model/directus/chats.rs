#[derive(Debug, ormlite::Model)]
pub struct Chats {
  pub id: i32,
  pub message: String,
  pub response: String,
  pub message_hash: Option<String>,
}
