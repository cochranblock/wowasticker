<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Backlog

*Prioritized stack. Most important at top. Max 20 items.*
*Self-reorganizes based on recency and relevance when idle.*

Last triaged: 2026-04-03

---

1. `[fix]` Fix Dioxus desktop build — wry API mismatch (`open_devtools`/`close_devtools` removed). Switch `features = ["mobile"]` to `["desktop"]` or add feature split. Do NOT upgrade to Dioxus 0.6.
2. `[feature]` Settings screen — add `view_mode` signal (Main/Settings) to ui.rs. Student name, goal count, block add/rename/delete.
3. `[feature]` DB CRUD for settings — `update_student`, `add_block`, `delete_block`, `rename_block` in db.rs. Tests for each. **DONE** — f152-f155 implemented and tested.
4. `[feature]` First-run onboarding — if no student exists, show setup screen. Replace hardcoded "Luka" default.
5. `[fix]` Schema: add `student_id` to `sticker_records` — current PK is `(block_id, date)` with no student association. Multi-student broken at schema level.
6. `[feature]` Multi-student UI — student selector (chips or dropdown), wire to filtered queries. Dep: item 5.
7. `[fix]` IRONHIVE node lf DNS — intermittent `github.com` resolution failures. Blocks `cargo clippy` (exopack git dep). Dep: exopack (cochranblock/exopack).
8. `[fix]` IRONHIVE node st DNS — same github.com resolution failure as lf. Blocks TRIPLE SIMS on st.
9. `[build]` IRONHIVE CI — automate `cargo test --no-default-features` on push across lf/gd/st. Dep: items 7-8 (DNS fix).
10. `[test]` Whisper integration test — gated on `WOWASTICKER_WHISPER_PATH` env var. Download model to one IRONHIVE node, test against known audio.
11. `[fix]` Clipboard silent failure — ui.rs `navigator.clipboard.writeText()` has empty `.catch()`. Surface error to user.
12. `[fix]` Missing Whisper model silent degradation — f119 returns "Processed" with no warning. Add status message when model not found.
13. `[feature]` Export daily report — PDF or CSV, not just clipboard. Teacher needs printable artifact.
14. `[feature]` Weekly/monthly progress trends — teachers need progress over time, not just daily view.
15. `[docs]` Add P23 protocol reference to project docs. Blocked: need P23 definition from kova protocols.
16. `[build]` Pull Linux x86_64 release binary from gd — built at `gd:~/wowasticker/target/release/wowasticker-cli` (1.5 MB). Add to GitHub Release.
17. `[test]` Accessibility audit — no ARIA labels, no keyboard nav, no screen reader testing. See govdocs/ACCESSIBILITY.md gap list.
18. `[research]` Dioxus 0.6 migration scope — breaking RSX/hooks changes. Estimate LOC impact on ui.rs (475 lines). Do not execute yet.
19. `[fix]` IRONHIVE node bt offline — SSH timeout. Diagnose: power? network? disk?

## Cross-Project Dependencies

| Dependency | Project | Impact |
|------------|---------|--------|
| exopack | [cochranblock/exopack](https://github.com/cochranblock/exopack) | TRIPLE SIMS test framework. Git dep — IRONHIVE nodes with DNS issues can't fetch it. |
| approuter | cochranblock/approuter | Deploy target for web builds (gd). Not blocking current work. |
| kova | cochranblock/kova | IRONHIVE C2 orchestration. Kova binary on Mac is Linux ELF (wrong arch). macOS build compiles lib only (needs `--features serve` for binary). |
| kova protocols | ~/.cursor/protocols/ | P23 definition needed for item 16. |

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
