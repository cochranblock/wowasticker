<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-27 — Federal Compliance Suite

**What:** 11 govdocs for federal procurement readiness: SBOM (EO 14028), SSDF (NIST 800-218), supply chain integrity, security posture, Section 508 accessibility, PRIVACY (GDPR/CCPA/COPPA/FERPA), FIPS status, FedRAMP notes, CMMC mapping, ITAR/EAR classification, federal use cases (DoDEA, VA, Head Start, BIE, BOP).
**Commit:** `7d8656b`
**AI Role:** AI wrote all 11 documents. Human directed scope and agency targeting.

### 2026-03-27 — User Story Analysis + Top 3 Fixes

**What:** Full user walkthrough as simulated classroom teacher. Scored: Usability 2/10, Completeness 3/10. Core finding: voice pipeline is scaffolded but non-functional. Fixes: (1) tap-to-score buttons on each card (manual sticker entry without voice), (2) download script shebang fix, (3) Cargo.lock checked in for reproducible builds.
**Commit:** `19a1071`
**AI Role:** AI performed simulated user testing and implemented fixes. Human directed the evaluation framework.

### 2026-03-27 — P13 Kova Tokenization

**What:** Applied Kova P13 compression mapping to all source code. Renamed 7 types (t119-t125), 29 functions (f119-f147), 18 struct fields (s0-s17). Updated compression map. Added `#![allow(non_camel_case_types)]` for P13 convention.
**Commits:** `baa1b85`, `ae27b08`
**AI Role:** AI executed mechanical rename across all files. Human specified the compression convention.

### 2026-03-27 — QA Red Alert + Dioxus Build Fix

**What:** Full QA audit. Found: binary didn't compile with Dioxus features (32 errors). Fixed: PartialEq for compressed types, Signal mutability, replaced std::thread with tokio for countdown, extracted rsx expressions, fixed main.rs dead imports. Verified: cargo build --release, clippy -D warnings, TRIPLE SIMS 3x pass.
**Commit:** `4b1e3ea`
**AI Role:** AI found and fixed all compilation issues. Human directed the QA protocol.

### 2026-03-27 — Binary Size Contest

**What:** Release profile: opt-level 'z', LTO, codegen-units 1, panic abort, strip. Slimmed tokio from "full" to "rt,macros,time". Dropped unused chrono serde feature. Result: 420 KB rlib, 313 KB test binary (aarch64).
**Commits:** `baa1b85` (included in P13 commit)
**AI Role:** AI applied optimizations. Human set competitive target.

### 2026-03-27 — Shippable Product: Daily Report + History + Undo

**What:** New report module (f147 generate_daily_report). Date navigation for browsing history. ScheduleCard shows observation notes. Share Daily Report copies to clipboard. Undo removes last dictation. 3 new DB methods (f144-f146), 7 new tests. 9 of 10 UI/UX gaps closed.
**Commit:** `e4a7e66`
**AI Role:** AI built all features. Human directed the feature priorities based on educator workflow.

### 2026-03-27 — Backlog Build: Student CRUD + Progress

**What:** 4 new DB methods (f140-f143): student CRUD, sticker counting, full record retrieval. UI: dynamic goal from Student.goal_stickers, "4 / 15 Stickers" progress counter, loading skeleton, transcription display with tags. 6 new tests (33 total at time).
**Commit:** `287bd51`
**AI Role:** AI built all features. Human identified backlog from UI/UX analysis.

### 2026-03-27 — Doc Sync

**What:** Updated README (module table, build commands, model path), UI/UX analysis (4 gaps resolved), PROOF_OF_ARTIFACTS (test count, tag count, line count), CLAUDE.md (build commands). Fixed exopack dep from local path to git. Flattened if-let in ai.rs.
**Commit:** `04b8255`
**AI Role:** AI audited and fixed all stale documentation. Human directed the review scope.

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
