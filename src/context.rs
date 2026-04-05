use serde::Serialize;

use crate::history;

#[derive(Debug, Serialize)]
pub struct ShellContext {
    pub current_directory: String,
    pub os_type: String,
    pub shell: String,
    pub history: Vec<String>,
}

impl ShellContext {
    pub fn build(shell: &str) -> Self {
        let current_directory = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let os_type = std::env::consts::OS.to_string();

        let history = history::detect_history_path(shell)
            .and_then(|path| history::read_history(&path, shell).ok())
            .unwrap_or_default();

        ShellContext {
            current_directory,
            os_type,
            shell: shell.to_string(),
            history,
        }
    }
}
