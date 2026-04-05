# ShellMate 源码开发与构建说明

## 1. 项目概述

ShellMate 是一个用 Rust 编写的 AI 驱动的 Shell 命令助手。它可以根据自然语言描述生成对应的 Shell 命令，并通过安全检查机制阻止危险命令的执行。

**核心特性：**

- 自然语言 → Shell 命令转换
- 多 AI Provider 支持（OpenAI / Ollama / Anthropic / Gemini / 自定义）
- 基于 Keyword 的命令安全检查（阻止 `rm`、`mkfs`、`dd` 等危险命令）
- Bash / Zsh / Sh / Fish 的 Shell 集成
- 触发前缀（`@ai`、`#ai`、`/ai`）和快捷键（`Ctrl+G`）支持

**技术栈：**

- Rust 2021 Edition
- `clap` — CLI 参数解析
- `reqwest` (blocking, rustls-tls) — HTTP 请求
- `serde` + `serde_yaml` + `serde_json` — 序列化/反序列化
- `regex` — 安全规则匹配（注：实际代码使用关键词匹配，未依赖 regex crate）
- `chrono` — 时间处理
- `dirs` — 系统目录检测
- `tempfile` (dev) — 测试临时文件

---

## 2. 环境准备

### 2.1 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

验证安装：

```bash
rustc --version    # 需要 1.56+ (Rust 2021 Edition)
cargo --version
```

### 2.2 系统依赖

- **Linux**: 需要 `pkg-config` 和 `openssl` 开发库（如果使用 `native-tls`），本项目使用 `rustls-tls`，无需 OpenSSL

  ```bash
  # Debian/Ubuntu
  sudo apt update && sudo apt install build-essential

  # Fedora/RHEL
  sudo dnf install gcc
  ```

- **macOS**: 安装 Xcode Command Line Tools

  ```bash
  xcode-select --install
  ```

---

## 3. 项目结构

```
shell-mate/
├── Cargo.toml              # 项目清单与依赖声明
├── Cargo.lock              # 依赖版本锁定
├── AGENTS.md               # AI 辅助开发说明
├── .gitignore
├── src/
│   ├── main.rs             # CLI 入口 (clap 参数解析, 命令分发)
│   ├── lib.rs              # 库根 — 重导出所有模块
│   ├── cli.rs              # Clap CLI 定义 (Commands 枚举)
│   ├── config.rs           # YAML 配置加载/保存, Config 结构体 (serde)
│   ├── core.rs             # CoreEngine — 编排 generate → security check → result
│   ├── command.rs          # CommandGenerator — 构建 prompt, 清洗 LLM 输出
│   ├── context.rs          # ShellContext — 收集 shell 环境、OS、cwd、历史
│   ├── security.rs         # SecurityChecker — 基于 regex 的命令安全拦截
│   ├── history.rs          # Shell 历史文件解析 (bash/zsh)
│   ├── ui.rs               # Terminal UI — 状态消息输出到 stderr
│   ├── error.rs            # AppError 枚举 (自定义错误类型)
│   └── llm/
│       ├── mod.rs          # LlmProvider trait + 工厂函数 create_provider()
│       ├── types.rs        # ChatMessage, ChatCompletionRequest/Response, ResponseUsage
│       ├── openai.rs       # OpenAI / Ollama provider (chat completions + responses API)
│       ├── anthropic.rs    # Anthropic / Kimi / MiniMax provider
│       └── gemini.rs       # Google Gemini provider
├── tests/                  # 集成测试 (每个模块一个文件)
│   ├── command_test.rs
│   ├── config_test.rs
│   ├── context_test.rs
│   ├── history_test.rs
│   ├── integration_test.rs
│   └── security_test.rs
├── shell/                  # Shell 集成脚本
│   ├── install.sh          # 一键安装脚本
│   ├── shellmate.bash      # Bash 集成 (command_not_found_handle + Ctrl+G)
│   ├── shellmate.zsh       # Zsh 集成 (preexec hook + Ctrl+G)
│   └── shellmate.sh        # POSIX sh 集成
└── docs/                   # 文档
```

---

## 4. 构建与运行

### 4.1 Debug 构建

```bash
cargo build
```

二进制文件输出到 `target/debug/shellmate`。

### 4.2 Release 构建

```bash
cargo build --release
```

二进制文件输出到 `target/release/shellmate`，启用优化，体积更小，性能更好。

### 4.3 直接运行

