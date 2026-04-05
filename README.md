<div align="center">

# ShellMate

**AI-Powered Shell Command Assistant**

[English](#features) | [中文](#功能特性)

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)

</div>

ShellMate is a command-line tool that translates natural language descriptions into shell commands using AI. It supports multiple LLM providers, includes a built-in security checker to block dangerous commands, and integrates seamlessly with your shell.

## Features

- **Natural Language to Command** — Describe what you want in plain language and get a ready-to-run shell command
- **Multi-Provider Support** — OpenAI, Anthropic, Gemini, Ollama, and custom endpoints
- **Security-First** — Built-in keyword-based command blocklist prevents dangerous operations (`rm -rf`, `mkfs`, `dd`, etc.)
- **Shell Integration** — Works with bash, zsh, sh, and fish via trigger prefixes (`@ai`, `#ai`, `/ai`) or keyboard shortcut (`Ctrl+G`)
- **Context-Aware** — Automatically gathers shell environment, OS, working directory, and command history for accurate results
- **Lightweight Model Friendly** — Works great with small local models like Qwen3.5-4B via Ollama — no GPU required
- **Privacy-Friendly** — Supports fully local models via Ollama — no data leaves your machine
- **Zero Async** — Written in pure synchronous Rust — fast startup, no runtime overhead

## Quick Start

### Install from Source

```bash
git clone https://github.com/sorchk/shellmate.git
cd shellmate
cargo build --release
```

The binary will be at `target/release/shellmate`.

### Configure AI Provider

#### Option A: Local Model (Recommended)

ShellMate's prompt is optimized for shell command generation — even a 4B parameter model produces excellent results. no API costs.

```bash
# 1. Install and start Ollama
ollama serve

# 2. Pull the model
ollama pull qwen3.5:4b

# 3. Configure ShellMate
```

Edit `~/.shellmate/config.yaml`:

```yaml
llm:
  provider: ollama
  model: qwen3.5:4b
  base_url: http://localhost:11434
  api_key: null
```

That's it — everything runs locally, no API key needed.

#### Option B: Cloud Provider

```bash
shellmate install --shell bash
```

This launches an interactive setup to choose your AI provider and enter credentials. Configuration is stored at `~/.shellmate/config.yaml`.

You can also edit the config directly:

```yaml
llm:
  provider: openai
  model: gpt-4-turbo
  api_key: sk-...
  base_url: https://api.openai.com
```

### Shell Integration

Add this line to your `~/.bashrc` (or `~/.zshrc` for zsh):

```bash
source /path/to/shellmate/shell/shellmate.bash
```

Then use ShellMate directly in your shell:

```bash
$ @ai list all docker images
docker images -a
```

## Usage

```bash
# Generate a command from natural language
shellmate generate "find all .log files larger than 100MB" --shell bash

# Check if a command passes security rules
shellmate check "rm -rf /"

# View current configuration
shellmate config

# Install shell integration
shellmate install --shell zsh
```

## Supported Providers

| Provider | Config Value | Default Endpoint |
|----------|-------------|-----------------|
| OpenAI | `openai` | `https://api.openai.com` |
| Anthropic | `anthropic` | `https://api.anthropic.com` |
| Gemini | `gemini` | `https://generativelanguage.googleapis.com` |
| Ollama | `ollama` | `http://localhost:11434` |
| Kimi Coding | `kimi-coding` | user-defined |
| MiniMax | `minimax` | user-defined |
| Custom | any | user-defined |

Custom providers support OpenAI-compatible, Anthropic-compatible, and Gemini-compatible API formats.

## Security

ShellMate blocks dangerous commands by default, including:

- `rm`, `mkfs`, `dd`, `fdisk`, `parted`, `sfdisk`, `shred`
- `wipefs`, `cfdisk`, `gdisk`, `sgdisk`, `blkdiscard`
- `curl | sh`, `curl | bash` patterns
- `chmod -R 777 /`
- `killall`, `iptables -F`
- `halt`, `shutdown`, `reboot`, `poweroff`, `init 0/1/6`
- Fork bombs `:(){:|:&};:`
- `-delete`, `-exec`, `--no-preserve-root`, `> /dev/`

You can customize the blocklist in `~/.shellmate/config.yaml`:

```yaml
security:
  mode: strict
  block_patterns:
    - "rm"
    - "mkfs"
    # ... add your own keywords
```

## Configuration

Config file: `~/.shellmate/config.yaml`

