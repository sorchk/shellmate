[English](../../en/GUIDE.md) | [中文](../../zh/GUIDE.md) | [← README](../../README.md)

---

# ShellMate User Guide

## 1. Introduction

ShellMate is an AI-powered command-line assistant that helps you generate shell commands from natural language descriptions. Simply type what you want to do, and ShellMate calls an AI model to produce the right command.

### Feature Highlights

- **Natural Language to Commands** — Type "list all files" and get `ls -la`
- **Security Interception** — Blocks dangerous commands like `rm -rf /`, `mkfs`, `dd`, etc.
- **Multi AI Provider** — OpenAI, Ollama, Anthropic, Gemini, custom providers
- **Shell Integration** — Supports Bash, Zsh, Sh, Fish
- **Multiple Triggers** — Prefix triggers (`@ai`/`#ai`/`/ai`) or keyboard shortcut (`Ctrl+G`)

---

## 2. Installation

### Method 1: Remote Install (Recommended)

```bash
curl -fsSL https://shai.apix.fun | sh
```

This script automatically:
1. Downloads the latest pre-built binary for your platform (Linux amd64/arm64, macOS arm64)
2. Installs to `~/.shellmate/bin/`
3. Configures PATH environment variable
4. Sets up shell integration
5. Guides you through AI provider configuration

After installation, restart your terminal or run:

```bash
source ~/.bashrc      # Bash users
source ~/.zshrc       # Zsh users
```

### Method 2: Build from Source