```bash
# 生成命令
cargo run -- generate "列出所有文件" --shell bash

# 安全检查
cargo run -- check "rm -rf /"

# 查看配置
cargo run -- config

# 安装 Shell 集成
cargo run -- install --shell zsh
```

### 4.4 安装到系统

```bash
cargo install --path .
```

这将把 `shellmate` 安装到 `~/.cargo/bin/shellmate`。

也可以使用项目提供的一键安装脚本（编译 + 配置 PATH + Shell 集成）：

```bash
bash shell/install.sh
```

---

## 5. 测试

### 5.1 运行全部测试

```bash
cargo test
```

### 5.2 运行单个测试文件

```bash
cargo test --test command_test
cargo test --test security_test
cargo test --test config_test
cargo test --test history_test
cargo test --test context_test
cargo test --test integration_test
```

### 5.3 运行单个测试用例

```bash
cargo test test_sanitize_removes_code_fences
cargo test --test security_test test_block_rm_rf_root
```

### 5.4 按名称模式过滤

```bash
cargo test sanitize
cargo test history
```

### 5.5 测试框架与约定

- 使用 Rust 内置的 `#[test]` 属性，无额外测试框架
- 测试文件位于 `tests/` 目录，为集成测试
- 使用 `tempfile` crate 创建临时文件进行文件相关测试
- 测试命名：`test_<模块>_<行为>_<条件>`

---

## 6. 代码质量检查

### 6.1 格式化

```bash
# 检查格式
cargo fmt -- --check

# 自动修复格式
cargo fmt
```

### 6.2 Lint (Clippy)

```bash
cargo clippy -- -D warnings
```

### 6.3 完整检查流程

每次代码修改后，建议运行：

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## 7. 架构设计

### 7.1 核心处理流程

```
用户输入自然语言
       ↓
   CLI (main.rs)
       ↓
   CoreEngine::process()
       ↓
   ┌─────────────────────────────────────────┐
   │  1. ShellContext::build()               │
   │     → 收集 shell 类型、OS、cwd、历史    │
   │                                         │
   │  2. CommandGenerator::generate()        │
   │     → 构建 system/user prompt           │
   │     → 调用 LLM Provider                 │
   │     → sanitize_command() 清洗输出       │
   │                                         │
   │  3. SecurityChecker::check_command()    │
    │     → 关键词匹配危险命令模式            │
   │     → Pass / Blocked                    │
   └─────────────────────────────────────────┘
       ↓
   ProcessResult::Command / Blocked / Error
```

### 7.2 模块职责

| 模块 | 文件 | 职责 |
|------|------|------|
| **CLI** | `cli.rs`, `main.rs` | 命令行参数解析、子命令分发 |
| **Config** | `config.rs` | YAML 配置文件的加载、保存、默认值 |
| **Core** | `core.rs` | 编排生成→安全检查→结果的完整流程 |
| **Command** | `command.rs` | Prompt 构建、LLM 调用、输出清洗 |
| **Context** | `context.rs` | 收集当前 Shell 环境（OS、目录、历史） |
| **Security** | `security.rs` | 基于关键词的命令安全拦截 |
| **History** | `history.rs` | Shell 历史文件解析（Bash/Zsh 格式） |
| **UI** | `ui.rs` | 终端状态消息（思考中/成功/拦截/错误） |
| **Error** | `error.rs` | 统一错误类型 `AppError` |
| **LLM** | `llm/mod.rs` | `LlmProvider` trait 定义与 Provider 工厂 |
| **OpenAI** | `llm/openai.rs` | OpenAI/Ollama (Chat Completions + Responses API) |
| **Anthropic** | `llm/anthropic.rs` | Anthropic/Kimi/MiniMax (Messages API) || **Gemini** | `llm/gemini.rs` | Google Gemini (GenerateContent API) |

### 7.3 关键设计模式

- **Trait 多态**: `LlmProvider` trait 定义了 `chat_completion()` 接口，各 Provider 分别实现。`create_provider()` 工厂函数根据配置选择实现。
- **枚举结果**: `ProcessResult` 和 `CheckResult` 使用枚举表示多种结果状态。
- **组合模式**: `CoreEngine` 组合了 `CommandGenerator`、`SecurityChecker`、`TerminalUi`。
- **同步 I/O**: 使用 `reqwest::blocking` 进行 HTTP 请求，全部 I/O 为同步阻塞式。
- **Serde 序列化**: YAML 用于配置文件，JSON 用于 API 请求体。

### 7.4 错误处理

所有错误统一使用 `AppError` 枚举：

