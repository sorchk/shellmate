[English](../../en/DEVELOPMENT.md) | [中文](../../zh/DEVELOPMENT.md) | [← README](../../README.md)

---

# ShellMate Development Guide

## 1. Project Overview

ShellMate is an AI-powered shell command assistant written in Rust. It generates shell commands from natural language descriptions and blocks dangerous commands through a security checking mechanism.

**Core Features:**

- Natural language → Shell command translation
- Multi AI Provider support (OpenAI / Ollama / Anthropic / Gemini / Custom)
- Keyword-based command security checking (blocks `rm`, `mkfs`, `dd`, etc.)
- Bash / Zsh / Sh / Fish shell integration
- Trigger prefix (`@ai`, `#ai`, `/ai`) and keyboard shortcut (`Ctrl+G`) support

**Tech Stack:**

- Rust 2021 Edition
- `clap` — CLI argument parsing
- `reqwest` (blocking, rustls-tls) — HTTP requests
- `serde` + `serde_yaml` + `serde_json` — Serialization/Deserialization
- `chrono` — Time handling
- `dirs` — System directory detection
- `tempfile` (dev) — Test temporary files

---

## 2. Environment Setup

### 2.1 Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify installation:

```bash
rustc --version    # Requires 1.56+ (Rust 2021 Edition)
cargo --version
```

### 2.2 System Dependencies

- **Linux**: Needs `build-essential`

  ```bash
  # Debian/Ubuntu
  sudo apt update && sudo apt install build-essential

  # Fedora/RHEL
  sudo dnf install gcc
  ```

- **macOS**: Install Xcode Command Line Tools

  ```bash
  xcode-select --install
  ```

---

## 3. Project Structure

```
shell-mate/
├── Cargo.toml              # Project manifest and dependencies
├── Cargo.lock              # Dependency version lock
├── AGENTS.md               # AI-assisted development guide
├── README.md               # Project documentation (bilingual)
├── CHANGELOG.md            # Version changelog
├── LICENSE                 # Apache 2.0 License
├── build.sh                # CI build script
├── src/
│   ├── main.rs             # CLI entry point (clap parsing, command dispatch)
│   ├── lib.rs              # Library root — re-exports all modules
│   ├── cli.rs              # Clap CLI definitions (Commands enum)
│   ├── config.rs           # YAML config loading/saving, Config structs (serde)
│   ├── core.rs             # CoreEngine — orchestrates generate → security check → result
│   ├── command.rs          # CommandGenerator — builds prompts, sanitizes LLM output
│   ├── context.rs          # ShellContext — gathers shell env, OS, cwd, history
│   ├── security.rs         # SecurityChecker — keyword-based command blocklist
│   ├── history.rs          # Shell history file parsing (bash/zsh)
│   ├── ui.rs               # Terminal UI — status messages to stderr
│   ├── error.rs            # AppError enum (custom error type)
│   └── llm/
│       ├── mod.rs          # LlmProvider trait + factory function create_provider()
│       ├── types.rs        # ChatMessage, ChatCompletionRequest/Response, ResponseUsage
│       ├── openai.rs       # OpenAI/Ollama provider (chat completions + responses API)
│       ├── anthropic.rs    # Anthropic provider
│       └── gemini.rs       # Google Gemini provider
├── tests/                  # Integration tests (one file per module)
│   ├── command_test.rs
│   ├── config_test.rs
│   ├── context_test.rs
│   ├── history_test.rs
│   ├── integration_test.rs
│   └── security_test.rs
├── shell/                  # Shell integration scripts
│   ├── install.sh          # One-click install script
│   ├── remote-install.sh   # Remote install via curl
│   ├── shellmate.bash      # Bash integration
│   ├── shellmate.zsh       # Zsh integration
│   └── shellmate.sh        # POSIX sh integration
├── docs/                   # Documentation
│   ├── en/                 # English docs
│   └── zh/                 # Chinese docs
└── .github/
    └── workflows/          # CI/CD workflows
```

---

## 4. Build & Run

### 4.1 Debug Build

```bash
cargo build
```

Binary output: `target/debug/shellmate`.

### 4.2 Release Build

