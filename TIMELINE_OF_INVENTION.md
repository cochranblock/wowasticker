<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-11 — Foundational Founders v0.2.0

**What:** Unlicense headers, contributor attribution locked (6 co-authors), version bump.
**Commit:** `12674df`
**AI Role:** AI applied headers. Human decided licensing and attribution model.

### 2026-03-11 — UI/UX Audit + macOS Build

**What:** UI/UX analysis documenting strengths (thumb-zone layout, clear hierarchy) and 10 gaps (no recording countdown, weak selection affordance). macOS build notes.
**Commit:** `6fd2ebd`
**AI Role:** AI performed UX audit. Human validated findings against real classroom usage scenarios.

### 2026-03-10 — Dependency Refactor

**What:** Moved exopack from local path to kova workspace reference.
**Commit:** `473c061`
**AI Role:** AI fixed dependency path. Human decided workspace structure.

### 2026-03-03 — Full App in One Sprint

**What:** Complete app: Dioxus UI with thumb-zone layout, cpal mic capture with 16kHz resampling, Candle Whisper scaffold, heuristic behavior parser, rusqlite persistence, schedule blocks, sticker scoring, TRIPLE SIMS test binary, compression tokenization (f119-f139).
**Why:** An educator needed in-the-moment behavioral observation capture during school pickup. Voice → sticker scoring had to work offline on a phone.
**Commits:** `5cb79ea` through `fce7ce2`
**AI Role:** AI generated all modules in one sprint. Human designed the behavioral science model, sticker values, schedule blocks, and UX flow based on real classroom experience.

---

*Built by educators, for educators. Every design decision serves the 30-second window between classroom exit and car pickup.*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