```rust
enum AppError {
    ConfigError(String),       // 配置读写/解析错误
    LlmError(String),          // LLM API 调用错误
    SecurityBlocked(String),   // 安全拦截（命令被阻止）
    IoError(String),           // IO 错误
    HistoryError(String),      // 历史文件解析错误
}
```

错误传播使用 `Result<T, AppError>`，外部错误通过 `.map_err()` 转换。`main.rs` 中统一输出到 stderr 并以非零退出码退出。

---

## 8. 添加新的 LLM Provider

### 8.1 步骤

1. 在 `src/llm/` 下创建新文件，如 `src/llm/newprovider.rs`
2. 实现 `LlmProvider` trait：

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
        // 从 config 初始化
        todo!()
    }
}

impl LlmProvider for NewProvider {
    fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        // 实现具体的 API 调用逻辑
        todo!()
    }
}
```

3. 在 `src/llm/mod.rs` 中注册：

```rust
pub mod newprovider;
```

4. 在 `create_provider()` 工厂函数中添加匹配分支：

```rust
match config.provider.to_lowercase().as_str() {
    "newprovider" => Ok(Box::new(newprovider::NewProvider::new(config)?)),
    // ...
}
```

5. 在 `tests/` 下添加对应的测试

---

## 9. 添加新的 Shell 集成

1. 在 `shell/` 目录下创建集成脚本，如 `shell/shellmate.fish`
2. 实现以下功能：
   - 触发前缀识别（`@ai`、`#ai`、`/ai`）
   - 调用 `shellmate generate "<prompt>" --shell <shell>` 获取命令
   - 显示命令并等待用户确认（Enter 执行 / ESC 取消）
   - 可选：快捷键绑定
3. 在 `main.rs` 的 `cmd_install()` 中添加对应的 shell 类型支持

---

## 10. 配置文件格式

配置文件位于 `~/.shellmate/config.yaml`：

```yaml
trigger:
  prefixes:
    - "@ai"
    - "#ai"
    - "/ai"
  shortcut: "Ctrl+G"

llm:
  provider: "openai"
  model: "gpt-4-turbo"
  timeout: 30
  api_key: "sk-..."
  base_url: "https://api.openai.com"
  api_type: null
  max_tokens: null

security:
  mode: "strict"
  block_patterns:
    - "rm"
    - "mkfs"
    - "mkfs.ext4"
    - "dd"
    - "wipefs"
    - "fdisk"
    - "parted"
    - "sfdisk"
    - "shred"
    - "-delete"
    - "> /dev/"
    - "cfdisk"
    - "gdisk"
    - "sgdisk"
    - "blkdiscard"
    - "halt"
    - "killall"
    - "iptables -F"
    - "--no-preserve-root"
    - "-exec"
    - "apt remove"
    - "apt purge"
    - "| sh"
    - "| bash"
    - "chmod -R 777 /"
    - "shutdown"
    - "reboot"
    - "poweroff"
    - "init 0"
    - "init 1"
    - "init 6"
    - ":(){:|:&};:"

ui:
  position: "top"
  success_duration: 2600
```

---

## 11. 依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| `clap` | 4 (derive) | CLI 参数解析 |
| `serde` | 1 (derive) | 序列化/反序列化 |
| `serde_json` | 1 | JSON 处理 (API 请求/响应) |
| `serde_yaml` | 0.9 | YAML 处理 (配置文件) |
| `reqwest` | 0.12 (blocking, rustls-tls) | HTTP 客户端 |
| `dirs` | 5 | 系统目录路径 (home, config) |
| `chrono` | 0.4 | 时间处理 |
| `tempfile` | 3 (dev) | 测试用临时文件 |

---

## 12. 常见问题

### Q: 编译时出现 OpenSSL 相关错误？

本项目使用 `rustls-tls`，不依赖系统 OpenSSL。如果遇到问题，确认 `Cargo.toml` 中 `reqwest` 的 `default-features = false` 已设置。

### Q: 如何调试 LLM 请求？

在运行时可以查看 stderr 输出中的错误信息。如需更详细的调试，可以在对应 Provider 的 `chat_completion()` 方法中临时添加 `eprintln!` 输出请求/响应内容。

### Q: 如何更换默认的 LLM 模型？

编辑 `~/.shellmate/config.yaml`，修改 `llm.model` 字段。或运行 `shellmate install` 重新配置。

### Q: 如何禁用安全检查？

不推荐。但可以在 `config.yaml` 中将 `security.block_patterns` 设为空数组 `[]`。
