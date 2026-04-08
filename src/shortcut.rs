use crate::error::AppError;

pub fn parse_shortcut(shortcut: &str) -> Result<(String, String), AppError> {
    let shortcut = shortcut.trim();
    let (modifier, key) = if let Some(key) = shortcut.strip_prefix("Ctrl+") {
        ("ctrl", key)
    } else if let Some(key) = shortcut.strip_prefix("Alt+") {
        ("alt", key)
    } else {
        return Err(AppError::ConfigError(format!(
            "Invalid shortcut format '{}'. Expected 'Ctrl+<Key>' or 'Alt+<Key>' (e.g. Ctrl+G, Alt+X)",
            shortcut
        )));
    };

    if key.len() != 1 || !key.chars().next().unwrap().is_ascii_alphabetic() {
        return Err(AppError::ConfigError(format!(
            "Invalid shortcut key '{}'. Expected a single letter (A-Z)",
            key
        )));
    }

    let key_lower = key.to_ascii_lowercase();
    let key_upper = key.to_ascii_uppercase();

    let (bash_key, zsh_key) = match modifier {
        "ctrl" => (format!("\\C-{}", key_lower), format!("^{}", key_upper)),
        "alt" => (format!("\\e{}", key_lower), format!("^[{}", key_upper)),
        _ => unreachable!(),
    };

    Ok((bash_key, zsh_key))
}

pub fn apply_shortcut_to_script(content: &str, bash_key: &str, zsh_key: &str) -> String {
    content
        .replace("__SHELLMATE_BIND_KEY_BASH__", bash_key)
        .replace("__SHELLMATE_BIND_KEY_ZSH__", zsh_key)
}
