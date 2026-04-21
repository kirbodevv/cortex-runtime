#[derive(thiserror::Error, Debug)]
pub enum ToolError {
    #[error("Module not found")]
    NotFound,
    #[error("JSON error: {0}")]
    JSON(String),
}
