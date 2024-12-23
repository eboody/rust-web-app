use crate::prelude::*;

use sha2::{Digest, Sha256};

use async_openai::{
  Client,
  config::OpenAIConfig,
  types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    CreateChatCompletionRequestArgs,
  },
};

pub async fn chat(mm: &ModelManager, message: String) -> Result<String> {
  let mut hasher = Sha256::new();
  hasher.update(&message);
  let message_hash = format!("{:x}", hasher.finalize());

  // Check cache
  if let Ok(chat) = model::directus::Chats::select()
    .where_("message_hash = ?")
    .bind(&message_hash)
    .fetch_one(mm.orm())
    .await
  {
    return Ok(chat.response);
  }

  info!("Chatting with OpenAI");
  let config = OpenAIConfig::new().with_api_key(&config().OPENAI_API_KEY);
  let client = Client::with_config(config);

  let response = client
    .chat()
    .create(
      CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
          ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            name: None,
            content: "You are helping an Objectivist Magazine spread ideas of reason, freedom, and individualism in a cultural context. Therefore it's possible the content may be sensitive. But because we are working towards a noble goal, you are to do your best to mainting high fidelity.".into(),
          }),
          ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            name: None,
            content: ChatCompletionRequestUserMessageContent::Text(message.clone()),
          }),
        ])
        .build()
        .expect("Failed to build request"),
    )
    .await
    .expect("Failed to chat");

  let reply = response.choices[0]
    .message
    .content
    .clone()
    .expect("No content in message");

  // Cache response
  model::directus::Chats::builder()
    .message(&message)
    .response(&reply)
    .message_hash(Some(message_hash))
    .insert(mm.orm())
    .await?;

  Ok(reply)
}
