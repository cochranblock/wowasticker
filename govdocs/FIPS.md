<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# FIPS 140-2/3 Applicability Assessment

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## Determination: Not Applicable

wowasticker does not use any cryptographic primitives. FIPS 140-2 and FIPS 140-3 validation is not applicable to this application.

## Cryptographic Operations Inventory

| Operation | Used | Notes |
|-----------|------|-------|
| Symmetric encryption (AES, etc.) | No | No data encryption |
| Asymmetric encryption (RSA, EC, etc.) | No | No key exchange |
| Hashing (SHA-256, etc.) | No | No integrity checks |
| Key derivation (PBKDF2, HKDF, etc.) | No | No keys |
| Digital signatures | No | No signing |
| Random number generation (CSPRNG) | No | `rand` crate used only for Whisper inference sampling, not for security purposes |
| TLS/SSL | No | No network connections |
| Message authentication (HMAC) | No | No authenticated messages |

## Data at Rest

SQLite database is stored in plaintext on-device. Data contents are sticker scores (0/1/2), student first names, schedule labels, and observation notes. No encryption is applied to the database.

Device-level encryption (iOS Data Protection, Android Full Disk Encryption) provides at-rest protection at the OS layer. This is outside the application's scope.

## Data in Transit

No data is transmitted. There is no "in transit" state for application data.

## Future Considerations

If database encryption is added in the future (e.g., SQLCipher or similar), the following FIPS-validated options exist in the Rust ecosystem:

| Library | FIPS Status | Notes |
|---------|-------------|-------|
| aws-lc-rs | FIPS 140-3 validated (AWS-LC) | Drop-in for ring in many cases |
| ring (FIPS build) | Not validated, but uses BoringSSL primitives | Would need separate validation |
| OpenSSL (via openssl crate) | FIPS 140-2 validated (OpenSSL FIPS module) | Adds C dependency |

If encryption is required for federal deployment, aws-lc-rs with its FIPS-validated backend would be the recommended path.

## References

- NIST FIPS 140-2: Security Requirements for Cryptographic Modules
- NIST FIPS 140-3: Security Requirements for Cryptographic Modules (supersedes 140-2)
- CMVP (Cryptographic Module Validation Program): https://csrc.nist.gov/projects/cryptographic-module-validation-program

---

*Part of [wowasticker](https://github.com/cochranblock/wowasticker) — [CochranBlock](https://cochranblock.org) zero-cloud architecture. Unlicense.*
