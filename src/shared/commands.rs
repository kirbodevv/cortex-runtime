use std::{error::Error, path::PathBuf};

use serde_json::Value;
use tokio::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum CommandsError {
    #[error("command not found")]
    NotFound,
    #[error("bad output")]
    BadOutput(#[from] Box<dyn Error>),
    #[error("bad json: {0}")]
    BadJSON(#[source] serde_json::Error),
}

pub async fn execute_command(path: &PathBuf, args: &[&str]) -> Result<Value, CommandsError> {
    let output = Command::new(path)
        .args(args)
        .output()
        .await
        .map_err(|_| CommandsError::NotFound)?;

    let stdout =
        String::from_utf8(output.stdout).map_err(|e| CommandsError::BadOutput(Box::new(e)))?;

    let result: Value = serde_json::from_str(&stdout).map_err(|e| CommandsError::BadJSON(e))?;

    Ok(result)
}
