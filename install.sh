#!/usr/bin/env bash
# rustfetch installer — prefer GitHub release binaries, fall back to cargo.
# Usage:
#   curl -sSL https://raw.githubusercontent.com/zackmsa777-a11y/rustfetch/master/install.sh | bash
#   INSTALL_DIR=~/.local/bin ./install.sh
#   FORCE=1 ./install.sh          # reinstall even if version already matches
#   METHOD=cargo ./install.sh     # skip binaries; cargo install --git / --path
#
# Naming: GitHub repo = rustfetch; crates.io crate = rustftechh; binary = rustfetch.
# After publish: cargo install rustftechh --locked  # installs ~/.cargo/bin/rustfetch
set -euo pipefail

REPO="zackmsa777-a11y/rustfetch"
REPO_URL="https://github.com/${REPO}.git"
RELEASES_BASE="https://github.com/${REPO}/releases"

die() { echo "error: $*" >&2; exit 1; }
info() { echo "$*"; }

# Prefer explicit INSTALL_DIR, else writable /usr/local/bin, else ~/.local/bin.
default_install_dir() {
    if [ -n "${INSTALL_DIR:-}" ]; then
        printf '%s' "$INSTALL_DIR"
        return
    fi
    if [ -d /usr/local/bin ] && [ -w /usr/local/bin ]; then
        printf '%s' /usr/local/bin
        return
    fi
    printf '%s' "${HOME}/.local/bin"
}

INSTALL_DIR="$(default_install_dir)"
METHOD="${METHOD:-auto}"
FORCE="${FORCE:-0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" 2>/dev/null && pwd || true)"

info "Installing rustfetch..."
info "Install directory: ${INSTALL_DIR}"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) die "Unsupported architecture: $ARCH (supported: x86_64, aarch64)" ;;
esac

case "$OS" in
    linux|darwin) ;;
    mingw*|msys*|cygwin*)
        die "Windows bash detected. Download a release zip from ${RELEASES_BASE}/latest or use: cargo install --git ${REPO_URL}"
        ;;
    *) die "Unsupported operating system: $OS (supported: linux, darwin)" ;;
esac

is_gzip() {
    local f="$1"
    [ -s "$f" ] || return 1
    local magic
    magic="$(od -An -N2 -tx1 "$f" | tr -d ' \n')"
    [ "$magic" = "1f8b" ]
}

existing_binary() {
    if [ -x "${INSTALL_DIR}/rustfetch" ]; then
        printf '%s' "${INSTALL_DIR}/rustfetch"
        return 0
    fi
    if command -v rustfetch >/dev/null 2>&1; then
        command -v rustfetch
        return 0
    fi
    return 1
}

version_matches_tag() {
    local tag="$1"
    local bin ver bare
    bin="$(existing_binary)" || return 1
    ver="$("$bin" --version 2>/dev/null | head -n1 || true)"
    [ -n "$ver" ] || return 1
    bare="${tag#v}"
    printf '%s' "$ver" | grep -Eq "(^|[^0-9])${bare}([^0-9]|$)"
}

install_binary() {
    local bin="$1"
    [ -f "$bin" ] || die "Binary not found: $bin"
    chmod +x "$bin"
    mkdir -p "$INSTALL_DIR"
    local dest="${INSTALL_DIR}/rustfetch"
    if [ -w "$INSTALL_DIR" ]; then
        mv -f "$bin" "$dest"
    else
        if command -v sudo >/dev/null 2>&1; then
            sudo mv -f "$bin" "$dest"
        else
            die "Cannot write to ${INSTALL_DIR} (not writable and sudo not available). Set INSTALL_DIR to a writable path (e.g. ~/.local/bin)."
        fi
    fi
    info "rustfetch installed to ${dest}"
    if ! printf '%s' ":${PATH}:" | grep -q ":${INSTALL_DIR}:"; then
        info "Note: ${INSTALL_DIR} is not on your PATH. Add it, e.g.:"
        info "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
    "$dest" --version 2>/dev/null || true
}

