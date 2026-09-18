#!/usr/bin/env bash
# Build a rustftechh .deb that installs /usr/bin/rustfetch.
# Prefer assembling with dpkg-deb after `cargo build --release` so GitHub
# Actions / local boxes with rustup (edition 2024) work without Debian's older cargo.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PKG_NAME="rustftechh"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
ARCH="$(dpkg --print-architecture 2>/dev/null || echo amd64)"
REVISION="${DEB_REVISION:-1}"
DEB_VERSION="${VERSION}-${REVISION}"
OUT_DIR="${OUT_DIR:-$ROOT/packaging/apt/out}"
STAGE="$OUT_DIR/${PKG_NAME}_${DEB_VERSION}_${ARCH}"
DEB_PATH="$OUT_DIR/${PKG_NAME}_${DEB_VERSION}_${ARCH}.deb"

mkdir -p "$OUT_DIR"
rm -rf "$STAGE"
mkdir -p "$STAGE/DEBIAN" \
  "$STAGE/usr/bin" \
  "$STAGE/usr/share/man/man1" \
  "$STAGE/usr/share/bash-completion/completions" \
  "$STAGE/usr/share/zsh/vendor-completions" \
  "$STAGE/usr/share/fish/vendor_completions.d" \
  "$STAGE/usr/share/doc/${PKG_NAME}"

echo "==> Building release binary (cargo)"
cargo build --release --locked

install -m 755 target/release/rustfetch "$STAGE/usr/bin/rustfetch"
install -m 644 man/rustfetch.1 "$STAGE/usr/share/man/man1/rustfetch.1"
gzip -9n -f "$STAGE/usr/share/man/man1/rustfetch.1"
install -m 644 completions/rustfetch.bash \
  "$STAGE/usr/share/bash-completion/completions/rustfetch"
install -m 644 completions/rustfetch.zsh \
  "$STAGE/usr/share/zsh/vendor-completions/_rustfetch"
install -m 644 completions/rustfetch.fish \
  "$STAGE/usr/share/fish/vendor_completions.d/rustfetch.fish"
install -m 644 LICENSE-MIT LICENSE-APACHE README.md \
  "$STAGE/usr/share/doc/${PKG_NAME}/"
cp debian/copyright "$STAGE/usr/share/doc/${PKG_NAME}/copyright"
cp debian/changelog "$STAGE/usr/share/doc/${PKG_NAME}/changelog.Debian"
gzip -9n -f "$STAGE/usr/share/doc/${PKG_NAME}/changelog.Debian"

SIZE_KB="$(du -sk "$STAGE" | awk '{print $1}')"

cat > "$STAGE/DEBIAN/control" << CTRL
Package: ${PKG_NAME}
Version: ${DEB_VERSION}
Architecture: ${ARCH}
Maintainer: Zack <zackmsa777-a11y@users.noreply.github.com>
Installed-Size: ${SIZE_KB}
Depends: libc6 (>= 2.34)
Provides: rustfetch
Section: utils
Priority: optional
Homepage: https://github.com/zackmsa777-a11y/rustfetch
Description: blazingly fast system information fetch tool (rustfetch)
 rustftechh installs the rustfetch binary — a fast, extensible system
 information tool written in Rust with fastfetch-style logos and probes.
 .
 Package name matches crates.io (rustftechh). Command: /usr/bin/rustfetch.
CTRL

# Optional shared-library Depends from the binary (best-effort)
if command -v dpkg-shlibdeps >/dev/null 2>&1; then
  mkdir -p "$STAGE/DEBIAN"
  # dpkg-shlibdeps expects a debian/ tree; skip if it fails
  true
fi

echo "==> Building ${DEB_PATH}"
dpkg-deb --root-owner-group --build "$STAGE" "$DEB_PATH"
echo "==> Contents:"
dpkg-deb -c "$DEB_PATH" | sed -n '1,80p'
echo
echo "==> Info:"
dpkg-deb -I "$DEB_PATH"
echo
echo "Built: $DEB_PATH"
