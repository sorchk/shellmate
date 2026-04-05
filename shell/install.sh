#!/usr/bin/env bash
set -euo pipefail

SHELLMATE_DIR="$HOME/.shellmate"
BIN_DIR="$SHELLMATE_DIR/bin"
SHELL_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SHELL_DIR/.." && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; exit 1; }

detect_os() {
    local os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    case "$os" in
        linux) echo "linux" ;;
        darwin) echo "macos" ;;
        *) echo "$os" ;;
    esac
}

detect_arch() {
    local arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) echo "$arch" ;;
    esac
}

detect_shell() {
    local shell_name="${SHELL:-}"
    shell_name="${shell_name##*/}"
    case "$shell_name" in
        bash|zsh|fish) echo "$shell_name" ;;
        dash|ash|sh)   echo "sh" ;;
        *)             echo "bash" ;;
    esac
}

detect_shell_rc() {
    local os="$(uname -s)"
    case "$(detect_shell)" in
        bash)
            if [[ "$os" == "Darwin" ]]; then
                echo "$HOME/.bash_profile"
            else
                echo "$HOME/.bashrc"
            fi
            ;;
        zsh)  echo "$HOME/.zshrc" ;;
        sh)   echo "$HOME/.profile" ;;
        fish) echo "$HOME/.config/fish/config.fish" ;;
        *)    echo "$HOME/.bashrc" ;;
    esac
}

build_shellmate() {
    info "Building ShellMate..."

    if ! command -v cargo &>/dev/null; then
        error "cargo not found. Please install Rust: https://rustup.rs/"
    fi

    export PATH="$HOME/.cargo/bin:$PATH"

    (cd "$REPO_DIR" && cargo build --release 2>&1) || error "Build failed"

    mkdir -p "$BIN_DIR"
    cp "$REPO_DIR/target/release/shellmate" "$BIN_DIR/shellmate"
    chmod +x "$BIN_DIR/shellmate"

    info "Installed binary to $BIN_DIR/shellmate"
}

setup_path() {
    local shell_rc="$1"
    local path_line='export PATH="$HOME/.shellmate/bin:$PATH"'

    if grep -qF '.shellmate/bin' "$shell_rc" 2>/dev/null; then
        return
    fi

    echo '' >> "$shell_rc"
    echo '# ShellMate' >> "$shell_rc"
    echo "$path_line" >> "$shell_rc"
    info "Added PATH to $shell_rc"
}

setup_integration() {
    local shell_type="$1"
    local shell_rc
    local os="$(uname -s)"

    case "$shell_type" in
        bash)
            if [[ "$os" == "Darwin" ]]; then
                shell_rc="$HOME/.bash_profile"
            else
                shell_rc="$HOME/.bashrc"
            fi
            ;;
        zsh)   shell_rc="$HOME/.zshrc" ;;
        sh)    shell_rc="$HOME/.profile" ;;
        fish)  shell_rc="$HOME/.config/fish/config.fish" ;;
        *)     error "Unsupported shell: $shell_type" ;;
    esac

    local integration_file="$SHELL_DIR/shellmate.$shell_type"
    if [[ ! -f "$integration_file" ]]; then
        warn "Integration file not found: $integration_file"
        return
    fi

    local source_line="source \"$integration_file\""

    if grep -qF "shellmate.$shell_type" "$shell_rc" 2>/dev/null; then
        info "Shell integration already configured in $shell_rc"
        return
    fi

    echo "$source_line" >> "$shell_rc"
    info "Added shell integration to $shell_rc"
}

setup_config() {
    "$BIN_DIR/shellmate" install --shell "$(detect_shell)"
}

main() {
    local shell_rc

    echo ""
    echo "╔══════════════════════════════════════╗"
    echo "║       ShellMate Installer            ║"
    echo "╚══════════════════════════════════════╝"
    echo ""

    info "OS: $(detect_os), Arch: $(detect_arch), Shell: $(detect_shell)"

    build_shellmate

    shell_rc="$(detect_shell_rc)"
    setup_path "$shell_rc"
    setup_integration "$(detect_shell)"
    setup_config

    echo ""
    info "Installation complete!"
    echo ""
    info "Next steps:"
    echo "  1. Restart your terminal or run: source ${shell_rc/#$HOME\//~/}"
    echo "  2. Edit ~/.shellmate/config.yaml to set your API key"
    echo "  3. Try: @ai list all files in current directory"
    echo "  4. Or type a description and press Ctrl+G"
    echo ""
}

main
