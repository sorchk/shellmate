[English](../../en/GUIDE.md) | [中文](../../zh/GUIDE.md) | [← README](../../README.md)

---

# ShellMate 使用指南

## 1. 简介

ShellMate 是一个 AI 驱动的命令行助手，帮助你在终端中用自然语言生成 Shell 命令。只需输入描述，ShellMate 就会调用 AI 模型为你生成合适的命令。

### 功能亮点

- **自然语言生成命令** — 输入 "列出所有文件" 自动生成 `ls -la`
- **安全拦截** — 自动阻止 `rm -rf /`、`mkfs`、`dd` 等危险命令
- **多 AI 支持** — OpenAI、Ollama、Anthropic、Gemini、自定义 Provider
- **Shell 集成** — 支持 Bash、Zsh、Sh、Fish
- **触发方式** — 前缀触发 (`@ai`/`#ai`/`/ai`) 或快捷键 (`Ctrl+G`)

---

## 2. 安装

### 方式一：远程一键安装（推荐）

```bash
curl -fsSL https://shai.apix.fun | sh
```

安装脚本自动完成：
1. 下载对应平台的预编译二进制文件（Linux amd64/arm64、macOS arm64）
2. 将二进制安装到 `~/.shellmate/bin/`
3. 配置 PATH 环境变量
4. 安装 Shell 集成脚本
5. 引导完成 AI Provider 配置

安装完成后重启终端或执行：

```bash
source ~/.bashrc      # Bash 用户
source ~/.zshrc       # Zsh 用户
```

### 方式二：手动编译安装

