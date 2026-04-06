#!/bin/sh
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
    local os
    os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    case "$os" in
        linux*)  echo "linux" ;;
        darwin*) echo "darwin" ;;
        *)       error "Unsupported OS: $os" ;;
    esac
}

detect_arch() {
    local arch
    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64) echo "amd64" ;;
        aarch64|arm64) echo "arm64" ;;
        *)             error "Unsupported architecture: $arch" ;;
    esac
}

detect_shell_type() {
    local shell_name="${SHELL:-}"
    shell_name="${shell_name##*/}"
    case "$shell_name" in
        bash|zsh|fish) echo "$shell_name" ;;
        dash|ash|sh)   echo "sh" ;;
        *)             echo "bash" ;;
    esac
}

detect_shell_rc() {
    local os
    os="$(uname -s)"
    case "$(detect_shell_type)" in
        bash)
            if [ "$os" = "Darwin" ]; then
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
    local url="$1"
    local output="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fSL -o "$output" "$url"
    else
        wget -qO "$output" "$url"
    fi
}

get_latest_version() {
    local url="https://api.github.com/repos/${REPO}/releases/latest"
    http_get "$url" | grep '"tag_name"' | head -1 | sed -E 's/.*"([^"]+)".*/\1/'
}

# Compute SHA-256 hash of a file, returns lowercase hex digest on stdout.
# Works on Linux (sha256sum), macOS (shasum -a 256), and Alpine/BusyBox.
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
    local shell_rc="$1"
    local path_line='export PATH="$HOME/.shellmate/bin:$PATH"'

    if grep -qF '.shellmate/bin' "$shell_rc" 2>/dev/null; then
        return
    fi

    mkdir -p "$(dirname "$shell_rc")"
    printf '\n' >> "$shell_rc"
    printf '%s\n' '# ShellMate' >> "$shell_rc"
    printf '%s\n' "$path_line" >> "$shell_rc"
    info "Added PATH to $shell_rc"
}

main() {
    local os arch version shell_type shell_rc

    printf '\n'
    printf '%b\n' "${CYAN}╔══════════════════════════════════════╗"
    printf '%b\n' "║       ShellMate Installer            ║"
    printf '%b\n' "╚══════════════════════════════════════╝${NC}"
    printf '\n'

    os="$(detect_os)"
    arch="$(detect_arch)"
    shell_type="$(detect_shell_type)"
    shell_rc="$(detect_shell_rc)"

    info "OS: $os  Arch: $arch  Shell: $shell_type"

    # Get latest version
    info "Fetching latest version..."
    version="$(get_latest_version)"
    if [ -z "$version" ]; then
        error "Failed to determine latest version. Check your internet connection."
    fi
    info "Latest version: $version"

    # Build download URL matching release workflow naming:
    # shellmate_{version}_{os}_{arch}.tar.gz
    local archive_name="shellmate_${version}_${os}_${arch}.tar.gz"
    local download_url="https://github.com/${REPO}/releases/download/${version}/${archive_name}"

    local tmp_dir
    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "$tmp_dir"' EXIT

    # Download
    info "Downloading $archive_name..."
    http_download "$download_url" "$tmp_dir/$archive_name" || {
        error "Download failed. The archive for your platform ($os/$arch) may not exist.\n  URL: $download_url"
    }

    # Verify checksum if available
    local checksum_url="https://github.com/${REPO}/releases/download/${version}/sha256sums.txt"
    if http_get "$checksum_url" > "$tmp_dir/sha256sums.txt" 2>/dev/null; then
        info "Verifying checksum..."
        local expected
        expected="$(grep "$archive_name" "$tmp_dir/sha256sums.txt" | awk '{print $1}')"
        if [ -n "$expected" ]; then
            local actual
            if actual="$(sha256_hash "$tmp_dir/$archive_name")"; then
                if [ "$expected" != "$actual" ]; then
                    error "Checksum mismatch! Expected $expected, got $actual"
                fi
                info "Checksum OK"
            else
                warn "No SHA-256 tool found (sha256sum/shasum/openssl), skipping verification"
            fi
        else
            warn "Checksum entry not found for $archive_name, skipping verification"
        fi
    else
        warn "Checksums not available, skipping verification"
    fi

    # Extract
    info "Extracting..."
    tar xzf "$tmp_dir/$archive_name" -C "$tmp_dir" || error "Extraction failed. The archive may be corrupted."

    # Install binary
    mkdir -p "$INSTALL_DIR"
    cp "$tmp_dir/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
    chmod +x "$INSTALL_DIR/$BIN_NAME"
    info "Installed $INSTALL_DIR/$BIN_NAME"

    # Setup PATH
    setup_path "$shell_rc"

    # Done
    local version_num="${version#v}"
    echo ""
    info "ShellMate $version_num installed successfully!"
    echo ""
    info "Next steps:"
    local display_rc="$shell_rc"
    case "$shell_rc" in
        "$HOME"/*) display_rc="~/${shell_rc#"$HOME"/}" ;;
    esac
    echo "  1. Restart your terminal or run: source $display_rc"
    echo "  2. Run: shellmate install --shell $shell_type"
    echo "  3. Or edit ~/.shellmate/config.yaml to configure your AI provider"
    echo ""
}

main
