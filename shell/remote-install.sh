#!/usr/bin/env sh
set -eu

REPO="sorchk/shellmate"
BIN_NAME="shellmate"
INSTALL_DIR="$HOME/.shellmate/bin"

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

http_get() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$1"
    else
        error "Neither curl nor wget found. Please install one and retry."
    fi
}

http_download() {
    _sm_url="$1"
    _sm_output="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fSL -o "$_sm_output" "$_sm_url"
    else
        wget -qO "$_sm_output" "$_sm_url"
    fi
}

get_latest_version() {
    _sm_api_url="https://api.github.com/repos/${REPO}/releases/latest"
    http_get "$_sm_api_url" | grep '"tag_name"' | head -1 | sed -E 's/.*"([^"]+)".*/\1/'
}

sha256_hash() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | awk '{print $1}'
    elif command -v openssl >/dev/null 2>&1; then
        openssl dgst -sha256 "$1" | awk '{print $NF}'
    else
        return 1
    fi
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

    info "Fetching latest version..."
    _sm_version="$(get_latest_version)"
    if [ -z "$_sm_version" ]; then
        error "Failed to determine latest version. Check your internet connection."
    fi
    _sm_latest_num="${_sm_version#v}"
    info "Latest version: $_sm_version"

    _sm_action="install"
    _sm_installed_version=""
    if [ -x "$INSTALL_DIR/$BIN_NAME" ]; then
        _sm_installed_version="$("$INSTALL_DIR/$BIN_NAME" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)"
    fi

    if [ -n "$_sm_installed_version" ] && [ "$_sm_installed_version" = "$_sm_latest_num" ]; then
        info "ShellMate $_sm_installed_version is already installed and up to date."
        _sm_action="skip"
    elif [ -n "$_sm_installed_version" ]; then
        info "Updating ShellMate from $_sm_installed_version to $_sm_latest_num..."
        _sm_action="update"
    else
        info "Installing ShellMate $_sm_latest_num..."
    fi

    if [ "$_sm_action" != "skip" ]; then
        _sm_archive_name="shellmate_${_sm_version}_${_sm_os}_${_sm_arch}.tar.gz"
        _sm_download_url="https://github.com/${REPO}/releases/download/${_sm_version}/${_sm_archive_name}"

        _sm_tmp_dir="$(mktemp -d)"
        trap 'rm -rf "$_sm_tmp_dir"' EXIT

        info "Downloading $_sm_archive_name..."
        http_download "$_sm_download_url" "$_sm_tmp_dir/$_sm_archive_name" || {
            error "Download failed. The archive for your platform ($_sm_os/$_sm_arch) may not exist.\n  URL: $_sm_download_url"
        }

        _sm_checksum_url="https://github.com/${REPO}/releases/download/${_sm_version}/sha256sums.txt"
        if http_get "$_sm_checksum_url" > "$_sm_tmp_dir/sha256sums.txt" 2>/dev/null; then
            info "Verifying checksum..."
            _sm_expected="$(grep "$_sm_archive_name" "$_sm_tmp_dir/sha256sums.txt" | awk '{print $1}')"
            if [ -n "$_sm_expected" ]; then
                if _sm_actual="$(sha256_hash "$_sm_tmp_dir/$_sm_archive_name")"; then
                    if [ "$_sm_expected" != "$_sm_actual" ]; then
                        error "Checksum mismatch! Expected $_sm_expected, got $_sm_actual"
                    fi
                    info "Checksum OK"
                else
                    warn "No SHA-256 tool found (sha256sum/shasum/openssl), skipping verification"
                fi
            else
                warn "Checksum entry not found for $_sm_archive_name, skipping verification"
            fi
        else
            warn "Checksums not available, skipping verification"
        fi

        info "Extracting..."
        tar xzf "$_sm_tmp_dir/$_sm_archive_name" -C "$_sm_tmp_dir" || error "Extraction failed. The archive may be corrupted."

        mkdir -p "$INSTALL_DIR"
        cp "$_sm_tmp_dir/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
        chmod +x "$INSTALL_DIR/$BIN_NAME"
        info "Installed $INSTALL_DIR/$BIN_NAME"
    fi

    setup_path "$_sm_shell_rc"

    "$INSTALL_DIR/$BIN_NAME" install --shell "$_sm_shell_type" </dev/tty

    printf '\n'
    _sm_display_rc="$_sm_shell_rc"
    case "$_sm_shell_rc" in
        "$HOME"/*) _sm_display_rc="~/${_sm_shell_rc#"$HOME"/}" ;;
    esac

    case "$_sm_action" in
        install)
            info "ShellMate v$_sm_latest_num installed successfully!"
            echo ""
            info "Next steps:"
            echo "  1. Restart your terminal or run: source $_sm_display_rc"
            echo "  2. Try: @ai list all files in current directory"
            echo "  3. Or type a description and press Ctrl+G"
            ;;
        update)
            info "ShellMate updated from v$_sm_installed_version to v$_sm_latest_num successfully!"
            echo ""
            info "Next steps:"
            echo "  1. Restart your terminal or run: source $_sm_display_rc"
            ;;
        skip)
            info "ShellMate is up to date (v$_sm_latest_num)."
            echo ""
            info "Restart your terminal for configuration changes to take effect."
            ;;
    esac
    echo ""
}

main
