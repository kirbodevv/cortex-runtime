use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec},
};

use crate::{
    app::ports::{LLMClient, LLMError},
    domain::{LLMRawResponse, LLMRequest, Message, Role},
};

pub struct OpenAIClient {
    client: Client,
}

impl OpenAIClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl From<&Message> for ChatMessage {
    fn from(message: &Message) -> Self {
        let content = message.content.clone();
        match message.role {
            Role::User => ChatMessage::user(content),
            Role::Assistant => ChatMessage::assistant(content),
            Role::System => ChatMessage::system(content),
        }
    }
}

#[async_trait::async_trait]
impl LLMClient for OpenAIClient {
    async fn generate(&self, req: LLMRequest) -> Result<LLMRawResponse, LLMError> {
        let messages = req.messages.iter().map(|m| m.into()).collect::<Vec<_>>();
        let request = ChatRequest::new(messages);

        let chat_options = ChatOptions::default()
            .with_response_format(JsonSpec::new("main-schema", req.json_schema));

        let rs = self
            .client
            .exec_chat("gpt-4o-mini", request, Some(&chat_options))
            .await
            .map_err(|e| LLMError::ApiError(Box::new(e)))?;

        let message = rs.first_text().ok_or(LLMError::EmptyResponse)?;

        Ok(LLMRawResponse {
            text: message.to_string(),
        })
    }
}
