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

Pure Rust, offline-first mobile app for student behavioral goals. Local AI dictation via Candle Whisper, SQLite persistence, thumb-zone optimized UI.

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

## Build

Default features include `dioxus`, `candle`, and `audio`. A plain `cargo build` enables all three.

**Desktop (Linux):** Install GTK/WebKit deps, then:

```bash
# Full build (default: dioxus + candle + audio)
cargo build -p wowasticker

# Lib only (no UI/audio/candle — for tests or CI)
cargo build -p wowasticker --no-default-features
```

**macOS:** Build with GTK/WebKit (via Homebrew):

```bash
cargo build -p wowasticker --release
```

**Linux deps (Ubuntu/Debian):**
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libasound2-dev
```

**Mobile (iOS/Android):** Use `dioxus mobile init` and target mobile. See [Dioxus Mobile](https://dioxuslabs.com/learn/0.5/getting_started/mobile).

## Modules

| Module | Purpose |
|--------|---------|
| `db` | SQLite: students, schedule_blocks, sticker_records (with note). Student CRUD, `count_stickers_today()` for progress, `get_sticker_record()` for full records |
| `audio` | cpal capture, 10s buffer, resample to 16kHz. Feature-gated (`--features audio`) |
| `ai` | `transcribe_audio()` Candle Whisper GGUF; `extract_behavior()` → score + note + tags; `extract_tags()` heuristic tag extraction; `parse_sticker_from_transcription()` heuristics |
| `ui` | Dioxus App, ScheduleCard, `run_dictation_flow()` (capture→transcribe→parse→save), dynamic goal from student, sticker progress counter, transcription display with tags, countdown + retry |

## Data Flow

1. User taps schedule block → selects it
2. User taps "Dictate Observation" → `capture_audio()` (10s) → `transcribe_audio()` → `extract_behavior()` → `db.set_sticker_today_with_note()`
3. UI refreshes via `refresh` signal

## Model

```bash
# Download Whisper-Tiny GGUF (Candle-compatible)
./scripts/download-whisper.sh

# Set path (optional; default: whisper-tiny.gguf in cwd)
export WOWASTICKER_WHISPER_PATH=/path/to/model-tiny-q4k.gguf
```

The download script fetches `model-tiny-q4k.gguf`, `config-tiny.json`, `tokenizer-tiny.json`, and `melfilters.bytes` from HuggingFace. Candle 0.8 loads GGUF; full decode pipeline (mel→encoder→decoder→tokenizer) is scaffolded. Heuristic `extract_behavior()` runs regardless of model availability.

## Test

```bash
# Unit tests (lib only, no GTK/audio required)
cargo test -p wowasticker --no-default-features

# Quality gate (TRIPLE SIMS via exopack)
cargo run -p wowasticker --bin wowasticker-test --features tests
```
