<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### 30-Second Window Capture — Voice-to-Sticker in the Pickup Line (March 2026)

**Invention:** An offline mobile app designed for one specific moment: the 30 seconds between a child exiting the classroom and reaching the parent's car. Voice dictation captures behavioral observations, a heuristic parser extracts sticker-worthy behaviors, and the sticker score updates before the car door opens.

**The Problem:** Behavioral observation in education requires immediate capture. Teachers use paper checklists during class. Parents get end-of-day summaries. Nobody captures the transition moments — hallway behavior, peer interactions during pickup, self-regulation during handoff — because the window is 30 seconds and both hands are occupied (holding a child, a backpack, car keys).

**The Insight:** Voice input is the only interface that works when both hands are occupied and the window is 30 seconds. But voice-to-text is slow (cloud STT), imprecise (general-purpose models don't understand behavioral language), and requires network (school pickup zones have spotty cell service). The solution: offline Whisper on-device, a domain-specific behavioral parser, and a sticker scoring system designed by an educator — not a software engineer.

**The Technique:**
1. Dioxus mobile UI with thumb-zone layout — designed for one-handed operation
2. cpal microphone capture with 16kHz resampling for Whisper compatibility
3. Candle Whisper scaffold for offline STT (no cloud, no network required)
4. Heuristic behavior parser: extracts sticker-worthy phrases from transcription (e.g., "shared with friend" = 2 stickers, "hit another student" = -1 sticker)
5. Schedule blocks: morning/afternoon/pickup — context-aware scoring
6. Daily report: shareable summary of observations and sticker counts

**Result:** An educator captures a behavioral observation in the pickup line, the app transcribes and scores it, and the daily report updates — all in the time it takes the child to walk to the car. No internet required. No cloud dependency.

**Named:** 30-Second Window Capture
**Commit:** `5cb79ea` through `fce7ce2` (initial sprint)
**Origin:** Michael Cochran's experience as a parent and educator. The behavioral observation gap isn't a technology problem — it's a timing problem. The 30-second window exists every school day, twice a day, and nobody captures it because existing tools require a desk, a screen, and both hands.

### Sticker Economy Behavioral Model (March 2026)

**Invention:** A behavioral scoring system based on the sticker economy used in real classrooms — specific behaviors earn specific sticker counts, daily goals drive motivation, and the scoring rules are designed by an educator, not an algorithm.

**The Problem:** Behavioral tracking apps use generic rating scales (1-5 stars, thumbs up/down). These don't match how teachers actually track behavior. In real classrooms, specific behaviors earn specific rewards — "helped a classmate" = 2 stickers, "completed homework" = 1 sticker, "disrupted class" = -1 sticker. The scoring is domain-specific and varies by classroom.

**The Insight:** The sticker economy is already the lingua franca of elementary education. Every teacher uses it. Every child understands it. The app should use stickers as the native unit, not stars or points. And the scoring rules should be editable by the teacher — because a kindergarten class values different behaviors than a 5th-grade class.

**The Technique:**
1. Sticker values per behavior: positive behaviors earn stickers, negative behaviors subtract
2. Daily goal: configurable per student (e.g., 15 stickers/day)
3. Progress display: "4 / 15 Stickers" with visual indicator
4. Schedule-aware: behaviors are tagged with time block (morning, afternoon, pickup)
5. Undo: last observation can be undone (because kids are kids and sometimes the adult misheard)

**Result:** The scoring model matches what teachers already do on paper. No training required. No new mental model. Just the same sticker economy, digitized, with voice input.

**Named:** Sticker Economy Model
**Commit:** `287bd51` (student CRUD + progress), `e4a7e66` (daily report)
**Origin:** Every elementary classroom Michael Cochran's children attended used sticker charts. The behavioral model is the sticker chart — formalized into a database schema.

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with 30-Second Window Capture and Sticker Economy Model.
**Commit:** See git log
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

---

## Entries

### 2026-04-09 — Onboarding + Settings UI Wired (Kill Hardcoded "Luka")

**What:** Wired settings DB CRUD (f152-f156) to the Dioxus UI. New `OnboardingView` (f157) shows on first run when no student exists — name + daily goal entry. New `SettingsView` (f158) accessed via gear icon — edit student name/goal, add/rename/delete schedule blocks. Added `f156=create_student` to db.rs, replacing the hardcoded "Luka" path (`f140`) for the Dioxus desktop frontend. App component routes Onboarding → Main → Settings via new `t126` view-mode enum. Fixed compile by wrapping `db` prop in `use_signal` so 4 closures (Save / Rename / Delete / Add) can each access the `Option<Arc<t123>>` without moving. Bumped Cargo edition to 2024. Tests: 151 total (129 lib + 12 ui + 10 CLI integration), all pass. Closes BACKLOG item 1.
**Commit:** `eebeabc`
**AI Role:** AI implemented the wiring, fixed the move errors, and ran the test suite. Human directed the BACKLOG priority and the "no hardcoded names" requirement.

### 2026-04-01 — IRONHIVE Bootstrap + Honest README

**What:** Bootstrapped Rust 1.94.x on 3 IRONHIVE nodes (lf, gd, st). Swarm-verified 40 tests + Linux x86_64 release build (1.5 MB on gd). Full guest audit scored app: DB 9/10, code quality 9/10, voice 0/10 (stub). Found `sticker_records` has no `student_id` — multi-student broken at schema level. Updated README with honest "What Works" vs "What's Scaffolded" sections. Added cochranblock.org headers/footers to all 11 govdocs. Cross-linked POA, TOI, and USER_STORY_ANALYSIS. Planned next phase: settings → Whisper inference → multi-student.
**Commits:** `dc967fe`, current session
**AI Role:** AI ran full guest audit, IRONHIVE bootstrap, and doc updates. Human directed swarm resource allocation and approved next-phase plan.

### 2026-03-30 — Polish Pass

**What:** Updated TOI with 4 new entries, POA with current metrics (2,106 LOC, 5 release assets, 6 new key artifacts). Added f148-f151 to compression map. Fixed .gitignore (android/build/, .DS_Store, vendor/). Verified: clippy -D warnings PASS, 40 tests PASS, TRIPLE SIMS 3x PASS.
**Commit:** `3e81290`
**AI Role:** AI ran full polish audit. Human directed the verification protocol.

### 2026-03-29 — Android AAB + APK Built

**What:** Real Play Store-ready AAB (4.6 MB) and sideload APK (5.8 MB). Fixed Java syntax error in MainActivity, generated Gradle wrapper, built libwowasticker.so via cargo-ndk. Both uploaded to GitHub Release v0.2.0.
**Commit:** `84678de`
**AI Role:** AI built the full Android pipeline. Human provided NDK path and directed Play Store targeting.

### 2026-03-29 — PWA + 12-Platform Build System

**What:** Installable PWA with offline service worker, JS fallback (works without WASM), manifest.json, SVG icons. WASM bridge module (f151). Master build script for 12 targets. macOS Intel binary added to release.
**Commit:** `9862097`
**AI Role:** AI built PWA, WASM bridge, and build scripts. Human directed platform coverage.

### 2026-03-29 — Multi-Arch Release + Android Scaffold

**What:** Linux x86_64 binary built on st via vendored tarball. Android project structure (Gradle, JNI bridge, WebView MainActivity). crate-type cdylib for .so generation. Crates.io metadata added to Cargo.toml.
**Commit:** `7d874b6`
**AI Role:** AI built cross-platform infrastructure. Human directed st deployment and Android package name.

### 2026-03-28 — CLI Binary: Working Demo + Runtime Govdocs

**What:** New wowasticker-cli binary — first working entry point. Full sticker workflow demo (create→score→report→undo→history). 11 govdocs baked via include_str!(). SPDX 2.3 SBOM parsed live from baked Cargo.toml. 1.3 MB binary with bundled SQLite + compliance docs.
**Commit:** `785bd3d`
**AI Role:** AI built the CLI. Human directed the dogfooding pattern (binary IS the compliance artifact).

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

## See Also

- [Proof of Artifacts](PROOF_OF_ARTIFACTS.md) — build output, metrics, QA results
- [User Story Analysis](USER_STORY_ANALYSIS.md) — simulated teacher walkthrough

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
