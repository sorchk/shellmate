use super::types::*;
use super::LlmProvider;
use crate::config::LlmConfig;
use crate::error::AppError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAiProvider {
    base_url: String,
    api_key: Option<String>,
    model: String,
    max_tokens: Option<u32>,
    api_type: String,
    client: Client,
}

#[derive(Serialize)]
struct ChatCompletionsRequest {
    model: String,
    messages: Vec<ChatCompletionsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Serialize)]
struct ChatCompletionsMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    id: Option<String>,
    model: Option<String>,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: Option<MessageContent>,
    text: Option<String>,
}

#[derive(Deserialize)]
struct MessageContent {
    content: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: Option<ErrorDetail>,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: Option<String>,
}

#[derive(Serialize)]
struct ResponsesRequest {
    model: String,
    input: Vec<ResponsesInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
}

#[derive(Serialize)]
struct ResponsesInput {
    role: String,
    content: Vec<ResponsesInputContent>,
}

#[derive(Serialize)]
struct ResponsesInputContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize)]
struct ResponsesApiResponse {
    id: Option<String>,
    model: Option<String>,
    output_text: Option<String>,
    output: Option<Vec<ResponsesOutput>>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct ResponsesOutput {
    content: Option<Vec<ResponsesOutputContent>>,
}

#[derive(Deserialize)]
struct ResponsesOutputContent {
    text: Option<String>,
}

fn normalize_base_url(provider: &str, base_url: &Option<String>) -> String {
    match (provider, base_url) {
        ("ollama", None) => "http://localhost:11434".to_string(),
        (_, None) => "https://api.openai.com".to_string(),
        (_, Some(url)) => url.trim_end_matches('/').to_string(),
    }
}

fn normalize_model_id(model: &str) -> String {
    if let Some(pos) = model.rfind('/') {
        model[pos + 1..].to_string()
    } else {
        model.to_string()
    }
}

fn ends_with_version(base_url: &str) -> bool {
    let path = base_url.split("://").nth(1).unwrap_or(base_url);
    let last = path.rsplit('/').next().unwrap_or("");
    let rest = last.strip_prefix('v').unwrap_or("");
    !rest.is_empty() && rest.chars().next().is_some_and(|c| c.is_ascii_digit())
}

fn build_chat_completions_url(base_url: &str) -> String {
    if base_url.ends_with("/chat/completions") {
        return base_url.to_string();
    }
    if ends_with_version(base_url) {
        return format!("{}/chat/completions", base_url);
    }
    format!("{}/v1/chat/completions", base_url)
}

fn build_responses_url(base_url: &str) -> String {
    if base_url.ends_with("/responses") {
        return base_url.to_string();
    }
    if ends_with_version(base_url) {
        return format!("{}/responses", base_url);
    }
    format!("{}/v1/responses", base_url)
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

impl OpenAiProvider {
    pub fn new(config: &LlmConfig) -> Result<Self, AppError> {
        let base_url = normalize_base_url(&config.provider, &config.base_url);
        let model = normalize_model_id(&config.model);
        let api_type = config
            .api_type
            .clone()
            .unwrap_or_else(|| "openai-completions".to_string());

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .map_err(|e| AppError::LlmError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            base_url,
            api_key: config.api_key.clone(),
            model,
            max_tokens: config.max_tokens,
            api_type,
            client,
        })
    }

    fn chat_completions(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        let url = build_chat_completions_url(&self.base_url);

        let messages: Vec<ChatCompletionsMessage> = req
            .messages
            .into_iter()
            .map(|m| ChatCompletionsMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        let body = ChatCompletionsRequest {
            model: self.model.clone(),
            messages,
            max_tokens: req.max_tokens.or(self.max_tokens),
            temperature: req.temperature,
        };

        let mut request_builder = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body);

        if let Some(ref api_key) = self.api_key {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let resp = request_builder
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

        let parsed: OpenAiResponse = serde_json::from_str(&resp_body)
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        let content = parsed
            .choices
            .first()
            .and_then(|c| c.message.as_ref())
            .and_then(|m| m.content.clone())
            .or_else(|| parsed.choices.first().and_then(|c| c.text.clone()))
            .unwrap_or_default();

        let usage = parsed
            .usage
            .map(|u| ResponseUsage {
                prompt_tokens: u.prompt_tokens.unwrap_or(0),
                completion_tokens: u.completion_tokens.unwrap_or(0),
                total_tokens: u.total_tokens.unwrap_or(0),
            })
            .unwrap_or_default();

        Ok(ChatCompletionResponse {
            id: parsed.id.unwrap_or_default(),
            model: parsed.model.unwrap_or_default(),
            content,
            usage,
        })
    }

    fn responses(&self, req: ChatCompletionRequest) -> Result<ChatCompletionResponse, AppError> {
        let url = build_responses_url(&self.base_url);

        let input: Vec<ResponsesInput> = req
            .messages
            .into_iter()
            .map(|m| ResponsesInput {
                role: m.role,
                content: vec![ResponsesInputContent {
                    content_type: "input_text".to_string(),
                    text: m.content,
                }],
            })
            .collect();

        let body = ResponsesRequest {
            model: self.model.clone(),
            input,
            max_output_tokens: req.max_tokens.or(self.max_tokens),
        };

        let mut request_builder = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body);

        if let Some(ref api_key) = self.api_key {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let resp = request_builder
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

        let parsed: ResponsesApiResponse = serde_json::from_str(&resp_body)
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        let content = parsed
            .output_text
            .or_else(|| {
                parsed
                    .output
                    .as_ref()?
                    .first()?
                    .content
                    .as_ref()?
                    .first()?
                    .text
                    .clone()
            })
            .unwrap_or_default();

        let usage = parsed
            .usage
            .map(|u| ResponseUsage {
                prompt_tokens: u.prompt_tokens.unwrap_or(0),
                completion_tokens: u.completion_tokens.unwrap_or(0),
                total_tokens: u.total_tokens.unwrap_or(0),
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

impl LlmProvider for OpenAiProvider {
    fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        match self.api_type.as_str() {
            "openai-responses" => self.responses(req),
            _ => self.chat_completions(req),
        }
    }
}
