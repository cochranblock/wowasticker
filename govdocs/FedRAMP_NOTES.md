<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# FedRAMP Applicability Assessment

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## Determination: Not Applicable

FedRAMP (Federal Risk and Authorization Management Program) applies to cloud service offerings (CSOs) used by federal agencies. wowasticker is not a cloud service.

## Deployment Model

| Attribute | Value |
|-----------|-------|
| Deployment type | Standalone mobile/desktop application |
| Cloud component | None |
| Server component | None |
| SaaS | No |
| PaaS | No |
| IaaS | No |
| Data storage | Local SQLite on-device only |
| Network requirement | None (offline-first) |

## Why FedRAMP Does Not Apply

FedRAMP authorizes cloud service providers (CSPs) that process, store, or transmit federal information. wowasticker:

1. **Is not a cloud service.** It runs entirely on the user's device.
2. **Has no server component.** There is no backend, no API, no hosted infrastructure.
3. **Does not transmit data.** No federal information leaves the device.
4. **Has no authorization boundary.** FedRAMP requires defining a boundary around cloud infrastructure. wowasticker has no infrastructure.
5. **Is not multi-tenant.** Each installation is isolated to one device.

## Alternative Authorization Paths for Federal Use

If a federal agency wishes to deploy wowasticker on government devices, the relevant authorization frameworks are:

| Framework | Applicability |
|-----------|--------------|
| Agency ATO (Authority to Operate) | Yes — agency IT evaluates the app for deployment on managed devices |
| DISA STIG | Potentially — if deployed on DoD devices, device-level STIGs apply |
| RMF (Risk Management Framework) | Yes — NIST SP 800-37 applies to the information system (the device), not the app in isolation |
| FedRAMP | No — not a cloud service |

## Recommendation

Federal agencies should evaluate wowasticker under their standard software approval process for on-device applications. The SBOM, SSDF, and Security documents in this directory provide the information needed for that evaluation.

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
