use super::types::*;
use super::LlmProvider;
use crate::config::LlmConfig;
use crate::error::AppError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub struct AnthropicProvider {
    base_url: String,
    api_key: String,
    model: String,
    max_tokens: Option<u32>,
    client: Client,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    id: Option<String>,
    model: Option<String>,
    content: Vec<ContentBlock>,
    usage: Option<AnthropicUsage>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: Option<u32>,
    output_tokens: Option<u32>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: Option<ErrorDetail>,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: Option<String>,
}

fn normalize_base_url(base_url: &Option<String>) -> String {
    match base_url {
        None => "https://api.anthropic.com".to_string(),
        Some(url) => url.trim_end_matches('/').to_string(),
    }
}

fn ends_with_version(base_url: &str) -> bool {
    let path = base_url.split("://").nth(1).unwrap_or(base_url);
    let last = path.rsplit('/').next().unwrap_or("");
    let rest = last.strip_prefix('v').unwrap_or("");
    !rest.is_empty() && rest.chars().next().is_some_and(|c| c.is_ascii_digit())
}

fn build_anthropic_url(base_url: &str) -> String {
    if base_url.ends_with("/messages") {
        return base_url.to_string();
    }
    if ends_with_version(base_url) {
        return format!("{}/messages", base_url);
    }
    format!("{}/v1/messages", base_url)
}

fn extract_error_message(status: reqwest::StatusCode, body: &str) -> String {
    if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(body) {
        if let Some(detail) = err_resp.error {
            if let Some(msg) = detail.message {
                return format!("HTTP {}: {}", status, msg);
            }
        }
    }
    format!("HTTP {}: {}", status, body)
}

impl AnthropicProvider {
    pub fn new(config: &LlmConfig) -> Result<Self, AppError> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| AppError::LlmError("Anthropic API key is required".into()))?;

        let base_url = normalize_base_url(&config.base_url);
        let model = config.model.clone();
        let max_tokens = config.max_tokens;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .map_err(|e| AppError::LlmError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            base_url,
            api_key,
            model,
            max_tokens,
            client,
        })
    }
}

impl LlmProvider for AnthropicProvider {
    fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        let url = build_anthropic_url(&self.base_url);

        let mut system_parts: Vec<String> = Vec::new();
        let mut messages: Vec<AnthropicMessage> = Vec::new();

        for msg in req.messages {
            if msg.role == "system" {
                system_parts.push(msg.content);
            } else if msg.role == "assistant" {
                messages.push(AnthropicMessage {
                    role: "assistant".into(),
                    content: msg.content,
                });
            } else {
                messages.push(AnthropicMessage {
                    role: "user".into(),
                    content: msg.content,
                });
            }
        }

        let system = if system_parts.is_empty() {
            None
        } else {
            Some(system_parts.join("\n\n"))
        };

        let max_tokens = req.max_tokens.or(self.max_tokens).unwrap_or(1024);

        let body = AnthropicRequest {
            model: self.model.clone(),
            system,
            messages,
            max_tokens,
        };

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .map_err(|e| AppError::LlmError(format!("Request failed: {}", e)))?;

        let status = resp.status();
        let resp_body = resp
            .text()
            .map_err(|e| AppError::LlmError(format!("Failed to read response: {}", e)))?;

        if !status.is_success() {
            return Err(AppError::LlmError(extract_error_message(
                status, &resp_body,
            )));
        }

        let parsed: AnthropicResponse = serde_json::from_str(&resp_body)
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        let content: String = parsed
            .content
            .iter()
            .filter(|b| b.content_type == "text")
            .filter_map(|b| b.text.clone())
            .collect::<Vec<_>>()
            .join("");

        let usage = parsed
            .usage
            .map(|u| ResponseUsage {
                prompt_tokens: u.input_tokens.unwrap_or(0),
                completion_tokens: u.output_tokens.unwrap_or(0),
                total_tokens: u.input_tokens.unwrap_or(0) + u.output_tokens.unwrap_or(0),
            })
            .unwrap_or_default();

        Ok(ChatCompletionResponse {
            id: parsed.id.unwrap_or_default(),
            model: parsed.model.unwrap_or_default(),
            content,
            usage,
        })
    }
}
