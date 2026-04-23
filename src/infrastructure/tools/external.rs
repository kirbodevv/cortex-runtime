use std::{fs, path::PathBuf};

use serde::Deserialize;
use serde_json::Value;

use crate::{
    app::{
        dto::Action,
        tools::{Tool, ToolError, ToolProvider, ToolResponse, ToolResult},
    },
    shared::commands::execute_command,
};

#[derive(Deserialize)]
struct ToolConfig {
    name: String,
    args_schema: Value,
    keywords: Vec<String>,
}

pub struct ExternalTool {
    pub path: PathBuf,
    config: ToolConfig,
}

impl ExternalTool {
    pub async fn new(path: PathBuf) -> Result<Self, ToolError> {
        let value = execute_command(&path, &["--describe"])
            .await
            .map_err(|_| ToolError::Failed)?;

        let config = serde_json::from_value(value)?;

        Ok(Self { path, config })
    }
}

#[async_trait::async_trait]
impl Tool for ExternalTool {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn args_schema(&self) -> Value {
        self.config.args_schema.clone()
    }

    fn keywords(&self) -> Vec<&str> {
        self.config.keywords.iter().map(|k| k.as_str()).collect()
    }

    async fn execute(&self, action: Action) -> ToolResult {
        let tool_args = action.args.to_string();
        let command_args = vec!["--run", &tool_args];

        let output = execute_command(&self.path, &command_args)
            .await
            .map_err(|_| ToolError::Failed)?;

        let value = serde_json::from_value(output)?;

        Ok(ToolResponse { message: value })
    }
}

pub struct ExternalToolProvider {
    pub dir: PathBuf,
}

#[async_trait::async_trait]
impl ToolProvider for ExternalToolProvider {
    async fn load_tools(&self) -> Vec<Box<dyn Tool>> {
        let mut tools: Vec<Box<dyn Tool>> = Vec::new();

        let cfg_path = self.dir.join("tools.cfg");

        println!("[LOAD] Loading tools from: {}", cfg_path.display());

        let content = match fs::read_to_string(cfg_path) {
            Ok(c) => c,
            Err(_) => return tools,
        };

        for line in content.lines() {
            let name = line.trim();
            if name.is_empty() {
                continue;
            }

            let path = self.dir.join(name);

            if !path.exists() {
                continue;
            }

            match ExternalTool::new(path).await {
                Ok(tool) => {
                    println!("[LOAD] Loaded tool: {}", tool.name());
                    tools.push(Box::new(tool))
                }
                Err(err) => {
                    eprintln!("Failed to load tool: {}", err);
                    continue;
                }
            }
        }

        tools
    }
}
