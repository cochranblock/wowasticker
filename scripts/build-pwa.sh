#!/bin/bash
# Unlicense — cochranblock.org
# Build WASM + PWA assets for wowasticker.
# Requires: wasm-pack (cargo install wasm-pack)
# Output: pwa/ directory ready to deploy as static site.

set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== Building WASM ==="
if command -v wasm-pack &>/dev/null; then
    cd "$ROOT"
    wasm-pack build --target web --release --no-default-features --features wasm --out-dir pwa/pkg
    cp pwa/pkg/wowasticker_wasm.js pwa/ 2>/dev/null || true
    cp pwa/pkg/wowasticker_wasm_bg.wasm pwa/ 2>/dev/null || true
    echo "WASM built. PWA ready at pwa/"
else
    echo "wasm-pack not found. PWA works in JS-only fallback mode."
    echo "Install: cargo install wasm-pack"
fi

echo "=== PWA files ==="
ls -la "$ROOT/pwa/"
echo "Deploy pwa/ to any static host. Works offline via service worker."
