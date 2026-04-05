use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ConfigError(String),
    LlmError(String),
    SecurityBlocked(String),
    IoError(String),
    HistoryError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            AppError::LlmError(msg) => write!(f, "LLM error: {}", msg),
            AppError::SecurityBlocked(cmd) => write!(f, "BLOCKED:{}", cmd),
            AppError::IoError(msg) => write!(f, "IO error: {}", msg),
            AppError::HistoryError(msg) => write!(f, "History error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
