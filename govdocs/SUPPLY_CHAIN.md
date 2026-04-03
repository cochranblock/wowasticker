<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Software Supply Chain Security

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## Source Code

- Repository: https://github.com/cochranblock/wowasticker
- License: Unlicense (public domain)
- Language: Rust (100%)
- Build system: Cargo

## Dependency Sources

All dependencies are sourced from **crates.io**, the official Rust package registry.

| Dependency | Source | Verified |
|------------|--------|----------|
| anyhow 1.0.102 | crates.io | Yes |
| chrono 0.4.44 | crates.io | Yes |
| rusqlite 0.32.1 | crates.io | Yes |
| serde 1.0.228 | crates.io | Yes |
| serde_json 1.0.149 | crates.io | Yes |
| tokio 1.50.0 | crates.io | Yes |
| dioxus 0.5.6 | crates.io | Yes |
| candle-core 0.8.4 | crates.io | Yes |
| candle-nn 0.8.4 | crates.io | Yes |
| candle-transformers 0.8.4 | crates.io | Yes |
| cpal 0.15.3 | crates.io | Yes |
| tokenizers 0.19.1 | crates.io | Yes |
| rand 0.8.5 | crates.io | Yes |
| exopack (latest) | crates.io | Yes |

## Version Pinning

`Cargo.lock` is committed to the repository. This file records the exact version of every dependency (including transitive dependencies) used in the build. Any change to dependency versions results in a Cargo.lock diff that is reviewed before committing.

## No Vendored Binaries

- No pre-compiled binaries are checked into the repository
- No pre-built `.so`, `.dylib`, `.dll`, or `.a` files
- SQLite is compiled from source via rusqlite's `bundled` feature (SQLite source is C, public domain)
- All Rust dependencies are compiled from source during `cargo build`

## No Pre-Built Artifacts Distributed

- No binary releases published
- Users build from source
- No package manager distribution (no npm, no pip, no apt)

## Build Determinism

Given the same:
- Rust toolchain version
- Cargo.lock contents
- Source code commit
- Target platform

The build produces the same binary. Cargo's resolver, combined with Cargo.lock pinning, ensures dependency resolution is deterministic.

## Supply Chain Controls

| Control | Status |
|---------|--------|
| Deps from crates.io only | Yes |
| Cargo.lock committed | Yes |
| No vendored binaries | Yes |
| No git dependencies | Verify in Cargo.toml |
| `cargo audit` available | Yes (not yet automated) |
| No build scripts downloading binaries | Yes |
| No post-install scripts | Yes (Rust has none) |

## Transitive Dependency Audit

Run `cargo tree` to see the full dependency graph. Run `cargo audit` to check all transitive dependencies against the RustSec advisory database.

## crates.io Security Model

crates.io provides:
- Package signing (crate authors authenticate via GitHub OAuth)
- Immutable published versions (a published version cannot be modified, only yanked)
- Checksum verification (Cargo verifies SHA256 checksums on download)
- Public audit log (all publishes are public and timestamped)

## Whisper Model Download (Optional)

When the `candle` feature is enabled, the app can download a Whisper model from HuggingFace Hub on first use. This is the only network activity the app performs, and it is:
- User-initiated (not automatic)
- One-time (model is cached locally after download)
- Optional (candle feature must be explicitly enabled at compile time)

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
