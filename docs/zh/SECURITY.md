[English](../../en/SECURITY.md) | [中文](../../zh/SECURITY.md) | [← README](../../README.md)

---

# ShellMate 安全文档

## 1. 简介

本文档介绍 ShellMate 的安全模型、潜在风险和安全使用指南。ShellMate 以安全为核心设计原则——每个 AI 生成的命令在执行前都会经过基于关键词的安全检查器。

## 2. 安全模型概述

ShellMate 采用纵深防御策略：

1. **AI 输入清洗** — 用户提示在发送到 LLM 提供商前进行清洗
2. **LLM 输出清洗** — 生成的命令被去除 Markdown 格式、注释和代码围栏
3. **基于关键词的安全检查** — 每个生成的命令都会与可配置的危险模式黑名单进行匹配
4. **用户确认机制** — 生成的命令需要用户明确批准（按 Enter）后才能执行

### 架构流程

```
用户输入 → 输入清洗 → LLM 调用 → 输出清洗 → 安全检查 → 用户确认 → 执行
```

## 3. 潜在风险与缓解措施

### 3.1 AI 生成的危险命令

**风险：** LLM 可能通过混淆、编码或创造性语法生成绕过关键词黑名单的命令。

**缓解措施：**
- 关键词黑名单覆盖常见危险操作
- 用户必须在执行前确认每条命令
- 黑名单可在 `config.yaml` 中完全自定义

**建议：** 在按 Enter 执行前始终审查生成的命令。安全检查器是安全网，不能替代用户判断。

### 3.2 API Key 暴露

**风险：** 存储在 `~/.shellmate/config.yaml` 中的 API Key 可能被系统上的其他用户或进程访问。

**缓解措施：**
- 配置文件存储在用户主目录（`~/.shellmate/`）
- API Key 永远不会被记录或打印到 stdout/stderr
- 显示配置时，API Key 会被掩码处理（如 `sk-abc...xyz`）

**建议：**
- 设置严格的文件权限：`chmod 600 ~/.shellmate/config.yaml`
- 不要将配置文件提交到版本控制
- 尽可能使用本地模型（Ollama）避免使用 API Key

### 3.3 Shell 集成脚本注入

**风险：** Shell 集成脚本（`shellmate.bash`、`shellmate.zsh`）修改 Shell 行为，可能被篡改。

**缓解措施：**
- 脚本与二进制文件一起打包，在安装时写入 `~/.shellmate/shell/`
- 源行带有清晰注释追加到 Shell RC 文件

**建议：** 定期检查 `~/.shellmate/shell/` 脚本内容和你的 Shell RC 文件。

### 3.4 网络通信

**风险：** 向 LLM 提供商发送的 API 请求会通过网络传输用户提示。

**缓解措施：**
- 所有 API 连接使用 HTTPS（TLS）
- `reqwest` 配置为 `rustls-tls` 确保安全传输
- ShellMate 本身不存储或缓存任何数据

**建议：** 为获得最大隐私，使用 Ollama 本地模型——数据不会离开你的机器。

### 3.5 命令历史暴露

**风险：** ShellMate 读取 Shell 历史文件以提供上下文，其中可能包含敏感信息（密码、令牌等）。

**缓解措施：**
- 历史仅用于为命令生成提供上下文
- 历史内容作为提示的一部分发送给 LLM 提供商

**建议：** 注意你最近的 Shell 历史会被包含在发送给 LLM 的上下文中。如有需要，清理历史中的敏感条目。

## 4. 安全使用指南

### 4.1 配置最佳实践

- 对配置文件设置严格权限：`chmod 600 ~/.shellmate/config.yaml`
- 永远不要公开分享你的配置文件
- 尽可能使用环境变量存储 API Key
- 定期轮换 API Key

### 4.2 环境安全

- 不要以 root 或提权权限运行 ShellMate
- 保持二进制文件更新以获取安全修复
- 下载预编译版本时验证二进制校验和

### 4.3 自定义安全黑名单

默认黑名单覆盖常见危险操作。根据你的环境进行自定义：

```yaml
security:
  mode: strict
  block_patterns:
    - "rm"
    - "mkfs"
    # 添加你自己的模式
    - "your-dangerous-pattern"
```

模式作为子字符串与生成的命令进行匹配。保持具体以避免误报。

### 4.4 使用本地模型

为获得最大安全性和隐私，使用 Ollama 本地模型：

```yaml
llm:
  provider: ollama
  model: qwen3.5:4b
  base_url: http://localhost:11434
  api_key: null
```

这确保没有数据被传输到外部服务。

## 5. 报告漏洞

如果你发现 ShellMate 中的安全漏洞，请负责任地报告：

1. **不要**为安全漏洞公开提交 Issue
2. 通过 [GitHub Security Advisories](https://github.com/sorchk/shellmate/security/advisories) 联系维护者
3. 包含漏洞的详细描述和复现步骤
4. 在修复发布前给予合理的处理时间

## 6. 安全更新策略

- 安全修复作为补丁版本发布（如 `0.1.6` → `0.1.7`）
- 关键安全修复可能作为热修复发布
- 建议用户保持 ShellMate 更新
- 安全公告在 GitHub 上发布

## 7. 致谢

感谢所有帮助识别和报告安全问题的贡献者和用户。
