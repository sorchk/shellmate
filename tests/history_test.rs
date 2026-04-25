use std::fs;
use tempfile::NamedTempFile;

use shellmate::history::{
    detect_history_path, is_trigger_command, parse_bash_history, parse_zsh_history, read_history,
};

#[test]
fn test_parse_bash_history_basic() {
    let content = "ls -la\ncd /tmp\npwd\n";
    let result = parse_bash_history(content);
    assert_eq!(result, vec!["ls -la", "cd /tmp", "pwd"]);
}

#[test]
fn test_parse_bash_history_filters_prefixes() {
    let content = "ls -la\n@ai fix this\necho hello\npwd";
    let result = parse_bash_history(content);
    let filtered: Vec<String> = result
        .into_iter()
        .filter(|line| !is_trigger_command(line))
        .collect();
    assert_eq!(filtered, vec!["ls -la", "echo hello", "pwd"]);
}

#[test]
fn test_parse_bash_history_limits_to_8() {
    let lines: Vec<String> = (1..=15).map(|i| format!("cmd{}", i)).collect();
    let content = lines.join("\n");
    let path = NamedTempFile::new().unwrap();
    fs::write(path.path(), &content).unwrap();
    let result = read_history(path.path(), "bash").unwrap();
    assert_eq!(result.len(), 8);
    assert_eq!(result[0], "cmd15");
    assert_eq!(result[7], "cmd8");
}

#[test]
fn test_parse_bash_history_empty_lines_filtered() {
    let content = "ls\n\n   \npwd\n\n";
    let path = NamedTempFile::new().unwrap();
    fs::write(path.path(), content).unwrap();
    let result = read_history(path.path(), "bash").unwrap();
    assert_eq!(result, vec!["pwd", "ls"]);
}

#[test]
fn test_parse_zsh_history_timestamps() {
    let content = ": 1234567890:0;ls -la";
    let result = parse_zsh_history(content);
    assert_eq!(result, vec!["ls -la"]);
}

#[test]
fn test_parse_zsh_history_mixed() {
    let content = ": 1234567890:0;ls -la\nplain_command\n: 9999999999:0;pwd";
    let result = parse_zsh_history(content);
    assert_eq!(result, vec!["ls -la", "plain_command", "pwd"]);
}

#[test]
fn test_is_trigger_command() {
    assert!(is_trigger_command("@ai fix this"));
    assert!(!is_trigger_command("#ai help"));
    assert!(!is_trigger_command("/ai do stuff"));
    assert!(!is_trigger_command("ls -la"));
    assert!(!is_trigger_command("git commit"));
    assert!(!is_trigger_command("echo @ai is cool"));
}

#[test]
fn test_read_history_from_file() {
    let path = NamedTempFile::new().unwrap();
    fs::write(path.path(), "ls -la\ncd /tmp\npwd\n").unwrap();
    let result = read_history(path.path(), "bash").unwrap();
    assert_eq!(result, vec!["pwd", "cd /tmp", "ls -la"]);
}

#[test]
fn test_detect_history_path() {
    let bash_path = detect_history_path("bash").unwrap();
    assert!(bash_path.to_string_lossy().ends_with(".bash_history"));

    let zsh_path = detect_history_path("zsh").unwrap();
    assert!(zsh_path.to_string_lossy().ends_with(".zsh_history"));

    let fish_path = detect_history_path("fish").unwrap();
    assert!(fish_path.to_string_lossy().ends_with("fish_history"));

    let unknown_path = detect_history_path("unknown").unwrap();
    assert!(unknown_path.to_string_lossy().ends_with(".bash_history"));
}
