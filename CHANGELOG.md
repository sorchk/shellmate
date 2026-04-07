# Changelog

All notable changes to ShellMate are documented here.

---

## 2026-04-06

### ✨ Features

- feat: add remote install script for curl-based installation (335f931)

### 🐛 Bug Fixes

- fix: 安装脚本使用 --config-only 避免重复提问 (03a0e16)
- fix: 修复安装时重复提问配置问题 (722cbe6)
- fix:修复多平台的兼容性 (45c2193)
- fix:remote-install.sh (50d6c67)
- fix: sh install (b25c90f)
- fix: sh: 2: set: Illegal option -o pipefail (abe607d)
- fix: remove macOS amd64 build (macos-13 runner unavailable) (9364c5a)

### 🔧 CI/Build

- release: bump version to 0.1.6 (f82011f)
- release: bump version to 0.1.5 (85f9fdc)
- release: bump version to 0.1.1 (f096d25)

### 🔄 Other Changes

- 使用静态 musl 目标 进行交叉编译 (2b692d7)
- 添加更新配置选择 (6d9cbb7)
- chore: remove .opencode from git tracking (0be2060)
- chore: track Cargo.lock in version control (98d9c16)
- 添加cicd (67f52dc)

**Commits:** 16 | **Contributors:** sorc

---


