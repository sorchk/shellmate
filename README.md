[English](#english) | [中文](#中文)

---

<a id="english"></a>

<div align="center">

# ShellMate

**AI-Powered Shell Command Assistant**

[![Version](https://img.shields.io/badge/version-0.1.6-blue.svg)](CHANGELOG.md)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)

</div>

ShellMate is a command-line tool that translates natural language descriptions into shell commands using AI. It supports multiple LLM providers, includes a built-in security checker to block dangerous commands, and integrates seamlessly with bash, zsh, sh, and fish.

## Features

- **Natural Language to Command** — Describe what you want in plain language and get a ready-to-run shell command
- **Multi-Provider Support** — OpenAI, Anthropic, Gemini, Ollama, and custom endpoints
- **Security-First** — Built-in keyword-based command blocklist prevents dangerous operations (`rm -rf`, `mkfs`, `dd`, `curl | sh`, fork bombs, etc.)
- **Shell Integration** — Works with bash, zsh, sh, and fish via trigger prefixes (`@ai`, `#ai`, `/ai`) or keyboard shortcut (`Ctrl+G`)
- **Context-Aware** — Automatically gathers shell environment, OS, working directory, and command history for accurate results
- **Lightweight Model Friendly** — Works great with small local models like Qwen3.5-4B via Ollama — no GPU required
- **Privacy-Friendly** — Supports fully local models via Ollama — no data leaves your machine
- **Zero Async** — Written in pure synchronous Rust — fast startup, no runtime overhead

## Quick Start

### Install

#### Option A: Remote Install (Recommended)

```bash
curl -fsSL https://shai.apix.fun | sh
```

Downloads pre-built binary, configures PATH, and sets up shell integration automatically.

#### Option B: Build from Source

```bash
git clone https://github.com/sorchk/shellmate.git
cd shellmate
bash shell/install.sh
```

### Configure AI Provider

#### Option A: Local Model (Recommended)

ShellMate's prompt is optimized for shell command generation — even a 4B parameter model produces excellent results, with no API costs.

```bash
# 1. Install and start Ollama
ollama serve

# 2. Pull the model
ollama pull qwen3.5:4b

# 3. Configure ShellMate
shellmate install --config-only
```

Select Ollama in the interactive prompt. Configuration is stored at `~/.shellmate/config.yaml`:

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

This launches an interactive setup to choose your AI provider and enter credentials.

## Usage

After installation, you can invoke ShellMate directly in your shell.

### Trigger Prefix

Type a prefix followed by your request in natural language. Press Enter to generate the command:

```bash
$ @ai list all docker images
docker images -a
```

```bash
$ @ai find all .log files larger than 100MB
find / -name "*.log" -size +100M
```

```bash
$ @ai kill the process listening on port 8080
lsof -ti:8080 | xargs kill -9
```

Supported prefixes: `@ai`, `#ai`, `/ai` — all work the same way.

### Keyboard Shortcut

Type your request, then press `Ctrl+G` (default, configurable) to submit:

```bash
$ show disk usage of current directory<Ctrl+G>
du -sh .
```

### Confirm or Cancel

After the command is generated, press `Enter` to execute or `Esc` to cancel.

### Direct CLI Usage

```bash
# Generate a command
shellmate generate "list all files" --shell bash

# Check if a command is safe
shellmate check "rm -rf /"
# Output: BLOCKED: rm -rf /

# View current configuration
shellmate config

# Install shell integration
shellmate install --shell zsh
```

## Supported Providers

| Provider | Config Value | Default Endpoint |
|----------|-------------|-----------------|
| OpenAI | `openai` | `https://api.openai.com` |
| Anthropic | `anthropic` | `https://api.anthropics.com` |
| Gemini | `gemini` | `https://generativelanguage.googleapis.com` |
| Ollama | `ollama` | `http://localhost:11434` |
| Custom | any | user-defined |

Custom providers support OpenAI-compatible, Anthropic-compatible, and Gemini-compatible API formats.

For more advanced usage, see the [User Guide](docs/en/GUIDE.md) | [中文版](docs/zh/GUIDE.md).

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
- `apt remove`, `apt purge`

You can customize the blocklist in `~/.shellmate/config.yaml`. See the [Security Policy](docs/en/SECURITY.md) | [安全文档](docs/zh/SECURITY.md) for details.

## Documentation / 文档

- 📖 [User Guide / 使用指南](docs/en/GUIDE.md) | [中文](docs/zh/GUIDE.md)
- 🛠 [Development Guide / 开发文档](docs/en/DEVELOPMENT.md) | [中文](docs/zh/DEVELOPMENT.md)
- 🔒 [Security Policy / 安全文档](docs/en/SECURITY.md) | [中文](docs/zh/SECURITY.md)
- 📋 [Changelog / 变更日志](CHANGELOG.md)

## Contributors

- sorc

## License

Licensed under the [Apache License 2.0](LICENSE).

---

<a id="中文"></a>

<div align="center">

# ShellMate

**AI 驱动的 Shell 命令助手**

[![版本](https://img.shields.io/badge/版本-0.1.6-blue.svg)](CHANGELOG.md)
[![许可证: Apache 2.0](https://img.shields.io/badge/许可证-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)

</div>

ShellMate 是一个命令行工具，利用 AI 将自然语言描述转换为 shell 命令。支持多种大语言模型提供商，内置安全检查器阻止危险命令，并与 bash、zsh、sh 和 fish 无缝集成。

## 功能特性

- **自然语言转命令** — 用自然语言描述你想要的操作，即可获得可直接执行的 shell 命令
- **多模型支持** — 支持 OpenAI、Anthropic、Gemini、Ollama 及自定义端点
- **安全优先** — 内置基于关键词匹配的命令黑名单，阻止危险操作（`rm -rf`、`mkfs`、`dd`、`curl | sh`、Fork 炸弹等）
- **Shell 集成** — 支持 bash、zsh、sh、fish，通过触发前缀（`@ai`、`#ai`、`/ai`）或快捷键（`Ctrl+G`）使用
- **上下文感知** — 自动收集 shell 环境、操作系统、工作目录和命令历史，生成更准确的命令
- **轻量模型友好** — 仅需 Qwen3.5-4B 等小模型即可优雅运行，无需 GPU，零成本
- **隐私友好** — 支持通过 Ollama 使用完全本地化的模型，数据不会离开你的机器
- **零异步** — 纯同步 Rust 编写，启动快速，无运行时开销

## 快速开始

### 安装

#### 方式一：远程安装（推荐）

```bash
curl -fsSL https://shai.apix.fun | sh
```

自动下载预编译二进制文件，配置 PATH 环境变量，并设置 Shell 集成。

#### 方式二：从源码构建

```bash
git clone https://github.com/sorchk/shellmate.git
cd shellmate
bash shell/install.sh
```

### 配置 AI 提供商

#### 方式一：本地模型（推荐）

ShellMate 的提示词专为 shell 命令生成优化，即使 4B 参数的小模型也能产生优秀结果，无需 API 费用。

```bash
# 1. 安装并启动 Ollama
ollama serve

# 2. 拉取模型
ollama pull qwen3.5:4b

# 3. 配置 ShellMate
shellmate install --config-only
```

在交互提示中选择 Ollama。配置保存在 `~/.shellmate/config.yaml`：

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
shellmate install --shell bash
```

启动交互式配置，选择 AI 提供商并输入凭据。

## 使用方法

安装完成后，可以直接在 shell 中调用 ShellMate。

### 触发前缀

输入前缀加自然语言描述，按回车即可生成命令：

```bash
$ @ai 列出所有 Docker 镜像
docker images -a
```

```bash
$ @ai 查找所有大于 100MB 的 .log 文件
find / -name "*.log" -size +100M
```

```bash
$ @ai 杀掉占用 8080 端口的进程
lsof -ti:8080 | xargs kill -9
```

支持的前缀：`@ai`、`#ai`、`/ai`，效果相同。

### 快捷键

输入描述后按 `Ctrl+G`（默认，可配置）提交：

```bash
$ 显示当前目录磁盘占用<Ctrl+G>
du -sh .
```

### 确认或取消

命令生成后，按 `Enter` 执行，按 `Esc` 取消。

### 直接使用 CLI

```bash
# 生成命令
shellmate generate "列出所有文件" --shell bash

# 安全检查
shellmate check "rm -rf /"
# 输出: BLOCKED: rm -rf /

# 查看当前配置
shellmate config

# 安装 Shell 集成
shellmate install --shell zsh
```

## 支持的 AI 提供商

| 提供商 | 配置值 | 默认地址 |
|--------|--------|----------|
| OpenAI | `openai` | `https://api.openai.com` |
| Anthropic | `anthropic` | `https://api.anthropics.com` |
| Gemini | `gemini` | `https://generativelanguage.googleapis.com` |
| Ollama（本地） | `ollama` | `http://localhost:11434` |
| 自定义 | 任意 | 用户自定义 |

自定义提供商支持 OpenAI 兼容、Anthropic 兼容和 Gemini 兼容的 API 格式。

更多高级用法请参阅[使用指南](docs/zh/GUIDE.md) | [English](docs/en/GUIDE.md)。

## 安全机制

ShellMate 默认阻止以下危险命令：

- `rm`、`mkfs`、`dd`、`fdisk`、`parted`、`sfdisk`、`shred`
- `wipefs`、`cfdisk`、`gdisk`、`sgdisk`、`blkdiscard`
- `curl | sh`、`curl | bash` 管道执行模式
- `chmod -R 777 /`
- `killall`、`iptables -F`
- `halt`、`shutdown`、`reboot`、`poweroff`、`init 0/1/6`
- Fork 炸弹 `:(){:|:&};:`
- `-delete`、`-exec`、`--no-preserve-root`、`> /dev/`
- `apt remove`、`apt purge`

你可以在 `~/.shellmate/config.yaml` 中自定义黑名单规则。详见[安全文档](docs/zh/SECURITY.md) | [Security Policy](docs/en/SECURITY.md)。

## 许可证

基于 [Apache License 2.0](LICENSE) 开源。