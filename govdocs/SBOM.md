# Software Bill of Materials (SBOM)

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

EO 14028 — Executive Order on Improving the Nation's Cybersecurity
Project: wowasticker
Generated: 2026-03-27
Format: Human-readable (CycloneDX/SPDX export available via `cargo sbom`)

## Supplier

- Organization: cochranblock.org
- Repository: https://github.com/cochranblock/wowasticker
- License: Unlicense (public domain)

## Core Dependencies (lib, --no-default-features)

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| anyhow | 1.0.102 | MIT OR Apache-2.0 | Error handling |
| chrono | 0.4.44 | MIT OR Apache-2.0 | Date/time for schedules and records |
| rusqlite | 0.32.1 | MIT | SQLite bindings (bundles SQLite, public domain) |
| serde | 1.0.228 | MIT OR Apache-2.0 | Serialization framework |
| serde_json | 1.0.149 | MIT OR Apache-2.0 | JSON serialization |
| tokio | 1.50.0 | MIT | Async runtime |

## Optional Dependencies (feature-gated)

| Crate | Version | License | Feature Gate | Purpose |
|-------|---------|---------|--------------|---------|
| dioxus | 0.5.6 | MIT OR Apache-2.0 | `dioxus` | UI framework (WebView) |
| candle-core | 0.8.4 | MIT OR Apache-2.0 | `candle` | Tensor operations for Whisper |
| candle-nn | 0.8.4 | MIT OR Apache-2.0 | `candle` | Neural network layers |
| candle-transformers | 0.8.4 | MIT OR Apache-2.0 | `candle` | Whisper model implementation |
| cpal | 0.15.3 | Apache-2.0 | `audio` | Audio capture from microphone |
| tokenizers | 0.19.1 | Apache-2.0 | `candle` | HuggingFace tokenizer for Whisper |
| rand | 0.8.5 | MIT OR Apache-2.0 | `candle` | Random number generation |
| exopack | latest | Unlicense | `tests` | Test framework (test builds only) |

## License Summary

- MIT OR Apache-2.0: 10 crates
- MIT only: 1 crate (rusqlite)
- Apache-2.0 only: 2 crates (cpal, tokenizers)
- Unlicense: 1 crate (exopack, test-only)
- Public domain: 1 bundled (SQLite via rusqlite)

**No GPL. No copyleft. No viral licenses.** All dependencies are permissive.

## Bundled Components

| Component | Version | License | Bundled By |
|-----------|---------|---------|------------|
| SQLite | 3.x (as bundled by rusqlite 0.32.1) | Public Domain | rusqlite `bundled` feature |

## Build Toolchain

| Tool | Version | Purpose |
|------|---------|---------|
| rustc | stable | Compiler |
| cargo | stable | Build system and package manager |

## Verification

All dependencies sourced from crates.io. Versions pinned in `Cargo.lock` (committed to repository). Dependency tree reproducible via `cargo tree`. Audit via `cargo audit`.
