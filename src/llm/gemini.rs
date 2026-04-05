use super::types::*;
use super::LlmProvider;
use crate::config::LlmConfig;
use crate::error::AppError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub struct GeminiProvider {
    base_url: String,
    api_key: String,
    model: String,
    max_tokens: Option<u32>,
    client: Client,
}

#[derive(Serialize)]
struct GeminiRequestBody {
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GeminiGenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    #[serde(rename = "modelVersion")]
    model_version: Option<String>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsage>,
}

#[derive(Deserialize)]
struct Candidate {
    content: GeminiContentResponse,
}

#[derive(Deserialize)]
struct GeminiContentResponse {
    parts: Vec<GeminiPartResponse>,
}

#[derive(Deserialize)]
struct GeminiPartResponse {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiUsage {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: Option<u32>,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: Option<u32>,
    #[serde(rename = "totalTokenCount")]
    total_token_count: Option<u32>,
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
        None => "https://generativelanguage.googleapis.com".to_string(),
        Some(url) => url.trim_end_matches('/').to_string(),
    }
}

fn ends_with_version(base_url: &str) -> bool {
    let path = base_url.split("://").nth(1).unwrap_or(base_url);
    let last = path.rsplit('/').next().unwrap_or("");
    let rest = last.strip_prefix('v').unwrap_or("");
    !rest.is_empty() && rest.chars().next().is_some_and(|c| c.is_ascii_digit())
}

fn build_gemini_url(base_url: &str, model: &str) -> String {
    if base_url.contains("/models/") && base_url.ends_with(":generateContent") {
        return base_url.to_string();
    }
    if ends_with_version(base_url) {
        return format!("{}/models/{}:generateContent", base_url, model);
    }
    format!("{}/v1beta/models/{}:generateContent", base_url, model)
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

impl GeminiProvider {
    pub fn new(config: &LlmConfig) -> Result<Self, AppError> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| AppError::LlmError("Gemini API key is required".into()))?;

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

impl LlmProvider for GeminiProvider {
    fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        let url = build_gemini_url(&self.base_url, &self.model);

        let contents: Vec<GeminiContent> = req
            .messages
            .into_iter()
            .map(|msg| {
                let role = if msg.role == "assistant" {
                    "model".to_string()
                } else {
                    "user".to_string()
                };
                GeminiContent {
                    role,
                    parts: vec![GeminiPart { text: msg.content }],
                }
            })
            .collect();

        let max_tokens = req.max_tokens.or(self.max_tokens).unwrap_or(1024);

        let body = GeminiRequestBody {
            contents,
            generation_config: GeminiGenerationConfig {
                max_output_tokens: max_tokens,
                temperature: req.temperature,
            },
        };

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-goog-api-key", &self.api_key)
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

        let parsed: GeminiResponse = serde_json::from_str(&resp_body)
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        let content = parsed
            .candidates
            .and_then(|c| c.into_iter().next())
            .and_then(|c| c.content.parts.into_iter().next())
            .and_then(|p| p.text)
            .unwrap_or_default();

        let usage = parsed
            .usage_metadata
            .map(|u| ResponseUsage {
                prompt_tokens: u.prompt_token_count.unwrap_or(0),
                completion_tokens: u.candidates_token_count.unwrap_or(0),
                total_tokens: u.total_token_count.unwrap_or(0),
            })
            .unwrap_or_default();

        Ok(ChatCompletionResponse {
            id: String::new(),
            model: parsed.model_version.unwrap_or_default(),
            content,
            usage,
        })
    }
}
