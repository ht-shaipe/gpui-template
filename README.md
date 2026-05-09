# GPUI Template

Desktop app (GPUI) + optional **Web (WASM)** build.

## Desktop

```bash
cargo run
```

Requires default Cargo features (`native-app`) for the `gpui-template` binary (system tray, etc.).

## Web (WASM + Vite)

Matches the model used by [`gpui-component` story-web](https://github.com/sxhxliang/gpui-component): Rust compiles to `wasm32-unknown-unknown`, `wasm-bindgen` emits JS glue, Vite serves the page with COOP/COEP for SharedArrayBuffer.

**Prerequisites:** Rust **nightly** (for the GPUI web stack), `wasm32-unknown-unknown`, `wasm-bindgen-cli` **0.2.121** (must match the `wasm-bindgen` crate version), [Bun](https://bun.sh/).

```bash
make install-web   # nightly target + wasm-bindgen-cli + www deps
make dev-web       # debug WASM + Vite on http://localhost:3000
```

Or manually:

```bash
./scripts/build-wasm.sh          # or ./scripts/build-wasm.sh --release
cd www && bun install && bun run dev
```

WASM build uses `--no-default-features --lib` so the desktop binary is not linked for `wasm32`.

**Note:** Icon/fonts for `gpui-component` load from the gallery CDN URL configured in `init_web` in `src/lib.rs` (same approach as story-web).
