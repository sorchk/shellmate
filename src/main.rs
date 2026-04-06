use clap::Parser;
use std::io::{self, Write};
use std::process;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        None => {
            cli::Cli::parse_from(["shellmate", "--help"]);
        }
        Some(cli::Commands::Generate { prompt, shell }) => {
            cmd_generate(&prompt, &shell);
        }
        Some(cli::Commands::Check { command }) => {
            cmd_check(&command);
        }
        Some(cli::Commands::Config) => {
            cmd_config();
        }
        Some(cli::Commands::Install { shell, config_only }) => {
            cmd_install(&shell, config_only);
        }
    }
}

fn cmd_generate(prompt: &str, shell: &str) {
    let config = shellmate::config::Config::load_or_default();
    let engine = match shellmate::core::CoreEngine::new(&config) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    match engine.process(prompt, shell) {
        shellmate::core::ProcessResult::Command(cmd) => {
            print!("{}", cmd);
        }
        shellmate::core::ProcessResult::Blocked(_cmd) => {
            process::exit(2);
        }
        shellmate::core::ProcessResult::Error(msg) => {
            eprintln!("Error: {}", msg);
            process::exit(1);
        }
    }
}

fn cmd_check(command: &str) {
    let config = shellmate::config::Config::load_or_default();
    let checker = match shellmate::security::SecurityChecker::new(&config.security) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    match checker.check_command(command) {
        shellmate::security::CheckResult::Pass => {
            println!("PASS: command is safe");
        }
        shellmate::security::CheckResult::Blocked(cmd) => {
            println!("BLOCKED: {}", cmd);
            process::exit(2);
        }
    }
}

fn cmd_config() {
    let config = shellmate::config::Config::load_or_default();
    match serde_yaml::to_string(&config) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => {
            eprintln!("Error serializing config: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_install(shell: &str, config_only: bool) {
    let shell_type = if shell == "auto" {
        detect_shell()
    } else {
        shell.to_string()
    };

    if config_only {
        configure_ai();
        return;
    }

    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            eprintln!("Error: cannot detect home directory");
            process::exit(1);
        }
    };

    let is_macos = std::env::consts::OS == "macos";

    let target_rc = match shell_type.as_str() {
        "bash" => {
            if is_macos {
                home.join(".bash_profile")
            } else {
                home.join(".bashrc")
            }
        }
        "zsh" => home.join(".zshrc"),
        "sh" => home.join(".profile"),
        "fish" => home.join(".config/fish/config.fish"),
        _ => {
            eprintln!(
                "Error: unsupported shell '{}'. Use bash, zsh, sh, or fish.",
                shell_type
            );
            process::exit(1);
        }
    };

    let integration_file = std::env::current_exe()
        .ok()
        .and_then(|exe| {
            exe.parent().map(|p| {
                p.join("..")
                    .join("..")
                    .join("shell")
                    .join(format!("shellmate.{}", shell_type))
            })
        })
        .unwrap_or_else(|| std::path::PathBuf::from(format!("shell/shellmate.{}", shell_type)));

    println!();
    println!("Shell: {}", shell_type);
    println!("RC file: {}", target_rc.display());
    println!("Integration file: {}", integration_file.display());
    println!();

    let config_dir = home.join(".shellmate");
    let config_path = config_dir.join("config.yaml");

    if !config_path.exists() {
        let config = shellmate::config::Config::default();
        match config.save() {
            Ok(()) => println!("Created default config at {}", config_path.display()),
            Err(e) => eprintln!("Warning: could not create config: {}", e),
        }
        println!();
        configure_ai();
    } else {
        println!("Config already exists at {}", config_path.display());
        match shellmate::config::Config::load() {
            Ok(config) => {
                if !config.llm.is_configured() {
                    println!("AI provider not configured yet.");
                    println!();
                    configure_ai();
                } else if prompt_yes_no("AI already configured, modify?", false) {
                    println!();
                    configure_ai();
                }
            }
            Err(_) => {
                println!("Could not load config, skipping AI configuration.");
            }
        }
    }

    println!();
    println!(
        "To complete installation, add this line to {}:",
        target_rc.display()
    );
    println!("  source {}", integration_file.display());
    println!();
}

