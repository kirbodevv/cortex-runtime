use std::sync::Arc;

use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec},
};
use serde_json::json;

use crate::{
    app::{
        ports::{LLMClient, LLMError},
        tools::ToolRegistry,
    },
    domain::{Context, LLMRawResponse, LLMRequest, Message, Role},
};

pub struct OpenAIClient {
    client: Client,
    tools: Arc<ToolRegistry>,
}

impl OpenAIClient {
    pub fn new(client: Client, tools: Arc<ToolRegistry>) -> Self {
        Self { client, tools }
    }
}

impl From<&Message> for ChatMessage {
    fn from(message: &Message) -> Self {
        match message.role {
            Role::User => ChatMessage::user(message.content.clone()),
            Role::Assistant => ChatMessage::assistant(message.content.clone()),
        }
    }
}

fn format_memory_and_input(ctx: &Context, input: &str) -> String {
    format!(
        "From memory:\n{}\n\nUser message:\n{}",
        ctx.get()
            .iter()
            .map(|m| m.content())
            .collect::<Vec<_>>()
            .join("\n"),
        input
    )
}

#[async_trait::async_trait]
impl LLMClient for OpenAIClient {
    async fn generate(&self, req: LLMRequest) -> Result<LLMRawResponse, LLMError> {
        let context = req.context();

        let messages = req
            .messages()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                if i == req.messages().len() - 1 {
                    Message::user(format_memory_and_input(context, &m.content))
                } else {
                    m.clone()
                }
            })
            .collect::<Vec<_>>();

        let request = ChatRequest::new(messages.iter().map(|m| m.into()).collect());

        let any_of = self
            .tools
            .get_modules_json_schema(&req.messages().last().unwrap().content);
        let should_use_modules = !any_of.is_empty();

        let items = if should_use_modules {
            json!({ "anyOf": any_of })
        } else {
            json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            })
        };

        let json_schema = json!({
          "type": "object",
          "properties": {
            "response": {
              "type": "string",
            },
            "mem": {
              "type": "array",
              "description": "Store only relevant memories in user's language",
              "items": {
                "type": "object",
                "properties": {
                  "summary": {
                    "type": "string",
                  },
                  "importance": {
                    "type": "number",
                  }
                },
                "required": ["summary", "importance"],
                "additionalProperties": false
              }
            },
            "tool_call": {
                "type": "array",
                "items": items
            }
          },
          "required": ["response", "mem", "tool_call"],
          "additionalProperties": false
        });

        let chat_options =
            ChatOptions::default().with_response_format(JsonSpec::new("main-schema", json_schema));

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
