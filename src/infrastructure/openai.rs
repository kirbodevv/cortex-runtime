use openai_api_rust::{
    Message, OpenAI, Role,
    chat::{ChatApi, ChatBody},
};
use serde_json::from_str;

use crate::{app::dto::LLMResponse, domain::error::AppError, services::llm::LLMService};

pub struct OpenAi {
    openai: OpenAI,
    messages: Vec<Message>,
}

impl OpenAi {
    pub fn new(openai: OpenAI) -> Self {
        Self {
            openai,
            messages: vec![Message {
                role: Role::System,
                content: r#"e.to_string()
                    Отвечай строго в формате Json:

                    {
                        "response": string,
                        "actions": [actions],
                        "memory_candidates": [memory_candidates]
                    }

                    //actions:
                    {
                        "type": string,
                        "args": [string]
                    }

                    //memory_candidates:
                    {
                        "summary": string,
                        "importance": float
                    }

                    НИКОГДА НЕ ОТВЕЧАЙ ПОЛЬЗОВАТЕЛЮ ПРОСТЫМ ТЕКСТОМ. НЕ ЗАПИСЫВАЙ В ПАМЯТЬ ТО, ЧТО УЖЕ ЗАПИСАНО.
                    "#
                .to_string(),
            }],
        }
    }
}

impl LLMService for OpenAi {
    async fn process(
        &mut self,
        input: &str,
        context: Vec<String>,
    ) -> Result<LLMResponse, AppError> {
        let mut messages = self.messages.clone();

        messages.push(Message {
            role: Role::System,
            content: format!("Из памяти:\n{}", context.join("\n")),
        });

        messages.push(Message {
            role: Role::User,
            content: input.to_string(),
        });

        let body = ChatBody {
            model: "gpt-4.1".to_string(),
            max_tokens: Some(200),
            temperature: Some(0.7),
            top_p: None,
            n: None,
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages,
        };

        let rs = self
            .openai
            .chat_completion_create(&body)
            .map_err(|e| AppError::LLMError(e.to_string()))?;

        let message = rs.choices[0]
            .message
            .as_ref()
            .ok_or_else(|| AppError::LLMError("No message in response".to_string()))?;

        let content = message.content.clone();

        /*self.messages.push(Message {
            role: Role::Assistant,
            content: content.clone(),
        });*/

        Ok(from_str::<LLMResponse>(&content).map_err(|e| AppError::LLMError(e.to_string()))?)
    }
}
