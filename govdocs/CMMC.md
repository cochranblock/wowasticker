<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# CMMC (Cybersecurity Maturity Model Certification) Assessment

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27
CMMC Version: 2.0
Applicable Level: Level 1 (Foundational)

## Overview

CMMC Level 1 requires 17 practices derived from FAR 52.204-21. This assessment maps those practices to wowasticker's architecture. Many practices apply to the organization operating the software rather than the software itself. For an offline, single-user, on-device application, several domains are not applicable.

## Domain Assessments

### AC — Access Control

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| AC.L1-3.1.1 | Limit system access to authorized users | Device-level OS authentication (lock screen, biometrics). No app-level auth. | Device-dependent |
| AC.L1-3.1.2 | Limit system access to authorized functions | Single-role application. All users have full access to all features. | N/A (single user) |
| AC.L1-3.1.20 | Control connection of external systems | No network connections. No external system integration. | Satisfied by design |
| AC.L1-3.1.22 | Control information posted publicly | No public posting capability. No network. | Satisfied by design |

### AU — Audit and Accountability

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| AU.L1-3.3.1 | Create and retain system audit logs | SQLite records include timestamps (chrono crate). Sticker entries, student additions, and modifications are timestamped. | Partial |
| AU.L1-3.3.2 | Ensure actions can be traced to users | Single-user app — all actions are by the device holder. | Satisfied by design |

### AT — Awareness and Training

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| AT.L1-3.2.1 | Security awareness | N/A — single-user on-device app. No multi-user security context. | N/A |
| AT.L1-3.2.2 | Role-based training | N/A — no roles. | N/A |

### CM — Configuration Management

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| CM.L1-3.4.1 | Establish and maintain baseline configurations | Cargo.lock pins all dependency versions. Build is deterministic from source. | Satisfied |
| CM.L1-3.4.2 | Track and control changes | Git version control. All changes committed with history. | Satisfied |

### IA — Identification and Authentication

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| IA.L1-3.5.1 | Identify system users | N/A — no multi-user. Device OS identifies the user. | N/A |
| IA.L1-3.5.2 | Authenticate users | N/A — no authentication mechanism. Device-level auth only. | N/A |

### IR — Incident Response

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| IR.L1-3.6.1 | Establish incident handling capability | N/A — offline application with no network attack surface. Incidents are device-level (handled by device administrator). | N/A |
| IR.L1-3.6.2 | Track and report incidents | N/A — no centralized reporting. | N/A |

### MA — Maintenance

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| MA.L1-3.7.1 | Perform maintenance | `cargo update` for dependency updates. Rebuild and redeploy. | Satisfied |
| MA.L1-3.7.2 | Control maintenance tools | Cargo, rustc, clippy — standard Rust toolchain. | Satisfied |

### MP — Media Protection

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| MP.L1-3.8.3 | Sanitize media before disposal | N/A — no removable media. SQLite file deleted with app uninstall (OS handles). | N/A |

### PS — Personnel Security

Not applicable. Single-developer open-source project. No personnel access controls.

### PE — Physical Protection

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| PE.L1-3.10.1 | Limit physical access | Device holder's responsibility. Mobile device physical security. | User responsibility |

### SC — System and Communications Protection

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| SC.L1-3.13.1 | Monitor and protect communications | No communications to monitor. Zero network activity. | Satisfied by design |
| SC.L1-3.13.5 | Implement subnetworks for public access | N/A — no network component. | N/A |

### SI — System and Information Integrity

| Practice | Requirement | Implementation | Status |
|----------|------------|----------------|--------|
| SI.L1-3.14.1 | Identify and correct flaws | `cargo clippy -D warnings` for static analysis. `cargo audit` for known vulnerabilities. TRIPLE SIMS 3-pass test gate. | Satisfied |
| SI.L1-3.14.2 | Protect against malicious code | Rust memory safety. No `unsafe` blocks. All deps from crates.io with checksum verification. | Satisfied |
| SI.L1-3.14.4 | Update malicious code protection | `cargo audit` against RustSec advisory database. `cargo update` for patched dependencies. | Satisfied |
| SI.L1-3.14.5 | Perform system monitoring | N/A — no runtime monitoring surface for offline app. | N/A |

## Summary

wowasticker's offline, single-user, no-network architecture satisfies many CMMC Level 1 practices by design (no communications to protect, no external connections to control). Practices related to multi-user access, incident response, and personnel security are not applicable to a standalone device application.

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
