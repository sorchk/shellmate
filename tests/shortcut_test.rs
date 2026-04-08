use shellmate::error::AppError;
use shellmate::shortcut::{apply_shortcut_to_script, parse_shortcut};

#[test]
fn test_parse_shortcut_ctrl_g() {
    let (bash, zsh) = parse_shortcut("Ctrl+G").unwrap();
    assert_eq!(bash, "\\C-g");
    assert_eq!(zsh, "^G");
}

#[test]
fn test_parse_shortcut_ctrl_a() {
    let (bash, zsh) = parse_shortcut("Ctrl+A").unwrap();
    assert_eq!(bash, "\\C-a");
    assert_eq!(zsh, "^A");
}

#[test]
fn test_parse_shortcut_ctrl_z() {
    let (bash, zsh) = parse_shortcut("Ctrl+Z").unwrap();
    assert_eq!(bash, "\\C-z");
    assert_eq!(zsh, "^Z");
}

#[test]
fn test_parse_shortcut_alt_x() {
    let (bash, zsh) = parse_shortcut("Alt+X").unwrap();
    assert_eq!(bash, "\\M-x");
    assert_eq!(zsh, "^[[X");
}

#[test]
fn test_parse_shortcut_alt_a() {
    let (bash, zsh) = parse_shortcut("Alt+A").unwrap();
    assert_eq!(bash, "\\M-a");
    assert_eq!(zsh, "^[[A");
}

#[test]
fn test_parse_shortcut_case_insensitive() {
    let (bash1, zsh1) = parse_shortcut("Ctrl+g").unwrap();
    let (bash2, zsh2) = parse_shortcut("Ctrl+G").unwrap();
    assert_eq!(bash1, bash2);
    assert_eq!(zsh1, zsh2);
}

#[test]
fn test_parse_shortcut_whitespace() {
    let (bash, zsh) = parse_shortcut("  Ctrl+G  ").unwrap();
    assert_eq!(bash, "\\C-g");
    assert_eq!(zsh, "^G");
}

#[test]
fn test_parse_shortcut_invalid_format() {
    let result = parse_shortcut("Super+G");
    assert!(result.is_err());
    if let Err(AppError::ConfigError(msg)) = result {
        assert!(msg.contains("Invalid shortcut format"));
    } else {
        panic!("Expected ConfigError");
    }
}

#[test]
fn test_parse_shortcut_invalid_key_multi_char() {
    let result = parse_shortcut("Ctrl+AB");
    assert!(result.is_err());
    if let Err(AppError::ConfigError(msg)) = result {
        assert!(msg.contains("Invalid shortcut key"));
    } else {
        panic!("Expected ConfigError");
    }
}

#[test]
fn test_parse_shortcut_invalid_key_number() {
    let result = parse_shortcut("Ctrl+1");
    assert!(result.is_err());
}

#[test]
fn test_parse_shortcut_empty() {
    let result = parse_shortcut("");
    assert!(result.is_err());
}

#[test]
fn test_apply_shortcut_bash_script() {
    let script = r#"bind -x '"\C-xc": _shellmate_shortcut'
bind '"__SHELLMATE_BIND_KEY_BASH__": "\C-xc\C-j'"#;
    let result = apply_shortcut_to_script(script, "\\C-x", "^X");
    assert!(result.contains(r#"bind '"\C-x": "\C-xc\C-j'"#));
    assert!(!result.contains("__SHELLMATE_BIND_KEY_BASH__"));
}

#[test]
fn test_apply_shortcut_zsh_script() {
    let script = "bindkey '__SHELLMATE_BIND_KEY_ZSH__' _shellmate_shortcut";
    let result = apply_shortcut_to_script(script, "\\C-g", "^X");
    assert!(result.contains("bindkey '^X' _shellmate_shortcut"));
    assert!(!result.contains("__SHELLMATE_BIND_KEY_ZSH__"));
}

#[test]
fn test_apply_shortcut_both_placeholders() {
    let script = r#"bind '"__SHELLMATE_BIND_KEY_BASH__": "\C-xc\C-j'"
bindkey '__SHELLMATE_BIND_KEY_ZSH__' _shellmate_shortcut"#;
    let result = apply_shortcut_to_script(script, "\\M-x", "^[[X");
    assert!(
        result.contains(r#"bind '"\M-x": "\C-xc\C-j'"#),
        "result was: {}",
        result
    );
    assert!(
        result.contains("bindkey '^[[X' _shellmate_shortcut"),
        "result was: {}",
        result
    );
}

#[test]
fn test_apply_shortcut_no_placeholder() {
    let script = "echo hello";
    let result = apply_shortcut_to_script(script, "\\C-g", "^G");
    assert_eq!(result, "echo hello");
}
