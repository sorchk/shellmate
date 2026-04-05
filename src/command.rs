use crate::context::ShellContext;
use crate::error::AppError;
use crate::llm::types::*;
use crate::llm::LlmProvider;

pub struct CommandGenerator {
    provider: Box<dyn LlmProvider>,
    max_tokens: Option<u32>,
}

pub struct GeneratedCommand {
    pub command: String,
    pub model: String,
    pub provider_name: String,
    pub usage: ResponseUsage,
}

impl CommandGenerator {
    pub fn new(provider: Box<dyn LlmProvider>, max_tokens: Option<u32>) -> Self {
        Self {
            provider,
            max_tokens,
        }
    }

    pub fn generate(
        &self,
        input: &str,
        context: &ShellContext,
    ) -> Result<GeneratedCommand, AppError> {
        if input.trim().is_empty() {
            return Err(AppError::LlmError("input is required".to_string()));
        }

        let messages = vec![
            ChatMessage::system(build_system_prompt()),
            ChatMessage::user(build_user_prompt(input, context)),
        ];

        let req = ChatCompletionRequest {
            messages,
            max_tokens: self.max_tokens,
            temperature: None,
        };

        let resp = self.provider.chat_completion(req)?;

        let command = sanitize_command(&resp.content);
        if command.is_empty() {
            return Err(AppError::LlmError(
                "model returned empty command".to_string(),
            ));
        }

        Ok(GeneratedCommand {
            command,
            model: resp.model.clone(),
            provider_name: provider_name_from_model(&resp.model),
            usage: resp.usage,
        })
    }
}

pub fn build_system_prompt() -> String {
    [
        "You are a shell command generator.",
        "Return a command that can be directly executed in the user’s shell, with no spaces before or after.",
        "Do not include code blocks, explanations, comments, or backticks.",
        "If multiple commands are required, join them with shell operators in a single line.",
        "Prefer safe, non-destructive commands unless the user explicitly asks for destructive behavior.",
        "Preserve the user's language when filenames or arguments are ambiguous, but output only the command.",
    ]
    .join("\n")
}

pub fn build_user_prompt(input: &str, context: &ShellContext) -> String {
    let mut sections = Vec::new();
    sections.push(format!("Task:\n{}", input.trim()));

    let mut env = Vec::new();
    let shell = context.shell.trim();
    if !shell.is_empty() {
        env.push(format!("Shell: {}", shell));
    }
    let wd = context.current_directory.trim();
    if !wd.is_empty() {
        env.push(format!("Working directory: {}", wd));
    }
    let os = context.os_type.trim();
    if !os.is_empty() {
        env.push(format!("Operating system: {}", os));
    }
    if !env.is_empty() {
        sections.push(format!("Environment:\n{}", env.join("\n")));
    }

    if let Some(block) = format_bullet_block_checked(&context.history) {
        sections.push(format!("Recent commands:\n{}", block));
    }

    sections.push("Output requirement:\nReturn one shell command only.".to_string());

    sections.join("\n\n")
}

pub fn sanitize_command(raw: &str) -> String {
    let mut command = raw.trim().to_string();
    if command.is_empty() {
        return String::new();
    }

    for prefix in &["```shell", "```bash", "```zsh", "```sh", "```"] {
        if command.starts_with(prefix) {
            command = command[prefix.len()..].to_string();
            break;
        }
    }

    if command.ends_with("```") {
        command = command[..command.len() - 3].to_string();
    }

    command = command.trim().to_string();

    for line in command.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        let mut processed = line;
        if line.to_lowercase().starts_with("command:") {
            processed = line["command:".len()..].trim();
        }
        let result = processed.trim_matches(|c| c == '`' || c == ' ');
        if !result.is_empty() {
            return result.to_string();
        }
    }

    String::new()
}

pub fn provider_name_from_model(model: &str) -> String {
    let model = model.trim();
    if model.is_empty() {
        return String::new();
    }
    if let Some(idx) = model.find('/') {
        model[..idx].to_string()
    } else {
        String::new()
    }
}

pub fn format_bullet_block(values: &[String]) -> String {
    let lines: Vec<String> = values
        .iter()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| format!("- {}", v))
        .collect();
    lines.join("\n")
}

fn format_bullet_block_checked(values: &[String]) -> Option<String> {
    let result = format_bullet_block(values);
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
