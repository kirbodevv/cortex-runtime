use openai_api_rust::{
    Auth, Message, OpenAI, Role,
    chat::{ChatApi, ChatBody},
};

use crate::{domain::error::AppError, services::llm::LLMService};

pub struct OpenAi {
    openai: OpenAI,
    messages: Vec<Message>,
}

impl OpenAi {
    pub fn new(auth: Auth) -> Self {
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        Self {
            openai,
            messages: vec![Message {
                role: Role::System,
                content: "Ты полезный ассистент.".to_string(),
            }],
        }
    }
}

impl LLMService for OpenAi {
    fn process(&mut self, input: &str, _context: Vec<String>) -> Result<String, AppError> {
        self.messages.push(Message {
            role: Role::User,
            content: input.to_string(),
        });

        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
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
            messages: self.messages.clone(),
        };

        let rs = self
            .openai
            .chat_completion_create(&body)
            .map_err(|_| AppError::LLMError)?;

        let message = rs.choices[0]
            .message
            .as_ref()
            .ok_or_else(|| AppError::LLMError)?;

        let content = message.content.clone();

        self.messages.push(Message {
            role: Role::Assistant,
            content: content.clone(),
        });

        Ok(content)
    }
}
