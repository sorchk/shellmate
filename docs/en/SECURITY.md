[English](../../en/SECURITY.md) | [中文](../../zh/SECURITY.md) | [← README](../../README.md)

---

# ShellMate Security Policy

## 1. Introduction

This document describes ShellMate's security model, potential risks, and safe usage guidelines. ShellMate is designed with security as a core principle — every AI-generated command pass through a keyword-based security checker before execution.

## 2. Security Model Overview

ShellMate employs a defense-in-depth approach:

1. **AI Input Sanitization** — User prompts are sanitized before sending to LLM providers
2. **LLM Output Sanitization** — Generated commands are stripped of markdown formatting, comments, and code fences
3. **Keyword-Based Security Checking** — Every generated command is matched against a configurable blocklist of dangerous patterns
4. **User Confirmation** — Generated commands require explicit user approval (Enter) before execution

### Architecture Flow

```
User Input → Sanitize → LLM Call → Sanitize Output → Security Check → User Confirm → Execute
```

## 3. Potential Risks and Mitigations

### 3.1 AI-Generated Dangerous Commands

**Risk:** The LLM might generate commands that bypass the keyword blocklist through obfuscation, encoding, or creative syntax.

**Mitigation:**
- Keyword-based blocklist covers common dangerous operations
- Users must confirm every command before execution
- Blocklist is fully customizable in `config.yaml`

**Recommended Action:** Always review generated commands before pressing Enter. The security checker is a safety net, not a replacement for user judgment.

### 3.2 API Key Exposure

**Risk:** API keys stored in `~/.shellmate/config.yaml` could be accessed by other users or processes on the system.

**Mitigation:**
- Config file is stored in user's home directory (`~/.shellmate/`)
- API keys are never logged or printed to stdout/stderr
- When displaying config, API keys are masked (e.g., `sk-abc...xyz`)

**Recommended Action:**
- Set restrictive file permissions: `chmod 600 ~/.shellmate/config.yaml`
- Do not commit config files to version control
- Use local models (Ollama) when possible to avoid API key usage

### 3.3 Shell Integration Script Injection

**Risk:** The shell integration scripts (`shellmate.bash`, `shellmate.zsh`) modify shell behavior and could potentially be tampered with.

**Mitigation:**
- Scripts are bundled with the binary and written to `~/.shellmate/shell/` during installation
- Source lines are appended to shell RC files with clear comments

**Recommended Action:** Periodically review the contents of `~/.shellmate/shell/` scripts and your shell RC files.

### 3.4 Network Communication

**Risk:** API requests to LLM providers transmit user prompts over the network.

**Mitigation:**
- All API connections use HTTPS (TLS)
- `reqwest` is configured with `rustls-tls` for secure transport
- No data is stored or cached by ShellMate itself

**Recommended Action:** For maximum privacy, use local models via Ollama — no data leaves your machine.

### 3.5 Command History Exposure

**Risk:** ShellMate reads shell history files to provide context, which may contain sensitive information (passwords, tokens, etc.).

**Mitigation:**
- History is only read to provide context for command generation
- History content is sent to the LLM provider as part of the prompt

**Recommended Action:** Be aware that your recent shell history is included in the context sent to the LLM. Clean sensitive entries from history if needed.

## 4. Safe Usage Guidelines

### 4.1 Configuration Best Practices

- Set restrictive permissions on config file: `chmod 600 ~/.shellmate/config.yaml`
- Never share your config file publicly
- Use environment variables for API keys when possible
- Regularly rotate API keys

### 4.2 Environment Security

- Do not run ShellMate as root or with elevated privileges
- Keep the binary updated to benefit from security fixes
- Verify binary checksums when downloading pre-built releases

### 4.3 Customizing the Security Blocklist

The default blocklist covers common dangerous operations. Customize it for your environment:

```yaml
security:
  mode: strict
  block_patterns:
    - "rm"
    - "mkfs"
    # Add your own patterns
    - "your-dangerous-pattern"
```

Patterns are matched as substrings against the generated command. Be specific to avoid false positives.

### 4.4 Using Local Models

For maximum security and privacy, use Ollama with local models:

```yaml
llm:
  provider: ollama
  model: qwen3.5:4b
  base_url: http://localhost:11434
  api_key: null
```

This ensures no data is transmitted to external services.

## 5. Reporting Vulnerabilities

If you discover a security vulnerability in ShellMate, please report it responsibly:

1. **Do NOT** open a public issue for security vulnerabilities
2. Contact the maintainer directly through [GitHub Security Advisories](https://github.com/sorchk/shellmate/security/advisories)
3. Include a detailed description of the vulnerability and steps to reproduce
4. Allow reasonable time for a fix before public disclosure

## 6. Security Updates Policy

- Security fixes are released as patch versions (e.g., `0.1.6` → `0.1.7`)
- Critical security fixes may be released as hotfixes
- Users are encouraged to keep ShellMate updated
- Security advisories are published on GitHub

## 7. Acknowledgments

Thanks to all contributors and users who help identify and report security issues.
