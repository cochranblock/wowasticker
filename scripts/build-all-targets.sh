#!/bin/bash
# Unlicense — cochranblock.org
# Build wowasticker-cli for every available target.
# Uses cargo for native, 'cross' for cross-compilation.
# Output: release/ directory with binaries named wowasticker-cli-{target}

set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/release"
mkdir -p "$OUT"
BIN="wowasticker-cli"
FLAGS="--release --bin $BIN --no-default-features"

built=0
failed=0

build_native() {
    local target="$1"
    echo "--- Building $target (native) ---"
    if cargo build $FLAGS --target "$target" 2>&1; then
        cp "$ROOT/target/$target/release/$BIN" "$OUT/$BIN-$target"
        local size=$(stat -f%z "$OUT/$BIN-$target" 2>/dev/null || stat -c%s "$OUT/$BIN-$target" 2>/dev/null)
        echo "OK: $BIN-$target ($size bytes)"
        built=$((built + 1))
    else
        echo "FAIL: $target"
        failed=$((failed + 1))
    fi
}

build_cross() {
    local target="$1"
    if ! command -v cross &>/dev/null; then
        echo "SKIP: $target (cross not installed — cargo install cross)"
        return
    fi
    echo "--- Building $target (cross) ---"
    if cross build $FLAGS --target "$target" 2>&1; then
        cp "$ROOT/target/$target/release/$BIN" "$OUT/$BIN-$target"
        local size=$(stat -f%z "$OUT/$BIN-$target" 2>/dev/null || stat -c%s "$OUT/$BIN-$target" 2>/dev/null)
        echo "OK: $BIN-$target ($size bytes)"
        built=$((built + 1))
    else
        echo "FAIL: $target"
        failed=$((failed + 1))
    fi
}

echo "=== WowaSticker Multi-Arch Build ==="
echo ""

# Tier 1: Native targets (build on this machine)
build_native aarch64-apple-darwin      # macOS ARM
build_native x86_64-apple-darwin       # macOS Intel

# Tier 2: Cross-compiled targets (need 'cross' or build on target)
build_cross x86_64-unknown-linux-gnu         # Linux x86_64
build_cross aarch64-unknown-linux-gnu        # Linux ARM64 (RPi 4/5, Graviton)
build_cross armv7-unknown-linux-gnueabihf    # Linux ARM 32-bit (older RPi, IoT)
build_cross x86_64-pc-windows-gnu           # Windows via MinGW
build_cross x86_64-unknown-freebsd          # FreeBSD
build_cross riscv64gc-unknown-linux-gnu     # RISC-V
build_cross powerpc64le-unknown-linux-gnu   # IBM POWER

# Tier 3: Mobile (separate build scripts)
# Android: ./scripts/build-android.sh
# iOS: cargo build --release --target aarch64-apple-ios --lib --no-default-features

# Tier 4: Web
# WASM: ./scripts/build-pwa.sh

echo ""
echo "=== Results: $built built, $failed failed ==="
echo "Binaries in: $OUT/"
ls -lh "$OUT/"
