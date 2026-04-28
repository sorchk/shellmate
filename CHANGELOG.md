# Changelog

All notable changes to ShellMate are documented here.

---

## 2026-04-27

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-26 (7f73a1a)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-26

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-25 (bc36a0f)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-25

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-24 (e489597)

### 🔄 Other Changes

- 修改仅支持@ai和快捷键激活AI (11ed0b2)

**Commits:** 2 | **Contributors:** github-actions[bot],sorc

---


## 2026-04-24

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-23 (1f0061b)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-23

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-22 (f5ec989)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-22

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-21 (d066ace)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-21

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-20 (fe0274b)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-20

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-19 (51e67cc)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-19

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-18 (3ab1d95)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-18

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-17 (7f9f99b)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-17

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-16 (db10daa)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-16

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-15 (9571c97)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-15

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-14 (de891d6)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-14

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-13 (96ef2cd)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-13

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-12 (efb0e2e)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-12

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-11 (e55e3e7)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-11

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-10 (dd2fca6)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-10

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-09 (a0494c6)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-09

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-08 (c91fffa)

**Commits:** 1 | **Contributors:** github-actions[bot]

---


## 2026-04-08

### ✨ Features

- feat: configurable keyboard shortcut from config.yaml (8a9ad78)

### 🐛 Bug Fixes

- fix: 修复 Alt 键绑定格式和安装流程顺序 (4bbfa19)

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-07 (7512e68)

### 🔧 CI/Build

- release: bump version to 0.1.8 (cc77414)
- release: bump version to 0.1.7 (f420cd5)

### 🔄 Other Changes

- 修复小的文字错误 (0338b8d)

**Commits:** 6 | **Contributors:** github-actions[bot],sorc

---


## 2026-04-08 v0.1.8

1. 发布(release): 发布 v0.1.8 版本 / Release v0.1.8
2. 修复(fix): 修复 Alt 键绑定格式（bash: \M- → \e, zsh: ^[[ → ^[）/ Fix Alt key binding format for bash and zsh
3. 修复(fix): 调整安装流程顺序，先完成配置再设置 shell 集成 / Adjust install flow order: configure before shell integration setup
4. 修复(fix): 修复小的文字错误 / Fix minor text errors

---

## 2026-04-08 v0.1.7

1. 发布(release): 发布 v0.1.7 版本 / Release v0.1.7
2. 新增(feature): 快捷键可配置 — 支持 Ctrl+Key / Alt+Key 格式，修改 config.yaml 后重新 install 即可生效 / Configurable keyboard shortcut — supports Ctrl+Key / Alt+Key formats, apply by reinstalling after editing config.yaml
3. 其他(chore): 新增 shortcut.rs 模块和 15 个测试用例 / Add shortcut.rs module with 15 test cases
4. 其他(chore): 文档重组为 en/zh 双语子目录，新增快捷键配置说明 / Restructure docs into en/zh bilingual subdirectories, add shortcut config docs

---

## 2026-04-07

### 📝 Documentation

- docs: update CHANGELOG.md for 2026-04-06 (8a56e9a)

**Commits:** 1 | **Contributors:** github-actions[bot]

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