```bash
cargo build --release
```

Binary output: `target/release/shellmate`, with optimizations enabled.

### 4.3 Run Directly

```bash
# Generate command
cargo run -- generate "list all files" --shell bash

# Security check
cargo run -- check "rm -rf /"

# View config
cargo run -- config

# Install shell integration
cargo run -- install --shell zsh
```

### 4.4 Install to System

```bash
cargo install --path .
```

Or use the provided install script (compiles + configures PATH + shell integration):

```bash
bash shell/install.sh
```

---

## 5. Testing

### 5.1 Run All Tests

```bash
cargo test
```

### 5.2 Run Single Test File

```bash
cargo test --test command_test
cargo test --test security_test
cargo test --test config_test
cargo test --test history_test
cargo test --test context_test
cargo test --test integration_test
```

### 5.3 Run Single Test Case

```bash
cargo test test_sanitize_removes_code_fences
cargo test --test security_test test_block_rm_rf_root
```

### 5.4 Pattern Filter

```bash
cargo test sanitize
cargo test history
```

### 5.5 Test Conventions

- Uses Rust built-in `#[test]` attribute, no external test framework
- Test files in `tests/` directory (integration tests)
- Uses `tempfile` crate for temporary file creation in tests
- Naming: `test_<module>_<behavior>_<condition>`
- Uses `assert!`, `assert_eq!`, and `matches!()` macros

---

## 6. Code Quality

### 6.1 Formatting

```bash
# Check format
cargo fmt -- --check

# Auto-fix format
cargo fmt
```

### 6.2 Lint (Clippy)

```bash
cargo clippy -- -D warnings
```

### 6.3 Full Check Workflow

After making changes, always run:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## 7. Architecture Design

### 7.1 Core Processing Flow

```
User natural language input
        ↓
    CLI (main.rs)
        ↓
    CoreEngine::process()
        ↓
    ┌─────────────────────────────────────────┐
    │  1. ShellContext::build()               │
    │     → Collect shell type, OS, cwd, hist │
    │                                         │
    │  2. CommandGenerator::generate()        │
    │     → Build system/user prompt          │
    │     → Call LLM Provider                 │
    │     → sanitize_command() clean output   │
    │                                         │
    │  3. SecurityChecker::check_command()    │
    │     → Keyword match danger patterns     │
    │     → Pass / Blocked                    │
    └─────────────────────────────────────────┘
        ↓
    ProcessResult::Command / Blocked / Error
```

### 7.2 Module Responsibilities

| Module | File | Responsibility |
|--------|------|----------------|
| **CLI** | `cli.rs`, `main.rs` | CLI argument parsing, subcommand dispatch |
| **Config** | `config.rs` | YAML config loading, saving, defaults |
| **Core** | `core.rs` | Orchestrates generate → security check → result |
| **Command** | `command.rs` | Prompt building, LLM calls, output sanitization |
| **Context** | `context.rs` | Collects current shell environment (OS, dirs, history) |
| **Security** | `security.rs` | Keyword-based command security interception |
| **History** | `history.rs` | Shell history file parsing (Bash/Zsh format) |
| **UI** | `ui.rs` | Terminal status messages (thinking/success/blocked/error) |
| **Error** | `error.rs` | Unified error type `AppError` |
| **LLM** | `llm/mod.rs` | `LlmProvider` trait definition + Provider factory |
| **OpenAI** | `llm/openai.rs` | OpenAI/Ollama (Chat Completions + Responses API) |
| **Anthropic** | `llm/anthropic.rs` | Anthropic (Messages API) |
| **Gemini** | `llm/gemini.rs` | Google Gemini (GenerateContent API) |

### 7.3 Key Design Patterns

- **Trait Polymorphism**: `LlmProvider` trait defines `chat_completion()` interface. Each provider implements the trait. `create_provider()` factory selects based on config.
- **Enum Results**: `ProcessResult` and `CheckResult` use enums for multi-outcome flows.
- **Composition**: `CoreEngine` composes `CommandGenerator`, `SecurityChecker`, `TerminalUi`.
- **Synchronous I/O**: Uses `reqwest::blocking` for HTTP. All I/O is synchronous.
- **Serde Serialization**: YAML for config, JSON for API payloads.

