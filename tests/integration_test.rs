use shellmate::command::{build_system_prompt, build_user_prompt, sanitize_command};
use shellmate::config::Config;
use shellmate::context::ShellContext;
use shellmate::error::AppError;
use shellmate::history::read_history;
use shellmate::security::{CheckResult, SecurityChecker};
use tempfile::TempDir;

#[test]
fn test_full_pipeline_safe_command_sanitize() {
    let raw = "```bash\nfind . -maxdepth 1000 | wc -l\n```";
    let cmd = sanitize_command(raw);
    assert_eq!(cmd, "find . -maxdepth 1000 | wc -l");

    let config = Config::default();
    let checker = SecurityChecker::new(&config.security).unwrap();
    assert!(matches!(checker.check_command(&cmd), CheckResult::Pass));
}

#[test]
fn test_full_pipeline_blocked_command_sanitize() {
    let raw = "rm -rf /";
    let cmd = sanitize_command(raw);
    assert_eq!(cmd, "rm -rf /");

    let config = Config::default();
    let checker = SecurityChecker::new(&config.security).unwrap();
    assert!(matches!(
        checker.check_command(&cmd),
        CheckResult::Blocked(_)
    ));
}

#[test]
fn test_full_pipeline_mkfs_blocked() {
    let raw = "mkfs -t ext4 /dev/sda1";
    let cmd = sanitize_command(raw);
    assert_eq!(cmd, "mkfs -t ext4 /dev/sda1");

    let config = Config::default();
    let checker = SecurityChecker::new(&config.security).unwrap();
    assert!(matches!(
        checker.check_command(&cmd),
        CheckResult::Blocked(_)
    ));
}

#[test]
fn test_config_integration() {
    let config = Config::default();
    assert_eq!(config.trigger.prefixes, vec!["@ai", "#ai", "/ai"]);
    assert_eq!(config.trigger.shortcut, "Ctrl+G");
    assert_eq!(config.llm.provider, "openai");
    assert_eq!(config.security.mode, "strict");
    assert!(!config.security.block_patterns.is_empty());
}

#[test]
fn test_context_with_history() {
    let tmpdir = TempDir::new().unwrap();
    let history_path = tmpdir.path().join(".bash_history");
    std::fs::write(
        &history_path,
        "ls -la\ncd src\ngit status\n@ai should be filtered\nnpm install\n",
    )
    .unwrap();

    let history = read_history(&history_path, "bash").unwrap();
    assert!(!history.iter().any(|h| h.contains("@ai")));
    assert!(history.contains(&"npm install".to_string()));
}

#[test]
fn test_prompt_built_correctly() {
    let sys = build_system_prompt();
    assert!(sys.contains("shell command generator"));
    assert!(sys.contains("single line"));

    let context = ShellContext {
        current_directory: "/home/user".to_string(),
        os_type: "linux".to_string(),
        shell: "bash".to_string(),
        history: vec!["ls -la".to_string(), "git status".to_string()],
    };
    let user = build_user_prompt("list all files", &context);
    assert!(user.contains("list all files"));
    assert!(user.contains("bash"));
    assert!(user.contains("/home/user"));
    assert!(user.contains("ls -la"));
    assert!(user.contains("one shell command only"));
}

#[test]
fn test_all_sanitize_patterns() {
    let cases = vec![
        ("ls -la", "ls -la"),
        ("```sh\nls\n```", "ls"),
        ("```bash\ngit status\n```", "git status"),
        ("# comment\necho hello", "echo hello"),
        ("  \n  ls -la  \n  ", "ls -la"),
        ("`ls -la`", "ls -la"),
        ("Command: find . -type f", "find . -type f"),
    ];
    for (input, expected) in cases {
        assert_eq!(
            sanitize_command(input),
            expected,
            "Failed for input: {:?}",
            input
        );
    }
}

#[test]
fn test_security_with_config_roundtrip() {
    let tmpdir = TempDir::new().unwrap();
    let config_path = tmpdir.path().join("config.yaml");
    let config = Config::default();

    let yaml = serde_yaml::to_string(&config).unwrap();
    std::fs::write(&config_path, &yaml).unwrap();

    let loaded: Config =
        serde_yaml::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
    let checker = SecurityChecker::new(&loaded.security).unwrap();

    assert!(matches!(
        checker.check_command("rm -rf /"),
        CheckResult::Blocked(_)
    ));
    assert!(matches!(checker.check_command("ls -la"), CheckResult::Pass));
}

#[test]
fn test_error_display() {
    let err = AppError::SecurityBlocked("rm -rf /".to_string());
    assert!(err.to_string().contains("BLOCKED:"));

    let err = AppError::ConfigError("bad config".to_string());
    assert!(err.to_string().contains("Config error"));

    let err = AppError::LlmError("timeout".to_string());
    assert!(err.to_string().contains("LLM error"));
}
