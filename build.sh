#!/usr/bin/env bash
set -euo pipefail

VERSION="${VERSION:-}"
if [[ -z "$VERSION" ]]; then
    VERSION="$(grep '^version =' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')"
fi

NAME="shellmate"
DIST_DIR="dist"
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

build_target() {
    local target="$1"
    local os arch

    case "$target" in
        *linux*)  os="linux" ;;
        *darwin*) os="darwin" ;;
        *windows*) os="windows" ;;
        *)        os="unknown" ;;
    esac

    case "$target" in
        *x86_64*)  arch="amd64" ;;
        *aarch64*) arch="arm64" ;;
        *i686*)    arch="386" ;;
        *)         arch="$(echo "$target" | cut -d'-' -f1)" ;;
    esac

    local archive_name="${NAME}_${VERSION}_${os}_${arch}"
    local bin_name="${NAME}"
    [[ "$os" == "windows" ]] && bin_name="${NAME}.exe"

    info "Building ${archive_name}..."

    if ! rustup target list --installed 2>/dev/null | grep -q "$target"; then
        info "Installing target ${target}..."
        rustup target add "$target" || true
    fi

    cargo build --release --target "$target" 2>&1 || {
        error "Build failed for ${target}"
        return 1
    }

    local staging_dir
    staging_dir="$(mktemp -d)"
    trap 'rm -rf "$staging_dir"' RETURN

    cp "target/${target}/release/${bin_name}" "${staging_dir}/"
    [[ -f LICENSE ]] && cp LICENSE "${staging_dir}/"
    [[ -f README.md ]] && cp README.md "${staging_dir}/"

    local ext="tar.gz"
    local archive_path="${DIST_DIR}/${archive_name}.${ext}"

    mkdir -p "$DIST_DIR"

    (cd "$staging_dir" && tar czf - "${bin_name}" $(ls LICENSE README.md 2>/dev/null || true)) > "$archive_path"

    info "Created ${archive_path}"
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Build ShellMate for multiple platforms and package as tar.gz archives.

Options:
  -t, --target <target>   Build only the specified target (can be repeated)
  -l, --list              List available targets
  -h, --help              Show this help message

Available targets:
$(printf '  %s\n' "${TARGETS[@]}")

Environment:
  VERSION   Override version (default: read from Cargo.toml)

Examples:
  $(basename "$0")                        # Build all targets
  $(basename "$0") -t x86_64-unknown-linux-gnu
  VERSION=2.0.0 $(basename "$0")          # Override version
EOF
}

list_targets() {
    printf '%s\n' "${TARGETS[@]}"
}

SELECTED_TARGETS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        -t|--target)
            SELECTED_TARGETS+=("$2")
            shift 2
            ;;
        -l|--list)
            list_targets
            exit 0
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

if [[ ${#SELECTED_TARGETS[@]} -eq 0 ]]; then
    SELECTED_TARGETS=("${TARGETS[@]}")
fi

echo ""
echo "╔══════════════════════════════════════╗"
echo "║       ShellMate Cross Builder        ║"
echo "╚══════════════════════════════════════╝"
echo ""
info "Version: ${VERSION}"
info "Targets: ${#SELECTED_TARGETS[@]}"
echo ""

FAILED=()

for target in "${SELECTED_TARGETS[@]}"; do
    if ! build_target "$target"; then
        FAILED+=("$target")
    fi
done

echo ""
if [[ ${#FAILED[@]} -eq 0 ]]; then
    info "All builds completed successfully!"
    info "Artifacts in ${DIST_DIR}/:"
    ls -lh "${DIST_DIR}/"*.tar.gz 2>/dev/null | awk '{print "  " $NF, $5}'
else
    error "Failed targets: ${FAILED[*]}"
    exit 1
fi
