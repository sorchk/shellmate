#!/usr/bin/env sh
set -eu

SHELLMATE_DIR="$HOME/.shellmate"
BIN_DIR="$SHELLMATE_DIR/bin"
SHELL_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SHELL_DIR/.." && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()  { printf '%b\n' "${GREEN}[INFO]${NC} $*"; }
warn()  { printf '%b\n' "${YELLOW}[WARN]${NC} $*"; }
error() { printf '%b\n' "${RED}[ERROR]${NC} $*" >&2; exit 1; }

detect_os() {
    case "$(uname -s | tr '[:upper:]' '[:lower:]')" in
        linux*)  echo "linux" ;;
        darwin*) echo "darwin" ;;
        *)       error "Unsupported OS: $(uname -s)" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "amd64" ;;
        aarch64|arm64) echo "arm64" ;;
        *)             error "Unsupported architecture: $(uname -m)" ;;
    esac
}

detect_shell_type() {
    _sm_shell_name="${SHELL:-}"
    _sm_shell_name="${_sm_shell_name##*/}"
    case "$_sm_shell_name" in
        bash|zsh|fish) echo "$_sm_shell_name" ;;
        dash|ash|sh)   echo "sh" ;;
        *)             echo "bash" ;;
    esac
}

detect_shell_rc() {
    _sm_os="$(uname -s)"
    case "$(detect_shell_type)" in
        bash)
            if [ "$_sm_os" = "Darwin" ]; then
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

    if ! command -v cargo >/dev/null 2>&1; then
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
    _sm_rc="$1"
    _sm_path_line='export PATH="$HOME/.shellmate/bin:$PATH"'

    if grep -qF '.shellmate/bin' "$_sm_rc" 2>/dev/null; then
        return
    fi

    mkdir -p "$(dirname "$_sm_rc")"
    printf '\n' >> "$_sm_rc"
    printf '%s\n' '# ShellMate' >> "$_sm_rc"
    printf '%s\n' "$_sm_path_line" >> "$_sm_rc"
    info "Added PATH to $_sm_rc"
}

get_installed_version() {
    if [ -x "$BIN_DIR/shellmate" ]; then
        "$BIN_DIR/shellmate" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1
    fi
}

main() {
    printf '\n'
    printf '%b\n' "${CYAN}╔══════════════════════════════════════╗"
    printf '%b\n' "║       ShellMate Installer            ║"
    printf '%b\n' "╚══════════════════════════════════════╝${NC}"
    printf '\n'

    _sm_os="$(detect_os)"
    _sm_arch="$(detect_arch)"
    _sm_shell_type="$(detect_shell_type)"
    _sm_shell_rc="$(detect_shell_rc)"

    info "OS: $_sm_os  Arch: $_sm_arch  Shell: $_sm_shell_type"

    _sm_installed_version="$(get_installed_version)"

    if [ -n "$_sm_installed_version" ]; then
        info "Updating ShellMate (current: v$_sm_installed_version)..."
    else
        info "Installing ShellMate..."
    fi

    build_shellmate

    _sm_new_version="$("$BIN_DIR/shellmate" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)"

    setup_path "$_sm_shell_rc"

    "$BIN_DIR/shellmate" install --shell "$_sm_shell_type" </dev/tty

    printf '\n'
    _sm_display_rc="$_sm_shell_rc"
    case "$_sm_shell_rc" in
        "$HOME"/*) _sm_display_rc="~/${_sm_shell_rc#"$HOME"/}" ;;
    esac

    if [ -n "$_sm_installed_version" ] && [ "$_sm_installed_version" != "$_sm_new_version" ]; then
        info "ShellMate updated from v$_sm_installed_version to v$_sm_new_version!"
    elif [ -n "$_sm_installed_version" ] && [ "$_sm_installed_version" = "$_sm_new_version" ]; then
        info "ShellMate reinstalled (v$_sm_new_version)."
    else
        info "ShellMate v$_sm_new_version installed successfully!"
    fi

    echo ""
    info "Next steps:"
    echo "  1. Restart your terminal or run: source $_sm_display_rc"
    echo "  2. Try: @ai list all files in current directory"
    echo "  3. Or type a description and press Ctrl+G"
    echo ""
}

main
