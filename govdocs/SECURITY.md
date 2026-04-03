<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Security Assessment

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## Overview

wowasticker is an offline-first mobile app for tracking student behavioral goals using a sticker chart model. It stores sticker scores (0/1/2), student names, schedule blocks, and observation notes in a local SQLite database.

## Cryptography

**None used.** This application does not perform:
- Encryption or decryption
- Hashing (cryptographic)
- Key derivation
- Digital signatures
- TLS/SSL (no network connections)
- Token generation or validation

## Secrets

**None.** The application has:
- No API keys
- No passwords
- No authentication tokens
- No certificates
- No environment variables containing secrets
- No `.env` files

## Authentication and Authorization

**None.** The application has:
- No login
- No user accounts
- No role-based access
- No session management

Access control is provided by the device OS (device lock screen, app sandboxing).

## Network Activity

**None at runtime.** The application makes zero network calls during normal operation.

Exception: when built with the `candle` feature, a one-time optional download of the Whisper model from HuggingFace Hub occurs on first use of voice input. This is user-initiated and does not transmit any application data.

## Attack Surface Analysis

### 1. Local SQLite Database

- **Threat:** Unauthorized access to student data on device
- **Mitigation:** Data stored in app sandbox (iOS/Android). OS-level file permissions. No network exfiltration path.
- **Data sensitivity:** Low. Sticker scores (0/1/2), student first names, schedule block labels, free-text observation notes, dates.
- **Encryption at rest:** None. SQLite database is plaintext. Data contents (sticker chart scores) are not classified as sensitive PII.

### 2. WebView (Dioxus)

- **Threat:** JavaScript injection via WebView eval
- **Mitigation:** No user input is interpolated into JavaScript strings. Clipboard operations use fixed JS snippets with no dynamic content. Dioxus manages the WebView bridge — application code does not construct raw JS.
- **Status:** Low risk. No `eval()` with user data.

### 3. Audio Capture (optional, candle feature)

- **Threat:** Unauthorized microphone access
- **Mitigation:** OS-level permission prompt required. Audio is processed locally via Candle Whisper. No audio data is transmitted over network. Audio is not persisted to disk — only the transcribed text is stored.

### 4. Whisper Model (optional, candle feature)

- **Threat:** Malicious model file
- **Mitigation:** Model downloaded from HuggingFace Hub (HTTPS). Model is used for inference only (no code execution). Candle processes model weights as numerical tensors.

## PII Assessment

| Data Element | Stored | Transmitted | Sensitivity |
|-------------|--------|-------------|-------------|
| Student first name | Yes (SQLite) | No | Low |
| Schedule block labels | Yes (SQLite) | No | None |
| Sticker scores (0/1/2) | Yes (SQLite) | No | Low |
| Observation notes (free text) | Yes (SQLite) | No | Low-Medium |
| Dates/timestamps | Yes (SQLite) | No | None |

**No PII is transmitted.** All data remains on-device.

## Observation Notes Risk

Free-text observation notes could contain sensitive information if the user writes it (e.g., behavioral descriptions, medical references). This is user-generated content stored locally. The app does not parse, validate, or transmit this content.

## Vulnerability Management

- `cargo audit` checks RustSec advisory database (manual, not yet automated)
- `cargo clippy -D warnings` enforces zero warnings
- Rust compiler provides memory safety guarantees
- No `unsafe` blocks in application code
- Dependency updates via `cargo update` with Cargo.lock review

## Incident Response

Not applicable in the traditional sense — there is no server, no network service, no cloud component. A vulnerability in the app would be addressed via a new build from updated source.

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
