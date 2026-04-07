<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

> **It's not the Mech — it's the pilot.**
>
> This repo is part of [CochranBlock](https://cochranblock.org) — 8 Unlicense Rust repositories that power an entire company on a **single <10MB binary**, a laptop, and a **$10/month** Cloudflare tunnel. No AWS. No Kubernetes. No six-figure DevOps team. Zero cloud.
>
> **[cochranblock.org](https://cochranblock.org)** is a live demo of this architecture. You're welcome to read every line of source code — it's all public domain.
>
> Every repo ships with **[Proof of Artifacts](PROOF_OF_ARTIFACTS.md)** (wire diagrams, screenshots, and build output proving the work is real) and a **[Timeline of Invention](TIMELINE_OF_INVENTION.md)** (dated commit-level record of what was built, when, and why — proving human-piloted AI development, not generated spaghetti).
>
> **Looking to cut your server bill by 90%?** → [Zero-Cloud Tech Intake Form](https://cochranblock.org/deploy)

---

# wowasticker

Pure Rust, offline-first mobile app for student behavioral goals. SQLite persistence, thumb-zone optimized UI. Manual tap-to-score (0/1/2) works today. Voice dictation via Candle Whisper is wired end-to-end (mel → encoder → decoder → tokenizer). Desktop GUI build has a wry API mismatch; CLI binary builds and runs on all platforms.

## Architecture

```
                    ┌─────────────────────────────────────────┐
                    │              ui.rs (Dioxus)              │
                    │  thumb-zone, ScheduleCard, Dictate btn   │
                    └───────────────┬─────────────────────────┘
                                    │
         ┌──────────────────────────┼──────────────────────────┐
         │                          │                          │
         ▼                          ▼                          ▼
┌─────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│     db.rs       │    │     audio.rs        │    │      ai.rs          │
│    (rusqlite)   │    │      (cpal)         │    │    (candle)         │
│                 │    │                     │    │                     │
│ • blocks        │    │ mic ──► 10s buffer  │    │ samples ──► Whisper │
│ • stickers      │◄───│        │            │    │   (GGUF)    │       │
│ • students      │    │        ▼            │    │        │            │
│                 │    │  resample 16kHz     │───►│        ▼            │
│ get/set_sticker │    │        │            │    │  parse 0/1/2        │
└────────┬────────┘    └─────────────────────┘    └──────────┬──────────┘
         │                                                      │
         │  ◄───────────────────────────────────────────────────┘
         │              sticker value
         ▼
┌─────────────────┐
│  wowasticker.db │
│  (on-device)    │
└─────────────────┘

Wire flow: User tap ─► audio capture ─► transcribe ─► parse ─► db write ─► UI refresh
```

## What Works Today

- **Tap-to-score** — Select a schedule block, tap 0/1/2 to record a sticker. No voice needed.
- **Daily report** — Generate plain-text report, copy to clipboard for sharing with parents.
- **Date navigation** — Browse past days (read-only). Today is editable.
- **Undo** — Remove last sticker entry.
- **CLI demo** — `wowasticker-cli demo` runs a full workflow without GUI dependencies.
- **122 unit tests** — DB, parser, report, audio, CLI integration, UI helpers. TRIPLE SIMS verified.

## What's In Progress

- **Voice dictation** — Audio capture works (cpal, 10s, 16kHz resample). Whisper inference wired end-to-end (mel → encoder → decoder → tokenizer). Requires model files via `scripts/download-whisper.sh`. Not yet tested on real device audio.
- **Desktop GUI** — Dioxus 0.5 UI code exists but desktop build has a wry API mismatch. CLI binary builds and runs.
- **Android** — JNI bridge + Gradle project exist. APK builds but has no native UI layer.
- **Multi-student** — `students` table exists but `sticker_records` has no `student_id`. Single-student only.

## Supported Platforms (CLI Binary)

| Platform | Target | Status |
|----------|--------|--------|
| macOS ARM | aarch64-apple-darwin | Builds, tested |
| macOS Intel | x86_64-apple-darwin | Builds, tested |
| Linux x86_64 | x86_64-unknown-linux-gnu | Builds, tested (IRONHIVE) |
| Linux ARM64 | aarch64-unknown-linux-gnu | Via cross |
| Linux ARM32 | armv7-unknown-linux-gnueabihf | Via cross |
| Windows | x86_64-pc-windows-gnu | Via cross |
| FreeBSD | x86_64-unknown-freebsd | Via cross |
| RISC-V | riscv64gc-unknown-linux-gnu | Via cross |
| IBM POWER | powerpc64le-unknown-linux-gnu | Via cross |
| Android | aarch64-linux-android | JNI scaffold |
| iOS | aarch64-apple-ios | lib only |
| Web/PWA | wasm32-unknown-unknown | JS fallback works |

## Build

```bash
# CLI binary (works on all platforms, no optional deps)
cargo build --release --bin wowasticker-cli --no-default-features

# All available targets
./scripts/build-all-targets.sh

# Android APK (requires Android SDK + NDK)
./scripts/build-android.sh

# PWA (optional WASM, JS fallback works without it)
./scripts/build-pwa.sh
```

**Linux deps (Ubuntu/Debian) for full GUI build:**
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libasound2-dev
```

**Mobile (iOS/Android):** Use `dioxus mobile init` and target mobile. See [Dioxus Mobile](https://dioxuslabs.com/learn/0.5/getting_started/mobile).

## Modules

| Module | Purpose |
|--------|---------|
| `db` | SQLite: students, schedule_blocks, sticker_records. Student CRUD (f152-f155: update, add/rename/delete block), `list_day_records()` for daily view, `count_stickers_for_date()`, `delete_sticker()` for undo |
| `audio` | cpal capture, 10s buffer, resample to 16kHz. Feature-gated (`--features audio`) |
| `ai` | `transcribe_audio()` Candle Whisper GGUF (mel → encoder → decoder → tokenizer, wired end-to-end); `extract_behavior()` → score + note + tags; `parse_sticker_from_transcription()` heuristics (works on any text input) |
| `report` | `generate_daily_report()` — plain-text daily sticker report for sharing with parents |
| `ui` | Dioxus App, ScheduleCard (with notes), date navigation, `run_dictation_flow()`, daily report share (clipboard), undo last dictation, progress counter |

## Data Flow

1. User taps schedule block → selects it
2. **Tap-to-score (working):** User taps 0/1/2 button → `db.set_sticker_today_with_note()` → UI refreshes
3. **Voice dictation (wired, untested on device):** User taps "Dictate Observation" → `capture_audio()` (10s) → `transcribe_audio()` → `extract_behavior()` → `db.set_sticker_today_with_note()`
4. UI refreshes via `refresh` signal
5. User taps "Share Daily Report" → `list_day_records()` → `generate_daily_report()` → clipboard
6. User taps date arrows to navigate history (past days are read-only)
7. User taps "Undo" to remove last sticker entry

## Model

```bash
# Download Whisper-Tiny GGUF (Candle-compatible)
./scripts/download-whisper.sh

# Set path (optional; default: whisper-tiny.gguf in cwd)
export WOWASTICKER_WHISPER_PATH=/path/to/model-tiny-q4k.gguf
```

The download script fetches `model-tiny-q4k.gguf`, `config-tiny.json`, `tokenizer-tiny.json`, and `melfilters.bytes` from HuggingFace. Candle 0.8 loads the GGUF model and runs the full decode pipeline (mel spectrogram → encoder → decoder → tokenizer). Heuristic `extract_behavior()` runs on any text input regardless of model availability.

## Test

```bash
# Unit tests (lib only, no GTK/audio required)
cargo test -p wowasticker --no-default-features

# Quality gate (TRIPLE SIMS via exopack)
cargo run -p wowasticker --bin wowasticker-test --features tests
```
