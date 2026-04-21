use serde_json::{Value, json};

use crate::app::{
    dto::Action,
    tools::{Tool, ToolError, ToolResponse, ToolResult},
};

pub struct EchoModule;

impl Tool for EchoModule {
    fn name(&self) -> &str {
        "echo"
    }
    fn description(&self) -> Value {
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

    fn keywords(&self) -> &[&str] {
        &["echo", "эхо", "консоль", "вывести", "текст"]
    }

    fn execute(&self, action: Action) -> ToolResult {
        let message = action
            .args
            .get("message")
            .ok_or(ToolError::JSON("Bad json".to_string()))?
            .as_str()
            .unwrap_or("");
        println!("[ECHO]: {}", message);
        Ok(ToolResponse {
            message: message.to_string(),
        })
    }
}
