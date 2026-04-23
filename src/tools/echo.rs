use serde_json::{Value, json};

use crate::app::{
    dto::Action,
    tools::{Tool, ToolError, ToolResponse, ToolResult},
};

pub struct EchoModule;

#[async_trait::async_trait]
impl Tool for EchoModule {
    fn name(&self) -> &str {
        "echo"
    }
    fn args_schema(&self) -> Value {
        json!(
        {
            "type": "object",
            "properties": {
                "type": { "type": "string", "const": "echo" },
                "args": {
                    "type": "object",
                    "properties": {
                      "message": { "type": "string" }
                    },
                    "required": ["message"],
                    "additionalProperties": false
                  }
                },
            "required": ["type", "args"],
            "additionalProperties": false
        }
        )
    }

    fn keywords(&self) -> Vec<&str> {
        vec!["echo", "эхо", "консоль", "вывести", "текст"]
    }

    async fn execute(&self, action: Action) -> ToolResult {
        let message = action
            .args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::BadArgs("message is missing or not a string".to_string()))?;

        println!("[ECHO]: {}", message);

        Ok(ToolResponse {
            message: message.to_string(),
        })
    }
}
