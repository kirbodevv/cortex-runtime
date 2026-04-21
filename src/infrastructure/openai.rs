use std::sync::Arc;

use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec},
};
use serde_json::{from_str, json};

use crate::{
    app::dto::LLMResponse,
    domain::error::AppError,
    services::{llm::LLMService, module::ModuleService},
};

pub struct OpenAi {
    client: Client,
    messages: ChatRequest,
    module: Arc<dyn ModuleService>,
}

impl OpenAi {
    pub fn new(client: Client, module: Arc<dyn ModuleService>) -> Self {
        Self {
            client,
            messages: ChatRequest::new(vec![ChatMessage::system(
                "НИКОГДА НЕ ОТВЕЧАЙ ПОЛЬЗОВАТЕЛЮ ПРОСТЫМ ТЕКСТОМ. НЕ ЗАПИСЫВАЙ В ПАМЯТЬ ТО, ЧТО УЖЕ ЗАПИСАНО.",
            )]),
            module,
        }
    }
}

impl LLMService for OpenAi {
    async fn process(
        &mut self,
        input: &str,
        context: Vec<String>,
    ) -> Result<LLMResponse, AppError> {
        let messages = self.messages.clone().append_messages(vec![
            ChatMessage::user(format!("Из памяти:\n{}", context.join("\n"))),
            ChatMessage::user(input.to_string()),
        ]);

        let any_of = self.module.get_modules_schema(input);
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

        Ok(from_str::<LLMResponse>(message).map_err(|e| AppError::LLMError(e.to_string()))?)
    }
}
