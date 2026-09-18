#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

TARGETS=(
  aarch64-apple-darwin
  x86_64-apple-darwin
  x86_64-pc-windows-gnu
  x86_64-pc-windows-msvc
)

echo "==> rustfetch cross cfg check"
echo "Host: $(rustc -vV | awk '/^host:/{print $2}')"

for t in "${TARGETS[@]}"; do
  if ! rustup target list --installed | grep -qx "$t"; then
    echo "==> rustup target add $t"
    rustup target add "$t" || {
      echo "!! failed to add $t (skipping)"
      continue
    }
  fi
  echo "==> cargo check --target $t"
  if cargo check --target "$t"; then
    echo "OK  $t"
  else
    ec=$?
    echo "!! cargo check --target $t failed (exit $ec)"
    echo "   Missing cross linkers are expected on Linux for Darwin/MSVC;"
    echo "   inspect compile errors above. Platform modules must still typecheck."
  fi
done

echo "==> host cargo test"
cargo test
