#!/bin/bash
# Download Whisper-Tiny GGUF for wowasticker (Candle-compatible).
# Usage: ./scripts/download-whisper.sh [output_dir]
# Output: whisper-tiny-q4_k.gguf, config-tiny.json, tokenizer-tiny.json

set -e
OUT="${1:-.}"
mkdir -p "$OUT"
cd "$OUT"

BASE="https://huggingface.co/oxide-lab/whisper-tiny-GGUF/resolve/main"
for f in whisper-tiny-q4_k.gguf config-tiny.json tokenizer-tiny.json; do
  if [ ! -f "$f" ]; then
    echo "Downloading $f..."
    curl -sL -o "$f" "$BASE/$f"
  else
    echo "Exists: $f"
  fi
done

echo "Done. Set WOWASTICKER_WHISPER_PATH to $(pwd)/whisper-tiny-q4_k.gguf"
