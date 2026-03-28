# Privacy Impact Assessment

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## Data Inventory

| Data Element | Type | Storage | Transmitted | Retention |
|-------------|------|---------|-------------|-----------|
| Student name | Text (first name) | SQLite on-device | No | Until user deletes |
| Schedule blocks | Text (labels) | SQLite on-device | No | Until user deletes |
| Sticker scores | Integer (0, 1, 2) | SQLite on-device | No | Until user deletes |
| Observation notes | Free text | SQLite on-device | No | Until user deletes |
| Dates/timestamps | Date/datetime | SQLite on-device | No | Until user deletes |

## Data Flow

```
User Input → Dioxus UI → Rust Application Logic → SQLite (on-device)
                                                       ↑
                                              No network egress
                                              No cloud sync
                                              No analytics
                                              No telemetry
```

All data flows are local. Data enters via touch input (or optional voice via Whisper) and is stored in a SQLite database file on the device. Data never leaves the device.

## Network Activity

**Zero network transmission of user data.**

- No analytics SDK
- No telemetry endpoints
- No crash reporting
- No cloud sync
- No push notifications
- No ad networks

The only network activity is an optional, user-initiated, one-time download of the Whisper model file when the `candle` feature is enabled. This download transmits zero user data.

## GDPR Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Lawful basis | N/A | No data processing by any entity — all data local to device |
| Data minimization | Compliant | Only sticker-relevant data stored |
| Purpose limitation | Compliant | Data used solely for behavioral goal tracking |
| Storage limitation | User-controlled | User deletes data manually |
| Right to access | Compliant | User has direct access to all data on their device |
| Right to erasure | Compliant | User can delete any student record |
| Right to portability | Partial | SQLite file can be copied; no export UI yet |
| Data protection officer | N/A | No data processing organization |

**No data controller exists** because no entity collects or processes the data. The user's device is both the data controller and processor.

## CCPA Compliance

| Requirement | Status | Notes |
|-------------|--------|-------|
| Right to know | N/A | No business collects data |
| Right to delete | N/A | No business holds data |
| Right to opt-out of sale | N/A | No data sale — no data leaves device |
| Non-discrimination | N/A | No service conditioned on data sharing |

**No sale of personal information.** No personal information is collected by any business entity. All data resides on the user's device.

## COPPA Compliance

| Requirement | Status | Notes |
|-------------|--------|-------|
| Parental consent | Applicable | App stores student names — if used in a school setting with children under 13, the school/district acts as parental agent per FTC guidance |
| Data collection notice | N/A | No operator collects data from children |
| Data minimization | Compliant | Minimal data: name, scores, notes |
| Data security | Compliant | On-device only, no transmission |
| Data retention limits | User-controlled | No automatic retention policy |

### School Setting Guidance

When deployed in a K-12 school setting:
- The school or district acts as the parent's agent for COPPA consent (per FTC "COPPA and Schools" guidance)
- No data is transmitted to any operator or third party
- The school controls the device and the data on it
- No additional COPPA compliance burden exists because no "operator" collects data

## FERPA Applicability

If used in a school context, student records (names, behavioral scores, observation notes) may constitute "education records" under FERPA. Because all data stays on-device:
- No disclosure to third parties occurs
- The school maintains sole control of records
- No FERPA exception or consent is needed for data sharing (there is no sharing)

## Third-Party Data Sharing

**None.** Zero third parties receive any data from this application.

| Third Party | Data Shared | Purpose |
|------------|-------------|---------|
| (none) | (none) | (none) |

## Data Breach Risk

Risk is limited to physical device theft or compromise. In such an event:
- SQLite database is not encrypted (data is sticker scores, not high-sensitivity PII)
- Device-level encryption (iOS/Android full-disk encryption) provides the primary protection
- No network exfiltration path exists in the application
