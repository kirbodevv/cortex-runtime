use std::sync::Arc;

use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec},
};
use serde_json::json;

use crate::{
    app::{ports::LLMClient, tools::ToolRegistry},
    domain::{LLMRawResponse, LLMRequest, error::AppError},
};

pub struct OpenAIClient {
    client: Client,
    messages: ChatRequest,
    tools: Arc<ToolRegistry>,
}

impl OpenAIClient {
    pub fn new(client: Client, tools: Arc<ToolRegistry>) -> Self {
        Self {
            client,
            messages: ChatRequest::new(vec![ChatMessage::system(
                "НИКОГДА НЕ ОТВЕЧАЙ ПОЛЬЗОВАТЕЛЮ ПРОСТЫМ ТЕКСТОМ. НЕ ЗАПИСЫВАЙ В ПАМЯТЬ ТО, ЧТО УЖЕ ЗАПИСАНО.",
            )]),
            tools,
        }
    }
}

#[async_trait::async_trait]
impl LLMClient for OpenAIClient {
    async fn generate(&self, req: LLMRequest) -> Result<LLMRawResponse, AppError> {
        let input = &req
            .messages
            .last()
            .ok_or(AppError::RuntimeError("No messages".to_string()))?
            .content;

        let messages = self.messages.clone().append_messages(vec![
            ChatMessage::user(format!(
                "Из памяти:\n{}",
                req.context
                    .get()
                    .iter()
                    .map(|m| m.content())
                    .collect::<Vec<_>>()
                    .join("\n")
            )),
            ChatMessage::user(input),
        ]);

        let any_of = self.tools.get_modules_json_schema(input);
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
            "actions": {
                "type": "array",
                "items": items
            }
          },
          "required": ["response", "mem", "actions"],
          "additionalProperties": false
        });

        let chat_options =
            ChatOptions::default().with_response_format(JsonSpec::new("main-schema", json_schema));

        let rs = self
            .client
            .exec_chat("gpt-4o-mini", messages, Some(&chat_options))
            .await
            .map_err(|e| AppError::LLMError(e.to_string()))?;

        let message = rs.first_text();
        let message = message
            .as_ref()
            .ok_or_else(|| AppError::LLMError("No message in response".to_string()))?;

        /*self.messages.push(Message {
            role: Role::Assistant,
            content: content.clone(),
        });*/

        Ok(LLMRawResponse {
            text: message.to_string(),
        })
    }
}
