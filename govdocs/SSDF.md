# NIST SP 800-218 — Secure Software Development Framework (SSDF)

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## PS — Prepare the Organization

| Practice | Implementation | Status |
|----------|---------------|--------|
| PS.1 — Security requirements | Offline-first architecture eliminates network attack surface. No auth, no crypto, no secrets. | Done |
| PS.2 — Roles and responsibilities | Single maintainer (cochranblock.org). Unlicense — public domain dedication. | Done |
| PS.3 — Supporting toolchains | Rust compiler (memory safety by default), cargo clippy, cargo audit. | Done |

### Language Choice

Rust provides compile-time memory safety guarantees:
- No null pointer dereferences (Option type)
- No buffer overflows (bounds checking)
- No use-after-free (ownership model)
- No data races (borrow checker)

This eliminates the majority of CVE classes found in C/C++ applications (per Microsoft Security Response Center: 70% of CVEs are memory safety issues).

## PW — Produce Well-Secured Software

| Practice | Implementation | Status |
|----------|---------------|--------|
| PW.1 — Secure design | On-device SQLite only. No network at runtime. No IPC. Minimal attack surface. | Done |
| PW.2 — Review design | Architecture reviewed: Dioxus WebView + SQLite + optional Candle Whisper. No server component. | Done |
| PW.4 — Reuse secure components | All deps from crates.io (MIT/Apache-2.0). No vendored binaries. No C FFI except SQLite (public domain, bundled). | Done |
| PW.5 — Secure coding | `cargo clippy -D warnings` — zero warnings policy. No `unsafe` blocks in application code. | Done |
| PW.6 — Build process | Cargo with Cargo.lock pinning. Deterministic builds from source. Feature gates isolate optional deps. | Done |
| PW.7 — Test | TRIPLE SIMS 3-pass testing gate. Compilation, unit, integration stages. Test binary is CI pipeline. | Done |
| PW.8 — Fix vulnerabilities | Dependency updates via `cargo update`. `cargo audit` for known CVEs (not yet automated). | Partial |
| PW.9 — Archive | Git repository on GitHub. Tagged releases. Cargo.lock committed. | Done |

### Feature Gate Isolation

Optional functionality is behind Cargo feature flags:
- `dioxus` — UI framework, only compiled when building the app
- `candle` — Whisper inference, only compiled when voice input needed
- `audio` — Microphone capture, only compiled with candle
- `tests` — Test framework, never included in release builds

This means a minimal build (`--no-default-features`) includes only: anyhow, chrono, rusqlite, serde, serde_json, tokio.

## RV — Respond to Vulnerabilities

| Practice | Implementation | Status |
|----------|---------------|--------|
| RV.1 — Identify vulnerabilities | `cargo audit` checks RustSec advisory database. Not yet in CI. | Partial |
| RV.2 — Assess vulnerabilities | Minimal attack surface (offline app) limits exploitable paths. | Done |
| RV.3 — Remediate | `cargo update` for dependency patches. Cargo.lock ensures version pinning between updates. | Done |

### Dependency Pinning

`Cargo.lock` is committed to the repository. Every build uses exact same dependency versions. Updates are explicit via `cargo update` and reviewed before committing the updated lockfile.

## PO — Protect Operations

| Practice | Implementation | Status |
|----------|---------------|--------|
| PO.1 — Protect software | On-device only. No cloud deployment. No server to secure. | N/A (offline) |
| PO.2 — Provide software info | This document. SBOM provided. Source available on GitHub. | Done |
| PO.3 — Monitor and respond | No runtime telemetry. No network. No monitoring surface. | N/A (offline) |

## Summary

The offline-first, no-network architecture of wowasticker makes many SSDF practices either straightforward (no server to secure) or not applicable (no cloud deployment). The primary security controls are: Rust memory safety, dependency pinning, clippy enforcement, and the TRIPLE SIMS test gate.
