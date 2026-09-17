#!/usr/bin/env bash
set -euo pipefail

REPO="zackmsa777-a11y/rustfetch"
INSTALL_DIR="/usr/local/bin"

echo "Installing rustfetch..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
    linux) TARGET="${ARCH}-unknown-linux-musl" ;;
    darwin) TARGET="${ARCH}-apple-darwin" ;;
    *) echo "Unsupported operating system: $OS"; exit 1 ;;
esac

LATEST_TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || echo "v0.1.0")
ARCHIVE="rustfetch-${LATEST_TAG}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ARCHIVE}"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading ${URL}..."
if curl -sSL "$URL" -o "${TMPDIR}/${ARCHIVE}"; then
    tar -xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"
    if [ -w "$INSTALL_DIR" ]; then
        mv "${TMPDIR}/rustfetch" "${INSTALL_DIR}/rustfetch"
    else
        sudo mv "${TMPDIR}/rustfetch" "${INSTALL_DIR}/rustfetch"
    fi
    chmod +x "${INSTALL_DIR}/rustfetch"
    echo "rustfetch successfully installed to ${INSTALL_DIR}/rustfetch!"
else
    echo "Prebuilt binary release not yet available. Falling back to cargo install..."
    cargo install rustfetch
fi
