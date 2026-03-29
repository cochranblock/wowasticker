#!/bin/bash
# Unlicense — cochranblock.org
# Build wowasticker APK for Android aarch64.
# Requires: Android SDK (API 35), NDK, Rust target aarch64-linux-android.
#
# Setup:
#   rustup target add aarch64-linux-android
#   export ANDROID_HOME=~/Library/Android/sdk
#   export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/<version>
#
# Usage: ./scripts/build-android.sh

set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== Building Rust lib for aarch64-linux-android ==="
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android26-clang"
cargo build --release --target aarch64-linux-android --lib --no-default-features --features jni -p wowasticker

echo "=== Copying .so to android/app/src/main/jniLibs ==="
mkdir -p "$ROOT/android/app/src/main/jniLibs/arm64-v8a"
cp "$ROOT/target/aarch64-linux-android/release/libwowasticker.so" \
   "$ROOT/android/app/src/main/jniLibs/arm64-v8a/"

echo "=== Building APK ==="
cd "$ROOT/android"
./gradlew assembleRelease

APK="$ROOT/android/app/build/outputs/apk/release/app-release-unsigned.apk"
if [ -f "$APK" ]; then
    SIZE=$(stat -f%z "$APK" 2>/dev/null || stat -c%s "$APK" 2>/dev/null)
    echo "=== APK built: $APK ($SIZE bytes) ==="
else
    echo "=== APK not found. Check gradle output above. ==="
    exit 1
fi
