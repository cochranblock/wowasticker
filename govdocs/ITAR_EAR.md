<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# ITAR / EAR Export Control Assessment

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## ITAR (International Traffic in Arms Regulations)

**Determination: Not applicable.**

wowasticker is not a defense article, defense service, or technical data as defined in the ITAR (22 CFR 120-130). It is a civilian educational application for tracking student behavioral goals using a sticker chart model.

| ITAR Criteria | Assessment |
|--------------|------------|
| Defense article (USML) | No — not on the United States Munitions List |
| Defense service | No — no military application |
| Technical data | No — no controlled technical information |
| Weapons system | No |
| Military application | No |

## EAR (Export Administration Regulations)

### Classification: EAR99

wowasticker is classified as **EAR99** — items subject to the EAR that are not listed in the Commerce Control List (CCL).

| EAR Criteria | Assessment |
|-------------|------------|
| CCL category | Not listed |
| ECCN | EAR99 (no ECCN assigned) |
| License required | No (for most destinations) |
| Encryption | None used |

### EAR Category 5 Part 2 (Encryption)

**Not applicable.** wowasticker does not contain, use, or implement any cryptographic functionality.

| Encryption Check | Result |
|-----------------|--------|
| Symmetric encryption | None |
| Asymmetric encryption | None |
| Key exchange | None |
| Hashing for security | None |
| Digital signatures | None |
| TLS/SSL implementation | None |
| Cryptographic libraries linked | None |

The `rand` crate (included with the `candle` feature) is used for random sampling during Whisper inference. This is not a cryptographic use and does not trigger Category 5 Part 2 controls.

### Open Source Exception

Even if cryptographic functionality were added in the future, the project may qualify for the publicly available / open source exception under EAR 742.15(b). The source code is:

- Publicly available on GitHub
- Licensed under the Unlicense (public domain dedication)
- Freely accessible without restriction

Per BIS (Bureau of Industry and Security) guidance, publicly available encryption source code may be exported under License Exception TSU (Technology and Software Unrestricted) with notification to BIS.

## Summary

| Regulation | Applicable | Classification |
|-----------|------------|---------------|
| ITAR | No | Not a defense article |
| EAR | Yes (minimally) | EAR99 — no license required |
| EAR Cat 5 Part 2 | No | No encryption used |

No export license is required to distribute wowasticker to any destination, with the exception of fully sanctioned countries/entities per OFAC regulations (which apply to all US-origin items regardless of classification).

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
