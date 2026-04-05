use shellmate::command::*;

fn make_context(
    shell: &str,
    dir: &str,
    os: &str,
    history: Vec<&str>,
) -> shellmate::context::ShellContext {
    shellmate::context::ShellContext {
        shell: shell.to_string(),
        current_directory: dir.to_string(),
        os_type: os.to_string(),
        history: history.iter().map(|s| s.to_string()).collect(),
    }
}

#[test]
fn test_sanitize_removes_code_fences() {
    assert_eq!(sanitize_command("```sh\nls -la\n```"), "ls -la");
}

#[test]
fn test_sanitize_removes_bash_fences() {
    assert_eq!(sanitize_command("```bash\nls\n```"), "ls");
}

#[test]
fn test_sanitize_plain_command() {
    assert_eq!(sanitize_command("ls -la"), "ls -la");
}

#[test]
fn test_sanitize_extracts_first_non_comment() {
    assert_eq!(sanitize_command("# comment\nls -la\necho done"), "ls -la");
}

#[test]
fn test_sanitize_strips_command_prefix() {
    assert_eq!(sanitize_command("Command: ls -la"), "ls -la");
}

#[test]
fn test_sanitize_empty_input() {
    assert_eq!(sanitize_command(""), "");
}

#[test]
fn test_sanitize_whitespace_only() {
    assert_eq!(sanitize_command("   \n  "), "");
}

#[test]
fn test_sanitize_backtick_wrapped() {
    assert_eq!(sanitize_command("`ls -la`"), "ls -la");
}

#[test]
fn test_sanitize_zsh_fences() {
    assert_eq!(sanitize_command("```zsh\necho hello\n```"), "echo hello");
}

#[test]
fn test_sanitize_shell_fences() {
    assert_eq!(sanitize_command("```shell\npwd\n```"), "pwd");
}

#[test]
fn test_sanitize_bare_fences() {
    assert_eq!(sanitize_command("```\ngit status\n```"), "git status");
}

#[test]
fn test_build_system_prompt() {
    let prompt = build_system_prompt();
    assert!(prompt.contains("shell command generator"));
    assert!(prompt.contains("exactly one command"));
    assert!(prompt.contains("non-destructive"));
    assert!(prompt.contains("markdown"));
}

#[test]
fn test_build_user_prompt_with_context() {
    let ctx = make_context("bash", "/home/user", "linux", vec!["ls", "cd /tmp"]);
    let prompt = build_user_prompt("list all files", &ctx);
    assert!(prompt.contains("Task:\nlist all files"));
    assert!(prompt.contains("Shell: bash"));
    assert!(prompt.contains("Working directory: /home/user"));
    assert!(prompt.contains("Operating system: linux"));
    assert!(prompt.contains("Recent commands:"));
    assert!(prompt.contains("- ls"));
    assert!(prompt.contains("- cd /tmp"));
    assert!(prompt.contains("Output requirement:"));
}

#[test]
fn test_build_user_prompt_minimal() {
    let ctx = make_context("", "", "", vec![]);
    let prompt = build_user_prompt("list files", &ctx);
    assert!(prompt.contains("Task:\nlist files"));
    assert!(prompt.contains("Output requirement:"));
    assert!(!prompt.contains("Environment:"));
    assert!(!prompt.contains("Recent commands:"));
}

#[test]
fn test_provider_name_from_model() {
    assert_eq!(provider_name_from_model("ollama/qwen3.5:4b"), "ollama");
    assert_eq!(provider_name_from_model("gpt-4"), "");
    assert_eq!(provider_name_from_model(""), "");
    assert_eq!(provider_name_from_model("openai/gpt-4o-mini"), "openai");
}

#[test]
fn test_format_bullet_block() {
    let values = vec!["ls".to_string(), "cd /tmp".to_string()];
    let result = format_bullet_block(&values);
    assert_eq!(result, "- ls\n- cd /tmp");
}

#[test]
fn test_format_bullet_block_skips_empty() {
    let values = vec![
        "ls".to_string(),
        "".to_string(),
        "  ".to_string(),
        "pwd".to_string(),
    ];
    let result = format_bullet_block(&values);
    assert_eq!(result, "- ls\n- pwd");
}

#[test]
fn test_format_bullet_block_empty() {
    let values: Vec<String> = vec![];
    let result = format_bullet_block(&values);
    assert_eq!(result, "");
}