fallback_cargo() {
    info "Falling back to cargo install..."
    if ! command -v cargo >/dev/null 2>&1; then
        die "cargo not found. Install Rust 1.88+ from https://rustup.rs then re-run, or download a release from ${RELEASES_BASE}/latest"
    fi

    # Prefer local path when invoked from a source checkout.
    if [ -n "${SCRIPT_DIR}" ] && [ -f "${SCRIPT_DIR}/Cargo.toml" ] && [ -d "${SCRIPT_DIR}/src" ]; then
        info "Detected source checkout at ${SCRIPT_DIR}; building release binary"
        (cd "${SCRIPT_DIR}" && cargo build --release --locked 2>/dev/null) \
            || (cd "${SCRIPT_DIR}" && cargo build --release) \
            || die "cargo build --release failed in ${SCRIPT_DIR}"
        local built="${SCRIPT_DIR}/target/release/rustfetch"
        [ -f "$built" ] || die "Expected binary missing: $built"
        # Copy then install so we do not move the workspace artifact.
        local tmpbin
        tmpbin="$(mktemp)"
        cp "$built" "$tmpbin"
        install_binary "$tmpbin"
        return 0
    fi

    info "Using cargo install --git ${REPO_URL} (installs to ~/.cargo/bin; INSTALL_DIR ignored)"
    info "Tip (after crates.io publish): cargo install rustftechh --locked  # binary: rustfetch"
    cargo install --git "$REPO_URL" --locked --force 2>/dev/null \
        || cargo install --git "$REPO_URL" --force
    info "Installed via cargo --git. Ensure ~/.cargo/bin is on your PATH."
}

install_from_archive() {
    local archive_path="$1"
    local extract_dir="$2"
    mkdir -p "$extract_dir"
    if ! tar -xzf "$archive_path" -C "$extract_dir"; then
        echo "Failed to extract archive: $archive_path" >&2
        return 1
    fi
    local bin=""
    if [ -f "${extract_dir}/rustfetch" ]; then
        bin="${extract_dir}/rustfetch"
    else
        bin="$(find "$extract_dir" -maxdepth 3 -type f \( -name 'rustfetch' -o -name 'rustfetch.exe' \) 2>/dev/null | head -n1 || true)"
    fi
    if [ -z "$bin" ]; then
        echo "Archive did not contain a rustfetch binary." >&2
        return 1
    fi
    install_binary "$bin"
}

resolve_latest_tag() {
    local tag=""
    # Prefer redirect Location from /releases/latest (no API quota).
    tag="$(
        curl -fsSIL "${RELEASES_BASE}/latest" 2>/dev/null \
          | tr -d '\r' \
          | sed -n 's|^[Ll]ocation:.*/tag/\([^/[:space:]]*\).*|\1|p' \
          | head -n1 || true
    )"
    if [ -z "$tag" ]; then
        tag="$(
            curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null \
              | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
              | head -n1 || true
        )"
    fi
    printf '%s' "$tag"
}

if [ "$METHOD" = "cargo" ] || [ "$METHOD" = "git" ] || [ "$METHOD" = "path" ]; then
    fallback_cargo
    exit 0
fi

if ! command -v curl >/dev/null 2>&1; then
    info "curl not found; cannot download release binaries."
    fallback_cargo
    exit 0
fi

LATEST_TAG="$(resolve_latest_tag)"
if [ -z "${LATEST_TAG}" ]; then
    info "Could not determine latest release tag (no releases yet, or network blocked)."
    fallback_cargo
    exit 0
fi

info "Latest release: ${LATEST_TAG}"

if [ "$FORCE" != "1" ] && version_matches_tag "$LATEST_TAG"; then
    info "Already up to date ($(existing_binary) matches ${LATEST_TAG}). Set FORCE=1 to reinstall."
    exit 0
fi

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
    # Only download from this project's GitHub releases URL pattern.
    URL="${RELEASES_BASE}/download/${LATEST_TAG}/${ARCHIVE}"
    OUT="${TMPDIR}/${ARCHIVE}"
    info "Trying ${URL}..."
    HTTP_CODE="$(curl -sSL -o "$OUT" -w '%{http_code}' "$URL" || true)"
    if [ "$HTTP_CODE" != "200" ]; then
        info "  HTTP ${HTTP_CODE}, skipping."
        rm -f "$OUT"
        continue
    fi
    if ! is_gzip "$OUT"; then
        info "  Download is not a gzip archive, skipping."
        rm -f "$OUT"
        continue
    fi
    if install_from_archive "$OUT" "$TMPDIR/extract"; then
        exit 0
    fi
    rm -rf "$TMPDIR/extract"
done

info "No matching prebuilt binary found for ${OS}/${ARCH}."
fallback_cargo
