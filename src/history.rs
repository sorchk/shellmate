use std::path::{Path, PathBuf};

use crate::error::AppError;

const PREFIXES: &[&str] = &["@ai", "#ai", "/ai"];
const MAX_ENTRIES: usize = 8;

pub fn read_history(history_path: &Path, shell: &str) -> Result<Vec<String>, AppError> {
    let content = std::fs::read_to_string(history_path)
        .map_err(|e| AppError::HistoryError(format!("Failed to read history file: {}", e)))?;

    let entries = match shell {
        "zsh" => parse_zsh_history(&content),
        _ => parse_bash_history(&content),
    };

    let filtered: Vec<String> = entries
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| !is_trigger_command(line))
        .collect();

    let last_entries: Vec<String> = filtered.into_iter().rev().take(MAX_ENTRIES).collect();

    Ok(last_entries)
}

pub fn detect_history_path(shell: &str) -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let path = match shell {
        "bash" => home.join(".bash_history"),
        "zsh" => home.join(".zsh_history"),
        "sh" => home.join(".sh_history"),
        "fish" => home.join(".local/share/fish/fish_history"),
        _ => home.join(".bash_history"),
    };
    Some(path)
}

pub fn parse_bash_history(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

pub fn parse_zsh_history(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with(':') {
                if let Some(idx) = trimmed.find(';') {
                    return trimmed[idx + 1..].trim().to_string();
                }
            }
            trimmed.to_string()
        })
        .collect()
}

pub fn is_trigger_command(line: &str) -> bool {
    let trimmed = line.trim();
    PREFIXES.iter().any(|prefix| trimmed.starts_with(prefix))
}
