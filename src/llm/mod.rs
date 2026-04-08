pub mod anthropic;
pub mod gemini;
pub mod openai;
pub mod types;

use crate::config::LlmConfig;
use crate::error::AppError;
use types::*;

pub trait LlmProvider: Send + Sync {
    fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError>;
}

pub fn create_provider(config: &LlmConfig) -> Result<Box<dyn LlmProvider>, AppError> {
    let api_type = config.api_type.as_deref().unwrap_or("");

    match api_type {
        "custom" | "openai-completions" | "openai-responses" => {
            Ok(Box::new(openai::OpenAiProvider::new(config)?))
        }
        "anthropic-messages" => Ok(Box::new(anthropic::AnthropicProvider::new(config)?)),
        "gemini-generate-content" => Ok(Box::new(gemini::GeminiProvider::new(config)?)),
        _ => match config.provider.to_lowercase().as_str() {
            "ollama" => Ok(Box::new(openai::OpenAiProvider::new(config)?)),
            "anthropic" => Ok(Box::new(anthropic::AnthropicProvider::new(config)?)),
            "gemini" => Ok(Box::new(gemini::GeminiProvider::new(config)?)),
            _ => Ok(Box::new(openai::OpenAiProvider::new(config)?)),
        },
    }
}
