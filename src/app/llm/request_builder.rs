use std::sync::Arc;

use serde_json::{Value, json};

use crate::{
    app::{session::ChatSession, tools::ToolRegistry},
    config::core::CortexConfig,
    domain::{LLMRequest, MemoryItem, Message},
};

pub struct LLMRequestBuilder {
    system: String,
    tools: Arc<ToolRegistry>,
    config: Arc<CortexConfig>,
}

impl LLMRequestBuilder {
    pub fn new(system: String, tools: Arc<ToolRegistry>, config: Arc<CortexConfig>) -> Self {
        Self {
            system,
            tools,
            config,
        }
    }

    pub fn build(
        &self,
        session: &ChatSession,
        memory: Vec<&MemoryItem>,
        input: impl Into<String>,
    ) -> LLMRequest {
        let mut messages = vec![Message::system(self.system.clone())];

        for m in memory {
            messages.push(Message::system(format!("Memory: {}", m.content())));
        }

        messages.extend(session.recent(self.config.context_window_size).to_vec());

        let json_schema = self.generate_json_schema(&input.into());

        LLMRequest::new(&messages, json_schema)
    }

    fn generate_json_schema(&self, query: &str) -> Value {
        let any_of = self.tools.get_modules_json_schema(query);
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
            "tool_call": {
                "type": "array",
                "items": items
            }
          },
          "required": ["response", "mem", "tool_call"],
          "additionalProperties": false
        });
        json_schema
    }
}
