use crate::config::SecurityConfig;
use crate::error::AppError;

pub enum CheckResult {
    Pass,
    Blocked(String),
}

pub struct SecurityChecker {
    blocked_commands: Vec<String>,
    mode: String,
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-'
}

fn contains_word_match(command: &str, keyword: &str) -> bool {
    let mut start = 0;
    while let Some(pos) = command[start..].find(keyword) {
        let match_start = start + pos;
        let match_end = match_start + keyword.len();

        if match_start > 0 {
            let before = command.as_bytes()[match_start - 1] as char;
            if is_word_char(before) {
                start = match_start + 1;
                continue;
            }
        }

        if match_end < command.len() {
            let after = command.as_bytes()[match_end] as char;
            if is_word_char(after) {
                start = match_start + 1;
                continue;
            }
        }

        return true;
    }
    false
}

impl SecurityChecker {
    pub fn new(config: &SecurityConfig) -> Result<Self, AppError> {
        let blocked_commands = config
            .block_patterns
            .iter()
            .map(|s| s.trim().to_lowercase())
            .collect();
        Ok(Self {
            blocked_commands,
            mode: config.mode.clone(),
        })
    }

    pub fn check_command(&self, command: &str) -> CheckResult {
        let normalized = command.trim().to_lowercase();
        for keyword in &self.blocked_commands {
            if contains_word_match(&normalized, keyword) {
                return CheckResult::Blocked(command.to_string());
            }
        }
        CheckResult::Pass
    }

    pub fn mode(&self) -> &str {
        &self.mode
    }
}