**Prerequisites:** [Rust](https://rustup.rs/) installed

```bash
# Clone the repository
git clone https://github.com/sorchk/shellmate.git
cd shellmate

# Build release binary
cargo build --release

# Copy to PATH
mkdir -p ~/.shellmate/bin
cp target/release/shellmate ~/.shellmate/bin/
```

Add the following to your shell config file:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.shellmate/bin:$PATH"
```

### Method 3: Cargo Install

```bash
cargo install --path /path/to/shellmate
```

This installs `shellmate` to `~/.cargo/bin/shellmate`.

---

## 3. Shell Integration

Shell integration lets you use trigger prefixes or keyboard shortcuts directly in your terminal.

### 3.1 Using `shellmate install`

```bash
# Auto-detect current shell
shellmate install

# Specify shell type
shellmate install --shell bash
shellmate install --shell zsh

# Only configure AI provider (skip shell integration)
shellmate install --config-only
```

The install command will:
1. Display shell type and RC file path
2. Guide you through AI provider configuration
3. Show the `source` line you need to add manually

### 3.2 Manual Bash Configuration

Edit `~/.bashrc` (or `~/.bash_profile` on macOS), add:

```bash
source "$HOME/.shellmate/shell/shellmate.bash"
```

### 3.3 Manual Zsh Configuration

Edit `~/.zshrc`, add:

```bash
source "$HOME/.shellmate/shell/shellmate.zsh"
```

### 3.4 Manual Sh Configuration

Edit `~/.profile`, add:

```bash
. "$HOME/.shellmate/shell/shellmate.sh"
```

### 3.5 After Configuration

Reload your shell configuration:

```bash
source ~/.bashrc    # Bash
source ~/.zshrc     # Zsh
```

---

## 4. AI Provider Configuration

ShellMate requires an AI provider to function. Configuration is stored at `~/.shellmate/config.yaml`.

### 4.1 Interactive Configuration

```bash
shellmate install
```

Follow the prompts to select a provider, enter API key and model name.

### 4.2 Manual Configuration

Edit `~/.shellmate/config.yaml`:

#### OpenAI

```yaml
llm:
  provider: "openai"
  model: "gpt-4-turbo"
  api_key: "sk-xxxxxxxxxxxxxxxx"
  base_url: "https://api.openai.com"
  timeout: 30
```

#### OpenAI-Compatible Services (DeepSeek, Moonshot, etc.)

```yaml
llm:
  provider: "openai"
  model: "deepseek-chat"
  api_key: "your-api-key"
  base_url: "https://api.deepseek.com"
  api_type: "openai-completions"
  timeout: 30
```

#### Ollama (Local Model, No API Key Required)

```yaml
llm:
  provider: "ollama"
  model: "qwen3.5:4b"
  base_url: "http://localhost:11434"
  timeout: 60
```

> Ensure Ollama is running and the model is pulled:
> ```bash
> ollama serve
> ollama pull qwen3.5:4b
> ```

#### Anthropic

```yaml
llm:
  provider: "anthropic"
  model: "claude-3-sonnet-20240229"
  api_key: "sk-ant-xxxxxxxxxxxxxxxx"
  base_url: "https://api.anthropic.com"
  timeout: 30
```

#### Google Gemini

```yaml
llm:
  provider: "gemini"
  model: "gemini-pro"
  api_key: "your-gemini-api-key"
  base_url: "https://generativelanguage.googleapis.com"
  timeout: 30
```

#### Custom Provider

```yaml
llm:
  provider: "my-provider"
  model: "my-model"
  api_key: "your-api-key"
  base_url: "https://api.example.com"
  api_type: "openai-completions"
  timeout: 30
```

Available `api_type` values:
- `openai-completions` — OpenAI Chat Completions API
- `openai-responses` — OpenAI Responses API
- `anthropic-messages` — Anthropic Messages API
- `gemini-generate-content` — Gemini GenerateContent API
- `custom` — Uses `base_url` as the complete endpoint

---

## 5. Usage

### 5.1 Prefix Trigger

Type a trigger prefix + natural language description in your terminal:

```bash
$ @ai list all files in current directory
ls -la

$ @ai find all .rs files
find . -name "*.rs" -type f

$ @ai count files in src directory
ls -1 src | wc -l
```

Supported prefixes: `@ai`, `#ai`, `/ai`

### 5.2 Keyboard Shortcut

1. Type your natural language description (without prefix)
2. Press `Ctrl+G` (default, configurable in `~/.shellmate/config.yaml`)
3. ShellMate automatically adds the prefix and processes the request

To change the shortcut, edit `trigger.shortcut` in config and run `shellmate install --shell <bash|zsh>` to apply. Supported formats: `Ctrl+A`–`Ctrl+Z`, `Alt+A`–`Alt+Z`.

### 5.3 Command Confirmation

After generation, the command is displayed waiting for your action:

- **Press Enter** — Execute the command (shows green ✓)
- **Press ESC** — Cancel the command (shows red ✗)

### 5.4 Direct CLI Usage

Even without shell integration, you can use the CLI directly:

```bash
# Generate a command
shellmate generate "list all files" --shell bash

# Security check
shellmate check "rm -rf /"
# Output: BLOCKED: rm -rf /

shellmate check "ls -la"
# Output: PASS: command is safe

# View current configuration
shellmate config

# Install shell integration
shellmate install --shell zsh
```

### 5.5 Security Interception

When AI generates a dangerous command, ShellMate intercepts it:

```
⚠ Blocked command (time: 2.1s, Tokens: 156, TTFT: 2100ms, TPS: 74.3)
  rm -rf /tmp/important
```

Default blocked patterns include: `rm`, `mkfs`, `dd`, `fdisk`, `parted`, `shred`, `| sh`, `| bash`, `chmod -R 777 /`, `killall`, `iptables -F`, `-delete`, `-exec`, `--no-preserve-root`, `> /dev/`, `apt remove`, `apt purge`, `halt`, `shutdown`, `reboot`, `poweroff`, `init 0/1/6`, fork bombs, and more.

---

## 6. Configuration Reference

Complete `~/.shellmate/config.yaml`:

```yaml
trigger:
  prefixes:
    - "@ai"
    - "#ai"
    - "/ai"
  shortcut: "Ctrl+G"

llm:
  provider: "openai"         # Provider name
  model: "gpt-4-turbo"       # Model name
  timeout: 30                # Request timeout (seconds)
  api_key: "sk-..."          # API Key (optional, not needed for Ollama)
  base_url: "https://api.openai.com"  # API Base URL
  api_type: null             # API type (optional)
  max_tokens: null           # Max generation tokens (optional)

security:
  mode: "strict"             # Security mode
  block_patterns:            # Block keyword list
    - "rm"
    - "mkfs"
    # ... (see full list below)

ui:
  position: "top"            # Status message position
  success_duration: 2600     # Success message duration (ms)
```

### Configuration Options

| Field | Default | Description |
|-------|---------|-------------|
| `trigger.prefixes` | `["@ai", "#ai", "/ai"]` | Trigger prefix list |
| `trigger.shortcut` | `Ctrl+G` | Keyboard shortcut (format: `Ctrl+<Key>` or `Alt+<Key>`) |
| `llm.provider` | `openai` | AI Provider name |
| `llm.model` | `gpt-4-turbo` | Model name |
| `llm.timeout` | `30` | Request timeout (seconds) |
| `llm.api_key` | `null` | API Key |
| `llm.base_url` | `null` | API Base URL |
| `llm.api_type` | `null` | API type |
| `llm.max_tokens` | `null` | Max generation tokens |
| `security.mode` | `strict` | Security mode |
| `security.block_patterns` | (see below) | Danger command patterns |
| `ui.position` | `top` | Status message position |
| `ui.success_duration` | `2600` | Success message duration (ms) |

---

## 7. Usage Examples

### File Operations

```bash
$ @ai find files larger than 100MB
find . -type f -size +100M

$ @ai count lines of code in src directory
find src -name "*.rs" | xargs wc -l
```

### System Administration

```bash
$ @ai check which process is using port 8080
lsof -i :8080

$ @ai check disk usage
df -h

$ @ai check Docker container logs
docker logs <container_id> --tail 100
```

### Git Operations

```bash
$ @ai check last 5 commits
git log --oneline -5

$ @ai undo last commit (keep changes)
git reset --soft HEAD~1

$ @ai create and switch to new branch
git checkout -b feature/new-feature
```

### Network Tools

```bash
$ @ai check my IP address
ip addr show | grep "inet "

$ @ai test connectivity to google.com
ping -c 4 google.com
```

---

## 8. Exit Codes

| Exit Code | Meaning |
|-----------|---------|
| `0` | Success |
| `1` | Error (config error, API error, etc.) |
| `2` | Command blocked by security checker |

---

## 9. Troubleshooting & FAQ

### Q: Command not found after installation?

Ensure `~/.shellmate/bin/` is in PATH:

```bash
echo $PATH | grep shellmate
```

If not found, add to your shell config:

```bash
export PATH="$HOME/.shellmate/bin:$PATH"
```

### Q: Trigger prefix has no response?

Check if shell integration is loaded:

```bash
grep shellmate ~/.bashrc ~/.zshrc
```

Ensure you've run `source ~/.bashrc` or `source ~/.zshrc`.

### Q: API call timeout?

Edit `~/.shellmate/config.yaml` and increase the `timeout` value:

```yaml
llm:
  timeout: 60
```

### Q: Ollama connection failed?

1. Ensure Ollama is running: `ollama serve`
2. Ensure base_url is correct: `http://localhost:11434`
3. Ensure model is pulled: `ollama pull qwen3.5:4b`

### Q: How to change the keyboard shortcut?

Edit `~/.shellmate/config.yaml`:

```yaml
trigger:
  shortcut: "Alt+X"   # Supports Ctrl+A-Z, Alt+A-Z
```

Then reinstall shell integration to apply:

```bash
shellmate install --shell bash   # or zsh
```

Restart your terminal or run `source ~/.bashrc` (or `~/.zshrc`).

### Q: How to change trigger prefixes?

Edit `~/.shellmate/config.yaml`:

```yaml
trigger:
  prefixes:
    - "@ai"
    - "hey"
```

Also update the `prefixes` variable in the corresponding shell integration script.

### Q: How to temporarily disable ShellMate?

Remove the `source .../shellmate.*` line from your shell config and reload.

### Q: Command executes without confirmation?

This usually happens in shells that don't support `read -rsn1`. Ensure you're using Bash 4+ or Zsh.
