#!/usr/bin/env bash
set -eo pipefail

# Build gpui-template for wasm32-unknown-unknown and run wasm-bindgen.
# Requires: rustup target add wasm32-unknown-unknown
#           cargo install wasm-bindgen-cli
# Uses nightly for WASM (required by wasm_thread / GPUI web stack).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ "${1:-}" == "--release" ]]; then
  RELEASE_FLAG=(--release)
  echo "Building WASM (release)…"
else
  RELEASE_FLAG=()
  echo "Building WASM (dev)…"
fi

rustup toolchain install nightly --component rustfmt 2>/dev/null || true
rustup target add wasm32-unknown-unknown --toolchain nightly 2>/dev/null || true

cargo +nightly build --target wasm32-unknown-unknown --no-default-features --lib "${RELEASE_FLAG[@]}"

if [[ "${RELEASE_FLAG[*]}" == *release* ]]; then
  MODE=release
else
  MODE=debug
fi

WASM_PATH="$ROOT/target/wasm32-unknown-unknown/$MODE/gpui_template.wasm"
if [[ ! -f "$WASM_PATH" ]]; then
  echo "error: WASM not found at $WASM_PATH" >&2
  exit 1
fi

OUT_DIR="$ROOT/www/src/wasm"
mkdir -p "$OUT_DIR"
echo "Running wasm-bindgen → $OUT_DIR"
if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "Installing wasm-bindgen-cli 0.2.121 (must match crate wasm-bindgen)…" >&2
  cargo install wasm-bindgen-cli --version 0.2.121 -f
fi
wasm-bindgen "$WASM_PATH" --out-dir "$OUT_DIR" --target web --no-typescript

echo "Done."