### 7.4 Error Handling

All errors use the `AppError` enum:

```rust
enum AppError {
    ConfigError(String),       // Config read/write/parse error
    LlmError(String),          // LLM API call error
    SecurityBlocked(String),   // Security interception
    IoError(String),           // IO error
    HistoryError(String),      // History file parse error
}
```

Error propagation uses `Result<T, AppError>`. External errors convert via `.map_err()`. `main.rs` outputs to stderr and exits with non-zero code.

---

## 8. Adding a New LLM Provider

### 8.1 Steps

1. Create a new file in `src/llm/`, e.g. `src/llm/newprovider.rs`

2. Implement the `LlmProvider` trait:

```rust
use super::types::*;
use super::LlmProvider;
use crate::config::LlmConfig;
use crate::error::AppError;

pub struct NewProvider {
    // ...
}

impl NewProvider {
    pub fn new(config: &LlmConfig) -> Result<Self, AppError> {
        // Initialize from config
        todo!()
    }
}

impl LlmProvider for NewProvider {
    fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        // Implement the specific API call logic
        todo!()
    }
}
```

3. Register in `src/llm/mod.rs`:

```rust
pub mod newprovider;
```

4. Add matching branch in `create_provider()` factory:

```rust
match config.provider.to_lowercase().as_str() {
    "newprovider" => Ok(Box::new(newprovider::NewProvider::new(config)?)),
    // ...
}
```

5. Add corresponding tests in `tests/`

---

## 9. Adding a New Shell Integration

1. Create integration script in `shell/`, e.g. `shell/shellmate.fish`
2. Implement:
   - Trigger prefix recognition (`@ai`, `#ai`, `/ai`)
   - Call `shellmate generate "<prompt>" --shell <shell>` for command generation
   - Display command and wait for user confirmation (Enter to execute / ESC to cancel)
   - Optional: keyboard shortcut binding
3. Add corresponding shell type support in `main.rs`'s `cmd_install()`

---

## 10. Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `clap` | 4 (derive) | CLI argument parsing |
| `serde` | 1 (derive) | Serialization/Deserialization |
| `serde_json` | 1 | JSON handling (API requests/responses) |
| `serde_yaml` | 0.9 | YAML handling (config files) |
| `reqwest` | 0.12 (blocking, rustls-tls) | HTTP client |
| `dirs` | 5 | System directory paths (home, config) |
| `chrono` | 0.4 | Time handling |
| `tempfile` | 3 (dev) | Test temporary files |

---

## 11. Contributing Guide

### 11.1 Branch Strategy

- Create a feature branch from `main`
- Branch naming: `feat/feature-name`, `fix/bug-name`, `chore/task-name`

### 11.2 Commit Conventions

- `feat:` — New features
- `fix:` — Bug fixes
- `chore:` — Maintenance tasks
- `release:` — Version bumps

### 11.3 PR Process

1. Ensure all tests pass: `cargo test`
2. Ensure linting passes: `cargo clippy -- -D warnings`
3. Ensure formatting: `cargo fmt`
4. Create a pull request with a clear description

---

## 12. Release Process

1. Update version in `Cargo.toml`
2. Run `cargo build --release` to verify
3. Run all tests: `cargo test`
4. Update `CHANGELOG.md`
5. Create git tag: `git tag v0.1.x`
6. Push tag: `git push origin v0.1.x`
7. CI will build and publish release artifacts

---

## 13. FAQ

### Q: OpenSSL errors during compilation?

This project uses `rustls-tls`, no system OpenSSL dependency needed. If you encounter issues, verify `Cargo.toml` has `default-features = false` for `reqwest`.

### Q: How to debug LLM requests?

Check stderr output for error messages. For detailed debugging, add temporary `eprintln!` statements in the provider's `chat_completion()` method to print request/response content.

### Q: How to change the default LLM model?

Edit `~/.shellmate/config.yaml` and modify the `llm.model` field. Or run `shellmate install` to reconfigure.

### Q: How to disable security checking?

Not recommended. But you can set `security.block_patterns` to an empty array `[]` in `config.yaml`.
