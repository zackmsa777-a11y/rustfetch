#!/usr/bin/env bash
set -euo pipefail

REPO="zackmsa777-a11y/rustfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
REPO_URL="https://github.com/${REPO}.git"

echo "Installing rustfetch..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

case "$OS" in
    linux|darwin) ;;
    *) echo "Unsupported operating system: $OS" >&2; exit 1 ;;
esac

is_gzip() {
    local f="$1"
    [ -s "$f" ] || return 1
    local magic
    magic="$(od -An -N2 -tx1 "$f" | tr -d ' \n')"
    [ "$magic" = "1f8b" ]
}

install_from_archive() {
    local archive_path="$1"
    local extract_dir="$2"
    mkdir -p "$extract_dir"
    tar -xzf "$archive_path" -C "$extract_dir"
    local bin
    bin="$(find "$extract_dir" -type f \( -name 'rustfetch' -o -name 'rustfetch.exe' \) | head -n1)"
    if [ -z "$bin" ]; then
        echo "Archive did not contain a rustfetch binary." >&2
        return 1
    fi
    chmod +x "$bin"
    mkdir -p "$INSTALL_DIR"
    if [ -w "$INSTALL_DIR" ]; then
        mv "$bin" "${INSTALL_DIR}/rustfetch"
    else
        sudo mv "$bin" "${INSTALL_DIR}/rustfetch"
    fi
    echo "rustfetch successfully installed to ${INSTALL_DIR}/rustfetch!"
}

fallback_cargo() {
    echo "Falling back to cargo install from git..."
    if ! command -v cargo >/dev/null 2>&1; then
        echo "cargo not found. Install Rust 1.88+ from https://rustup.rs then re-run." >&2
        exit 1
    fi
    cargo install --git "$REPO_URL" --locked 2>/dev/null || cargo install --git "$REPO_URL"
    echo "Installed via cargo. Ensure ~/.cargo/bin is on your PATH."
}

# Resolve latest tag via /releases/latest redirect (no API quota).
LATEST_TAG="$(
    curl -fsSIL "https://github.com/${REPO}/releases/latest" 2>/dev/null \
      | tr -d '\r' \
      | sed -n 's|^[Ll]ocation:.*/tag/\([^/[:space:]]*\).*|\1|p' \
      | head -n1 || true
)"
if [ -z "${LATEST_TAG}" ]; then
    LATEST_TAG="$(
        curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" \
          | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
          | head -n1 || true
    )"
fi
if [ -z "${LATEST_TAG}" ]; then
    echo "Could not determine latest release tag." >&2
    fallback_cargo
    exit 0
fi

echo "Latest release: ${LATEST_TAG}"

CANDIDATES=()
case "$OS" in
    linux)
        CANDIDATES+=(
            "rustfetch-${LATEST_TAG}-${ARCH}-unknown-linux-musl.tar.gz"
            "rustfetch-${LATEST_TAG}-${ARCH}-unknown-linux-gnu.tar.gz"
            "rustfetch-linux-${ARCH}.tar.gz"
        )
        ;;
    darwin)
        CANDIDATES+=(
            "rustfetch-${LATEST_TAG}-${ARCH}-apple-darwin.tar.gz"
        )
        ;;
esac

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

for ARCHIVE in "${CANDIDATES[@]}"; do
    URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ARCHIVE}"
    OUT="${TMPDIR}/${ARCHIVE}"
    echo "Trying ${URL}..."
    HTTP_CODE="$(curl -sSL -o "$OUT" -w '%{http_code}' "$URL" || true)"
    if [ "$HTTP_CODE" != "200" ]; then
        echo "  HTTP ${HTTP_CODE}, skipping."
        rm -f "$OUT"
        continue
    fi
    if ! is_gzip "$OUT"; then
        echo "  Download is not a gzip archive, skipping."
        rm -f "$OUT"
        continue
    fi
    if install_from_archive "$OUT" "$TMPDIR/extract"; then
        exit 0
    fi
    rm -rf "$TMPDIR/extract"
done

echo "No matching prebuilt binary found for ${OS}/${ARCH}."
fallback_cargo
