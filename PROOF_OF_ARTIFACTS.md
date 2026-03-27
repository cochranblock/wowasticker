<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Concrete evidence that this project works, ships, and is real.*

> Voice dictation → on-device AI → behavioral sticker scoring. No cloud. No privacy leaks.

## Architecture

```mermaid
flowchart TD
    User[Educator / Parent] --> UI[Dioxus UI: Thumb-Zone Layout]
    UI --> Select[Select Schedule Block]
    Select --> Dictate[Tap: Dictate Observation]
    Dictate --> Audio[cpal: 10s Mic Capture]
    Audio --> Resample[16kHz Mono Resampling]
    Resample --> Whisper[Candle Whisper-Tiny GGUF]
    Whisper --> Parse[Heuristic Parser]
    Parse --> Score[Sticker Score: 0 / 1 / 2]
    Score --> DB[(rusqlite: On-Device)]
    UI --> Display[Confirmation: Math ●● — saved!]
```

## Build Output

| Metric | Value |
|--------|-------|
| Lines of Rust | 1141 across 7 files (6 modules + test binary) |
| AI model | Whisper-Tiny GGUF (on-device, no cloud) |
| UI framework | Dioxus 0.5 (pure Rust, mobile-native) |
| Audio | cpal (cross-platform mic capture) |
| Storage | rusqlite (bundled SQLite, zero external deps) |
| Unit tests | 33 (parser heuristics, DB operations, student CRUD, sticker records, behavior tags, audio stubs) |
| Quality gate | TRIPLE SIMS via exopack (3-pass determinism) |
| Schedule blocks | 5 (Cultural Arts, Community Circle, Math, Recess, Lunch) |
| Sticker values | 3-tier: 0 (concern), 1 (good), 2 (great) |
| Behavior tags | 6 (elopement, refusal, combative, stay_in_space, finish_work, positive) |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| On-Device Whisper | Candle Whisper-Tiny GGUF — speech-to-text runs entirely on-device. No API calls, no privacy leaks |
| Heuristic Parser | great/excellent → 2 stickers, good/ok → 1 sticker, refusal/elopement → 0. Works even if Whisper fails |
| Thumb-Zone UI | All controls in bottom half of screen for one-handed use. Safe-area insets respect notches |
| Behavioral Tags | Auto-extracted from transcription: elopement, refusal, combative, finish_work, positive |
| TRIPLE SIMS | 3-pass test via exopack — real tempfile SQLite, no mocks |
| Student Profile | Default student with configurable goal_stickers — dynamic goal display in UI |
| Progress Counter | `count_stickers_today()` sums sticker values; UI shows "4 / 15 Stickers" with goal-met state |
| Transcription Display | After dictation, status shows block name + score + transcription text + behavior tags |
| Feature Gates | Tests run without audio/UI libs (--no-default-features) |

## How to Verify

```bash
cargo build --release -p wowasticker
cargo test -p wowasticker --no-default-features           # 33 tests
cargo run -p wowasticker --bin wowasticker-test --features tests  # TRIPLE SIMS
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
