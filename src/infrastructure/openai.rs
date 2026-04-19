use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec},
};
use serde_json::{from_str, json};

use crate::{app::dto::LLMResponse, domain::error::AppError, services::llm::LLMService};

pub struct OpenAi {
    client: Client,
    messages: ChatRequest,
    chat_options: ChatOptions,
}

impl OpenAi {
    pub fn new(client: Client) -> Self {
        let json_schema = json!({
          "type": "object",
          "properties": {
            "response": {
              "type": "string",
              "description": "Основной текст ответа пользователю"
            },
            "memory_candidates": {
              "type": "array",
              "description": "Кандидаты для сохранения в память",
              "items": {
                "type": "object",
                "properties": {
                  "summary": {
                    "type": "string",
                    "description": "Краткое описание факта"
                  },
                  "importance": {
                    "type": "number",
                    "description": "Важность (0.0 - 1.0)"
                  }
                },
                "required": ["summary", "importance"],
                "additionalProperties": false
              }
            }
          },
          "required": ["response", "memory_candidates"],
          "additionalProperties": false
        });

        Self {
            client,
            messages: ChatRequest::new(vec![ChatMessage::system(
                "НИКОГДА НЕ ОТВЕЧАЙ ПОЛЬЗОВАТЕЛЮ ПРОСТЫМ ТЕКСТОМ. НЕ ЗАПИСЫВАЙ В ПАМЯТЬ ТО, ЧТО УЖЕ ЗАПИСАНО.",
            )]),
            chat_options: ChatOptions::default()
                .with_response_format(JsonSpec::new("main-schema", json_schema)),
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

        let rs = self
            .client
            .exec_chat("gpt-5.4", messages, Some(&self.chat_options))
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
