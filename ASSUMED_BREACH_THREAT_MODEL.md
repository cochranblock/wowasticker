# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (project-specific — adapt below)

**wowasticker context:** Offline-first mobile/desktop app for K-12 behavioral observation. Emits education records (student names, sticker scores 0/1/2, observation notes, daily reports) into a local SQLite database. Optional on-device voice dictation via Candle Whisper-Tiny GGUF captures 10s audio → transcribes → discards buffer. **No network egress for correctness.** Primary deployment context: a teacher's phone, tablet, or Chromebook in a classroom.

### Records of consequence this project emits

| Record | Sensitivity | Storage | Transmitted |
|--------|-------------|---------|-------------|
| Student first name | FERPA education record (if school context) / COPPA PII (under-13) | `wowasticker.db` SQLite on-device | Never |
| Sticker scores (0/1/2 per block per day) | FERPA education record — behavioral assessment | `wowasticker.db` SQLite on-device | Never |
| Observation notes (free text from voice/typing) | FERPA education record — may contain staff impressions of student behavior | `wowasticker.db` SQLite on-device | Never |
| Daily report (plain text) | FERPA education record | In-memory → OS clipboard (user-initiated "Share Daily Report") | Never by the app; user may paste anywhere |
| Audio buffer | Ephemeral; may incidentally capture non-consenting third parties (other students, staff, parents) | 10s RAM buffer during `capture_audio()` | Never; discarded after `transcribe_audio()` |
| Whisper model artifacts (`model-tiny-q4k.gguf`, `tokenizer-tiny.json`, `melfilters.bytes`) | Not sensitive, but integrity-critical — tampered model can misclassify observations | Local filesystem, user-downloaded via `scripts/download-whisper.sh` from HuggingFace | One-time download only |

### In-scope threats

- **Physical device seizure / loss / theft.** Teachers carry phones and tablets home and into shared spaces. A lost device = disclosure of every sticker record and observation note on it. This is the primary threat vector.
- **Shared-device compromise.** A classroom iPad or Chromebook is used by multiple adults (substitutes, aides, admins). Any user of the device inherits read access to every student's history unless OS-level user accounts are enforced.
- **Clipboard exfiltration.** "Share Daily Report" writes FERPA-protected content to the OS clipboard. Any background app with clipboard-read permission receives the report. On mobile, iOS 14+ and Android 12+ surface clipboard reads but do not block them.
- **Audio-capture misuse.** The 10s mic buffer may incidentally record student voices, peer conversations, or adult bystanders who did not consent. A compromised build could persist or exfiltrate the buffer instead of discarding it.
- **Voice-model tampering.** A swapped or poisoned `model-tiny-q4k.gguf` can systematically mis-score behavior (e.g., always emit "2" regardless of transcript). The GGUF file is not signature-verified at load time.
- **SQLite at rest.** `wowasticker.db` is not application-encrypted. Protection depends entirely on OS-level full-disk encryption (iOS/Android FDE, FileVault, LUKS). If the device is unlocked or encryption is off, records are readable with any SQLite browser.
- **Backup exfiltration.** iOS/Android platform backups (iCloud, Google One) and desktop Time Machine / rsync backups capture `wowasticker.db` into third-party systems outside this app's control.
- **Supply chain (deps).** 602 pinned crates via `Cargo.lock`. A compromised dep (especially `rusqlite`, `cpal`, or `candle-*`) could introduce an exfiltration path or a silent behavior-score skew.
- **Insider / self-tampering by educator.** An educator could rewrite past sticker scores to match a narrative (e.g., "student was fine until the incident") — either to mislead a parent or to avoid scrutiny. The current schema permits row updates.
- **Clock manipulation.** Date navigation treats past days as read-only based on the device clock. A rewound device clock makes "past" days editable again, enabling backdated fabrication.
- **Android JNI / PWA / WASM frontends.** The `jni.rs`, `wasm.rs`, and PWA frontends are alternate entry points to the same SQLite layer. A compromise of any one frontend reaches the whole database.

### Out-of-scope / N/A

- **Network MITM.** N/A for correctness. The app has zero network egress for user data. The only network path is a user-initiated one-time HuggingFace download of the Whisper model, which transmits no user data. If future work adds cloud sync, revisit.
- **Public-chain hash publishing to `cochranblock/wowasticker-chain`.** **Deliberately N/A.** Publishing hashes of per-day student behavioral records — even as opaque BLAKE3 digests — creates a FERPA exposure surface (disclosure of record existence / cadence / cardinality to third parties) that outweighs the tamper-evidence benefit. External-witness requirements for education records, if ever needed, belong inside the school district's records-management system (SIS), not a public git repo. Tamper-evidence for wowasticker stays local: append-only schema + device-local hash chain, verifiable by the record custodian (teacher/school).
- **Hardware-key signing of daily records.** N/A for the expected deployment profile. Teachers using phones mid-classroom cannot reasonably invoke a YubiKey touch per sticker entry. Hardware signing remains in scope for release artifacts (see `PROOF_OF_ARTIFACTS.md`), not per-record.
- **Server-side audit log.** N/A. There is no server. Audit trail requirements are satisfied by append-only SQLite + device-local hash chain.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