```yaml
trigger:
  prefixes: ["@ai", "#ai", "/ai"]
  shortcut: "Ctrl+G"

llm:
  provider: openai
  model: gpt-4-turbo
  api_key: sk-...
  base_url: https://api.openai.com
  timeout: 30
  max_tokens: null

security:
  mode: strict
  block_patterns: [...]

ui:
  position: top
  success_duration: 2600
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

## Project Structure

```
src/
  main.rs          CLI entrypoint (clap)
  cli.rs           CLI definitions
  config.rs        YAML config management
  core.rs          CoreEngine orchestrator
  command.rs       Prompt building & output sanitization
  context.rs       Shell environment detection
  security.rs      Regex-based command blocklist
  history.rs       Shell history parsing
  ui.rs            Terminal status messages
  error.rs         Custom error types
  llm/             LLM provider implementations
    mod.rs         Provider trait & factory
    openai.rs      OpenAI / Ollama provider
    anthropic.rs   Anthropic provider
    gemini.rs      Gemini provider
    types.rs       Shared API types
tests/             Integration tests
shell/             Shell integration scripts
```

## License

Licensed under the [Apache License 2.0](LICENSE).

---

<div align="center">

## 功能特性

</div>

ShellMate 是一个命令行工具，利用 AI 将自然语言描述转换为 shell 命令。支持多种大语言模型提供商，内置安全检查器阻止危险命令，并与你的 shell 无缝集成。

- **自然语言转命令** — 用自然语言描述你想要的操作，即可获得可直接执行的 shell 命令
- **多模型支持** — 支持 OpenAI、Anthropic、Gemini、Ollama 及自定义端点
- **安全优先** — 内置基于关键词匹配的命令黑名单，阻止危险操作（`rm`、`mkfs`、`dd` 等）
- **Shell 集成** — 支持 bash、zsh、sh、fish，通过触发前缀（`@ai`、`#ai`、`/ai`）或快捷键（`Ctrl+G`）使用
- **上下文感知** — 自动收集 shell 环境、操作系统、工作目录和命令历史，生成更准确的命令
- **轻量模型友好** — 仅需 Qwen3.5-4B 等小模型即可优雅运行，无需 GPU，零成本
- **隐私友好** — 支持通过 Ollama 使用完全本地化的模型，数据不会离开你的机器
- **零异步** — 纯同步 Rust 编写，启动快速，无运行时开销

### 快速开始

```bash
# 克隆并编译
git clone https://github.com/your-username/shellmate.git
cd shellmate
cargo build --release
```

#### 方式一：本地模型（推荐）

ShellMate 的提示词专为 shell 命令生成优化，即使 4B 参数的小模型也能产生优秀结果。无需 API 费用。

```bash
# 1. 安装并启动 Ollama
ollama serve

# 2. 拉取模型
ollama pull qwen3.5:4b
```

编辑 `~/.shellmate/config.yaml`：

```yaml
llm:
  provider: ollama
  model: qwen3.5:4b
  base_url: http://localhost:11434
  api_key: null
```

全部在本地运行，无需 API Key。

#### 方式二：云端提供商

```bash
# 交互式配置 AI 提供商
./target/release/shellmate install --shell bash
```

#### Shell 集成

将以下内容添加到 `~/.bashrc`（zsh 用户添加到 `~/.zshrc`）：

```bash
source /path/to/shellmate/shell/shellmate.bash
```

在 shell 中直接使用：

```bash
$ @ai 列出所有 Docker 镜像
docker images -a
```

### 命令行用法

```bash
# 从自然语言生成命令
shellmate generate "查找所有大于 100MB 的 .log 文件" --shell bash

# 检查命令是否通过安全规则
shellmate check "rm -rf /"

# 查看当前配置
shellmate config

# 安装 shell 集成
shellmate install --shell zsh
```

### 支持的 AI 提供商

| 提供商 | 配置值 | 默认地址 |
|--------|--------|----------|
| OpenAI | `openai` | `https://api.openai.com` |
| Anthropic | `anthropic` | `https://api.anthropic.com` |
| Gemini | `gemini` | `https://generativelanguage.googleapis.com` |
| Ollama（本地） | `ollama` | `http://localhost:11434` |
| 自定义 | 任意 | 用户自定义 |

### 安全机制

ShellMate 默认阻止以下危险命令：

- `rm`、`mkfs`、`dd`、`fdisk`、`parted`、`sfdisk`、`shred`
- `wipefs`、`cfdisk`、`gdisk`、`sgdisk`、`blkdiscard`
- `curl | sh`、`curl | bash` 管道执行模式
- `chmod -R 777 /`
- `killall`、`iptables -F`
- `halt`、`shutdown`、`reboot`、`poweroff`、`init 0/1/6`
- Fork 炸弹 `:(){:|:&};:`
- `-delete`、`-exec`、`--no-preserve-root`、`> /dev/`

你可以在 `~/.shellmate/config.yaml` 中自定义黑名单规则。

### 开发

```bash
cargo build          # 编译
cargo test           # 运行测试
cargo clippy -- -D warnings  # 代码检查
cargo fmt            # 格式化
```

### 许可证

基于 [Apache License 2.0](LICENSE) 开源。