**前置要求：** 安装 [Rust](https://rustup.rs/)

```bash
# 克隆项目
git clone https://github.com/sorchk/shellmate.git
cd shellmate

# 编译
cargo build --release

# 复制到 PATH 中
mkdir -p ~/.shellmate/bin
cp target/release/shellmate ~/.shellmate/bin/
```

将以下内容添加到你的 Shell 配置文件中：

```bash
# 添加到 ~/.bashrc 或 ~/.zshrc
export PATH="$HOME/.shellmate/bin:$PATH"
```

### 方式三：Cargo Install

```bash
cargo install --path /path/to/shellmate
```

这将把 `shellmate` 安装到 `~/.cargo/bin/shellmate`。

---

## 3. Shell 集成

Shell 集成让你可以直接在终端中使用触发前缀或快捷键调用 ShellMate。

### 3.1 使用 `shellmate install` 命令

```bash
# 自动检测当前 Shell
shellmate install

# 指定 Shell 类型
shellmate install --shell bash
shellmate install --shell zsh

# 仅配置 AI Provider（跳过 Shell 集成）
shellmate install --config-only
```

该命令会：
1. 显示 Shell 类型和 RC 文件路径
2. 引导配置 AI Provider
3. 提示你需要手动添加的 `source` 行

### 3.2 手动配置 Bash

编辑 `~/.bashrc`（macOS 为 `~/.bash_profile`），添加：

```bash
source "$HOME/.shellmate/shell/shellmate.bash"
```

### 3.3 手动配置 Zsh

编辑 `~/.zshrc`，添加：

```bash
source "$HOME/.shellmate/shell/shellmate.zsh"
```

### 3.4 手动配置 Sh

编辑 `~/.profile`，添加：

```bash
. "$HOME/.shellmate/shell/shellmate.sh"
```

### 3.5 配置完成后

重新加载 Shell 配置：

```bash
source ~/.bashrc    # Bash
source ~/.zshrc     # Zsh
```

---

## 4. AI Provider 配置

ShellMate 需要配置一个 AI Provider 才能使用。配置文件位于 `~/.shellmate/config.yaml`。

### 4.1 交互式配置

运行安装命令会引导你完成配置：

```bash
shellmate install
```

按照提示选择 Provider、输入 API Key 和 Model 名称。

### 4.2 手动编辑配置文件

编辑 `~/.shellmate/config.yaml`：

#### OpenAI

```yaml
llm:
  provider: "openai"
  model: "gpt-4-turbo"
  api_key: "sk-xxxxxxxxxxxxxxxx"
  base_url: "https://api.openai.com"
  timeout: 30
```

#### OpenAI 兼容服务（如 DeepSeek、Moonshot 等）

```yaml
llm:
  provider: "openai"
  model: "deepseek-chat"
  api_key: "your-api-key"
  base_url: "https://api.deepseek.com"
  api_type: "openai-completions"
  timeout: 30
```

#### Ollama（本地模型，无需 API Key）

```yaml
llm:
  provider: "ollama"
  model: "qwen3.5:4b"
  base_url: "http://localhost:11434"
  timeout: 60
```

> 使用 Ollama 前请确保 Ollama 已启动并拉取了对应模型：
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

#### 自定义 Provider

```yaml
llm:
  provider: "my-provider"
  model: "my-model"
  api_key: "your-api-key"
  base_url: "https://api.example.com"
  api_type: "openai-completions"
  timeout: 30
```

可选的 `api_type` 值：
- `openai-completions` — OpenAI Chat Completions API
- `openai-responses` — OpenAI Responses API
- `anthropic-messages` — Anthropic Messages API
- `gemini-generate-content` — Gemini GenerateContent API
- `custom` — 使用 `base_url` 作为完整的 endpoint 地址

---

## 5. 使用方法

### 5.1 前缀触发

在终端中输入触发前缀 + 自然语言描述：

```bash
$ @ai 列出当前目录所有文件
ls -la

$ @ai 查找所有 .rs 文件
find . -name "*.rs" -type f

$ @ai 统计 src 目录下的文件数量
ls -1 src | wc -l
```

支持的前缀：`@ai`、`#ai`、`/ai`

### 5.2 快捷键触发

1. 在终端中输入自然语言描述（不带前缀）
2. 按 `Ctrl+G`（默认，可在 `~/.shellmate/config.yaml` 中配置）
3. ShellMate 会自动添加前缀并处理

修改快捷键：编辑配置文件中的 `trigger.shortcut`，然后运行 `shellmate install --shell <bash|zsh>` 生效。支持格式：`Ctrl+A`–`Ctrl+Z`、`Alt+A`–`Alt+Z`。

### 5.3 命令确认机制

生成的命令会显示在终端上，等待你的操作：

- **按 Enter** — 执行命令（显示绿色 ✓）
- **按 ESC** — 取消命令（显示红色 ✗）

### 5.4 CLI 直接使用

即使没有 Shell 集成，也可以直接使用命令行：

```bash
# 生成命令
shellmate generate "列出所有文件" --shell bash

# 安全检查
shellmate check "rm -rf /"
# 输出: BLOCKED: rm -rf /

shellmate check "ls -la"
# 输出: PASS: command is safe

# 查看当前配置
shellmate config

# 安装 Shell 集成
shellmate install --shell zsh
```

### 5.5 安全拦截

当 AI 生成了危险命令时，ShellMate 会自动拦截并提示：

```
⚠ 已拦截风险命令 (耗时:2.1s, Tokens:156, 首字延时:2100ms, 每秒:74.3 tokens)
  rm -rf /tmp/important
```

默认拦截的命令模式包括：`rm`、`mkfs`、`dd`、`fdisk`、`parted`、`shred`、`| sh`、`| bash`、`chmod -R 777 /`、`killall`、`iptables -F`、`-delete`、`-exec`、`--no-preserve-root`、`> /dev/`、`apt remove`、`apt purge`、`halt`、`shutdown`、`reboot`、`poweroff`、`init 0/1/6`、Fork 炸弹等。

---

## 6. 配置参考

完整的 `~/.shellmate/config.yaml` 配置：

```yaml
trigger:
  prefixes:
    - "@ai"
    - "#ai"
    - "/ai"
  shortcut: "Ctrl+G"

llm:
  provider: "openai"         # Provider 名称
  model: "gpt-4-turbo"       # 模型名称
  timeout: 30                # 请求超时（秒）
  api_key: "sk-..."          # API Key（可选，Ollama 不需要）
  base_url: "https://api.openai.com"  # API Base URL
  api_type: null             # API 类型（可选）
  max_tokens: null           # 最大生成 Token 数（可选）

security:
  mode: "strict"             # 安全模式
  block_patterns:            # 拦截关键词列表
    - "rm"
    - "mkfs"
    # ... （见下方完整列表）

ui:
  position: "top"            # 提示位置
  success_duration: 2600     # 成功消息持续时间（ms）
```

### 配置项说明

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `trigger.prefixes` | `["@ai", "#ai", "/ai"]` | 触发前缀列表 |
| `trigger.shortcut` | `Ctrl+G` | 快捷键（格式：`Ctrl+<Key>` 或 `Alt+<Key>`） |
| `llm.provider` | `openai` | AI Provider |
| `llm.model` | `gpt-4-turbo` | 模型名称 |
| `llm.timeout` | `30` | 请求超时（秒） |
| `llm.api_key` | `null` | API Key |
| `llm.base_url` | `null` | API Base URL |
| `llm.api_type` | `null` | API 类型 |
| `llm.max_tokens` | `null` | 最大生成 Token |
| `security.mode` | `strict` | 安全模式 |
| `security.block_patterns` | （见下方） | 危险命令模式 |
| `ui.position` | `top` | 提示位置 |
| `ui.success_duration` | `2600` | 成功消息持续时间(ms) |

---

## 7. 使用示例

### 文件操作

```bash
$ @ai 查找大于100MB的文件
find . -type f -size +100M

$ @ai 统计 src 目录下的代码行数
find src -name "*.rs" | xargs wc -l
```

### 系统管理

```bash
$ @ai 查看当前占用端口 8080 的进程
lsof -i :8080

$ @ai 查看磁盘使用情况
df -h

$ @ai 查看 Docker 容器日志
docker logs <container_id> --tail 100
```

### Git 操作

```bash
$ @ai 查看最近5条提交记录
git log --oneline -5

$ @ai 撤销最后一次提交（保留修改）
git reset --soft HEAD~1

$ @ai 创建并切换到新分支
git checkout -b feature/new-feature
```

### 网络工具

```bash
$ @ai 查看本机 IP 地址
ip addr show | grep "inet "

$ @ai 测试到 google.com 的网络连通性
ping -c 4 google.com
```

---

## 8. 退出码说明

| 退出码 | 含义 |
|--------|------|
| `0` | 成功 |
| `1` | 错误（配置错误、API 错误等） |
| `2` | 命令被安全检查拦截 |

---

## 9. 常见问题

### Q: 安装后命令找不到？

确保 `~/.shellmate/bin/` 在 PATH 中：

```bash
echo $PATH | grep shellmate
```

如果没有，手动添加到 Shell 配置文件：

```bash
export PATH="$HOME/.shellmate/bin:$PATH"
```

### Q: 触发前缀没有反应？

检查 Shell 集成是否已加载：

```bash
grep shellmate ~/.bashrc ~/.zshrc
```

确保已执行 `source ~/.bashrc` 或 `source ~/.zshrc`。

### Q: API 调用超时？

编辑 `~/.shellmate/config.yaml`，增大 `timeout` 值：

```yaml
llm:
  timeout: 60
```

### Q: 使用 Ollama 时连接失败？

1. 确保 Ollama 正在运行：`ollama serve`
2. 确保 base_url 正确：`http://localhost:11434`
3. 确保已拉取模型：`ollama pull qwen3.5:4b`

### Q: 如何修改快捷键？

编辑 `~/.shellmate/config.yaml`：

```yaml
trigger:
  shortcut: "Alt+X"   # 支持 Ctrl+A-Z, Alt+A-Z
```

然后重新安装 Shell 集成以生效：

```bash
shellmate install --shell bash   # 或 zsh
```

重启终端或执行 `source ~/.bashrc`（或 `~/.zshrc`）。

### Q: 如何修改触发前缀？

编辑 `~/.shellmate/config.yaml`：

```yaml
trigger:
  prefixes:
    - "@ai"
    - "hey"
```

同时需要修改对应的 Shell 集成脚本中的 `prefixes` 变量。

### Q: 如何暂时禁用 ShellMate？

移除 Shell 配置文件中的 `source .../shellmate.*` 行，然后重新加载配置即可。

### Q: 生成命令后不等待确认直接执行了？

这通常发生在不支持 `read -rsn1` 的 Shell 环境中。确保使用的是 Bash 4+ 或 Zsh。
