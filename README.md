# wowasticker

Pure Rust, offline-first mobile app for student behavioral goals. Local AI dictation via Candle Whisper, SQLite persistence, thumb-zone optimized UI.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Dioxus UI (thumb-zone, bottom-weighted, swipeable cards)   │
├─────────────────────────────────────────────────────────────┤
│  State: selected block, status, refresh trigger             │
├──────────────┬──────────────┬──────────────┬───────────────┤
│  db.rs       │  audio.rs    │  ai.rs       │  ui.rs        │
│  rusqlite    │  cpal        │  candle      │  components   │
│  students    │  10s buffer  │  Whisper     │  ScheduleCard │
│  blocks      │  16kHz mono  │  GGUF        │  Dictate btn  │
│  stickers    │  resample    │  parse 0/1/2 │               │
└──────────────┴──────────────┴──────────────┴───────────────┘
```

## Build

**Desktop (Linux):** Install GTK/WebKit deps, then:

```bash
# With audio (requires libalsa)
cargo build -p wowasticker --features audio

# Without audio (UI + DB only)
cargo build -p wowasticker
```

**Linux deps (Ubuntu/Debian):**
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libasound2-dev
```

**Mobile (iOS/Android):** Use `dioxus mobile init` and target mobile. See [Dioxus Mobile](https://dioxuslabs.com/learn/0.5/getting_started/mobile).

## Modules

| Module | Purpose |
|--------|---------|
| `db` | SQLite: students, schedule_blocks, sticker_records. `ensure_default_schedule()`, `get/set_sticker_today()` |
| `audio` | cpal capture, 10s buffer, resample to 16kHz. Feature-gated (`--features audio`) |
| `ai` | `transcribe_audio()` stub for Candle Whisper GGUF; `parse_sticker_from_transcription()` for 0/1/2 |
| `ui` | Dioxus App, ScheduleCard, dictation button, async flow |

## Data Flow

1. User taps schedule block → selects it
2. User taps "Dictate Observation" → `capture_audio()` (10s) → `transcribe_audio()` → `parse_sticker_from_transcription()` → `db.set_sticker_today()`
3. UI refreshes via `refresh` signal

## Model

Place `whisper-tiny.gguf` in the working directory, or set path in `run_dictation_flow()`. Candle Whisper GGUF loading is stubbed; implement per [candle whisper example](https://github.com/huggingface/candle/tree/main/candle-examples/examples/whisper).
