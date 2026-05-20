use std::sync::Arc;

use serde_json::{Value, json};

use crate::app::tools::ToolRegistry;

pub struct JsonSchemaGenerator {
    tools: Arc<ToolRegistry>,
}

impl JsonSchemaGenerator {
    pub fn new(tools: Arc<ToolRegistry>) -> Self {
        Self { tools }
    }

    pub fn generate(&self, query: &str) -> Value {
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