fn configure_ai() {
    println!("--- AI Provider Configuration ---");
    println!();

    let provider = prompt_select(
        "请选择 AI Provider:",
        &[
            ("1", "OpenAI"),
            ("2", "Ollama"),
            ("3", "Anthropic"),
            ("4", "Gemini"),
            ("5", "Custom (自定义)"),
        ],
    );

    let (provider_name, default_base_url, default_model, default_api_type) = match provider.as_str()
    {
        "1" => (
            "openai".to_string(),
            Some("https://api.openai.com"),
            Some("gpt-4-turbo"),
            None,
        ),
        "2" => (
            "ollama".to_string(),
            Some("http://localhost:11434"),
            Some("qwen3.5:4b"),
            None,
        ),
        "3" => (
            "anthropic".to_string(),
            Some("https://api.anthropic.com"),
            Some("claude-3-sonnet-20240229"),
            None,
        ),
        "4" => (
            "gemini".to_string(),
            Some("https://generativelanguage.googleapis.com"),
            Some("gemini-pro"),
            None,
        ),
        _ => {
            let api_type = prompt_select(
                "请选择 API 类型:",
                &[
                    ("1", "openai-completions"),
                    ("2", "openai-responses"),
                    ("3", "anthropic-messages"),
                    ("4", "gemini-generate-content"),
                    ("5", "custom (输入完整 endpoint 地址)"),
                ],
            );
            let api_type_str = match api_type.as_str() {
                "1" => "openai-completions",
                "2" => "openai-responses",
                "3" => "anthropic-messages",
                "4" => "gemini-generate-content",
                _ => "custom",
            };
            let name = prompt_input("请输入 Provider 名称:", None);
            (name, None, None, Some(api_type_str.to_string()))
        }
    };

    let is_custom_api = default_api_type.as_deref() == Some("custom");
    let base_url = if is_custom_api {
        prompt_input("请输入完整 endpoint URL:", None)
    } else {
        prompt_input("请输入 Base URL", default_base_url)
    };

    let api_key_input = prompt_input("请输入 API Key (直接回车跳过)", None);
    let api_key = if api_key_input.trim().is_empty() {
        None
    } else {
        Some(api_key_input.trim().to_string())
    };

    let model = prompt_input("请输入 Model 名称", default_model);

    let api_type_value = match provider.as_str() {
        "1" | "2" => None,
        "3" => None,
        "4" => None,
        _ => default_api_type,
    };

    println!();
    println!("配置确认:");
    println!("  Provider:  {}", provider_name);
    if let Some(ref t) = api_type_value {
        println!("  API Type:  {}", t);
    }
    println!("  Base URL:  {}", base_url);
    println!("  Model:     {}", model);
    if let Some(ref k) = api_key {
        if k.len() > 8 {
            println!("  API Key:   {}...{}", &k[..4], &k[k.len() - 4..]);
        } else {
            println!("  API Key:   ****");
        }
    } else {
        println!("  API Key:   (not set)");
    }
    println!();

    if !prompt_yes_no("确认保存？", true) {
        println!("配置已取消。");
        return;
    }

    let mut config = shellmate::config::Config::load_or_default();
    config.llm.provider = provider_name;
    config.llm.base_url = Some(base_url);
    config.llm.api_key = api_key;
    config.llm.model = model;
    config.llm.api_type = api_type_value;

    match config.save() {
        Ok(()) => println!("Configuration saved."),
        Err(e) => eprintln!("Error saving config: {}", e),
    }
}

fn prompt_input(message: &str, default: Option<&str>) -> String {
    let display_default = default.unwrap_or("");
    if display_default.is_empty() {
        print!("{}: ", message);
    } else {
        print!("{} (默认: {}): ", message, display_default);
    }
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        display_default.to_string()
    } else {
        trimmed
    }
}

fn prompt_select(message: &str, options: &[(&str, &str)]) -> String {
    println!("{}", message);
    for (key, label) in options {
        println!("  {}) {}", key, label);
    }
    loop {
        print!("请输入选项 (1-{}): ", options.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();
        for (key, _) in options {
            if choice == *key {
                println!();
                return choice.to_string();
            }
        }
        println!("Invalid option, please try again.");
    }
}

fn prompt_yes_no(message: &str, default_yes: bool) -> bool {
    let hint = if default_yes { "(Y/n)" } else { "(y/N)" };
    print!("{} {}: ", message, hint);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let answer = input.trim().to_lowercase();
    if answer.is_empty() {
        default_yes
    } else {
        answer == "y" || answer == "yes"
    }
}

fn detect_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_default()
        .rsplit('/')
        .next()
        .unwrap_or("bash")
        .to_string()
}
