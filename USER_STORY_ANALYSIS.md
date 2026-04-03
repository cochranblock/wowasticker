<!-- Unlicense — cochranblock.org -->

# User Story Analysis

*Full user walkthrough by a simulated real user — parent or teacher discovering this product.*

---

## 1. DISCOVERY

The README opens with a 12-line CochranBlock marketing banner about zero-cloud architecture. A teacher scanning this page first sees "Cloudflare tunnel" and "Unlicense Rust repositories," not "track your student's behavior." The actual product description — "Pure Rust, offline-first mobile app for student behavioral goals" — is buried on line 18 after a horizontal rule.

The architecture diagram is developer-facing (module names, data flow arrows). No screenshot, no GIF, no "here's what it looks like."

**10-second verdict:** A developer might get it. A teacher would bounce.

## 2. INSTALLATION

**Lib-only build (`--no-default-features`): PASS.** Compiles clean. 40 tests pass.

**Full Dioxus build:** Fails with `dioxus-desktop` 0.5.6 calling `open_devtools`/`close_devtools` on `wry` which no longer exposes those methods. `panic = 'abort'` in release profile also conflicts with Dioxus's catch-unwind patterns. No `Cargo.lock` checked in to pin working versions.

**The app binary cannot be built. A user cannot run it.**

## 3. FIRST USE (Happy Path — Code Reading)

Since the binary doesn't compile, this is a code walkthrough:

1. **Launch:** `main.rs` calls `dioxus::launch(ui::App)`. No CLI flags, no `--help`. Just opens a WebView.
2. **Screen:** Title "Luka's Sticker Chart." Shows "0 / 15 Stickers." Date navigation (< Today >).
3. **Schedule blocks:** 5 hardcoded cards: Cultural Arts, Community Circle, Math, Recess, Lunch. No way to customize.
4. **Tap a block:** Highlights blue. Only selection affordance.
5. **Tap "Dictate Observation":** Records 10s of audio. Countdown "Recording... 10s" to "1s." Transcribes via Whisper. Heuristic parser scores text.
6. **Result:** Status shows "Math: ●● saved! — 'text' [positive]"

**Critical:** Whisper transcription (`f137`) loads the model but never runs inference — `let _ = samples;` discards the audio. Always returns "Processed." The heuristic parser sees no trigger words in "Processed," so it always scores Zero. The core feature does not function.

## 4. SECOND USE CASE (Share Daily Report)

The "Share Daily Report" button generates clean plain text and copies via `navigator.clipboard.writeText()`. Format is good:

```
Luka's Sticker Report — 2026-03-27
Progress: 3 / 15 stickers

●● Math — Great: Great focus
○ Lunch — No observation

— Sent from WowaSticker
```

Useful if it worked. But since transcription always returns "Processed" and scores Zero, the report shows all zeros.

## 5. EDGE CASES

| # | Scenario | Result |
|---|----------|--------|
| 1 | No microphone | Error: "no default input device." Retry button appears but fails again. No mic permission guidance. |
| 2 | Whisper model missing | Silent degradation — returns "Processed." No warning, no download prompt. Default experience. |
| 3 | DB path invalid | "DB error: open db: /path." Entire app dead. No recovery. |
| 4 | Download script interrupted | Partial .gguf file. Candle will error on load. No checksum verification. |
| 5 | Clipboard API fails (non-HTTPS) | Silent failure — empty `.catch()`. User sees "copied!" but nothing on clipboard. |
| 6 | Schedule blocks not customizable | No UI to change. Must edit SQLite directly. |
| 7 | Student name change | No UI. Hardcoded "Luka" in default student. |
| 8 | download-whisper.sh shebang | Unlicense header on line 1, `#!/bin/bash` on line 3. Direct execution fails. Must use `bash scripts/download-whisper.sh`. |

## 6. FEATURE GAPS

1. **Multi-student support.** Every teacher has 20+ students.
2. **Customizable schedule blocks.** Every school has different periods.
3. **Manual sticker entry.** Tap-to-score without 10s dictation.
4. **Working speech-to-text.** Core feature is scaffolded, not implemented.
5. **Settings screen.** Student name, goal count, block CRUD.
6. **Export/print reports.** PDF, email, not just clipboard.
7. **Onboarding flow.** First-time setup for student name and schedule.
8. **Data backup/restore.** Phone dies → all data lost.
9. **Visual stickers.** Stars, smiley faces, not Unicode dots.
10. **Weekly/monthly trends.** Teachers need progress over time.

## 7. DOCUMENTATION GAPS

1. No screenshots or demo GIF.
2. Model download not mentioned in build instructions flow.
3. No "Getting Started" guide for non-developers.
4. Script shebang is broken (Unlicense header above `#!/bin/bash`).
5. No mobile build instructions (just a link to Dioxus docs).
6. Dioxus feature says "mobile" but builds produce desktop binary.

## 8. COMPETITOR CHECK

| Product | Price | Multi-Student | Voice Input | Offline |
|---------|-------|:---:|:---:|:---:|
| **ClassDojo** | Free | Yes | No | No |
| **GoalBook** | Paid | Yes | No | No |
| **Catalyst (DataFinch)** | Paid | Yes | No | No |
| **Bloomz** | Free | Yes | No | No |
| **Paper sticker chart** | $0 | Yes | N/A | Yes |
| **WowaSticker** | Free | No | Scaffolded | Yes |

WowaSticker's differentiator is voice dictation for hands-free observation during transitions. Real pain point. But the differentiator doesn't work, and the rest is below feature parity with free alternatives.

## 9. VERDICT

| Category | Score (1-10) | Notes |
|----------|:---:|-------|
| Usability | 2 | Cannot build. If it could: single-student, no customization, no settings. |
| Completeness | 3 | DB layer solid. UI structurally sound. Core feature (voice) is a stub. |
| Error Handling | 4 | DB/audio errors surface. Silent failures on missing model and clipboard. |
| Documentation | 3 | Architecture diagram good. No screenshots, broken instructions, script bug. |
| Would You Pay | 1 | Cannot run. Even if it ran, ClassDojo is free and does 100x more. |

## 10. TOP 3 FIXES

### Fix 1: Manual sticker entry (tap-to-score)

Bypass the broken voice pipeline. Add tap buttons (0/1/2) on each schedule card so teachers can record stickers without dictation. Makes the app immediately usable.

### Fix 2: Fix the download script shebang

Move the `#!/bin/bash` to line 1. Currently broken — Unlicense header is on line 1.

### Fix 3: Check in Cargo.lock

Pin dependency versions so the build is reproducible. The Dioxus 0.5 + wry version matrix is broken without pinned versions.

---

*Analysis performed 2026-03-27. Simulated user: classroom teacher evaluating behavior tracking tools.*

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
