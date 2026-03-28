# Federal Use Cases

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27

## Overview

wowasticker is an offline-first behavioral goal tracking app built in pure Rust. Its on-device architecture, zero-network design, and public domain license make it suitable for federal environments where connectivity is limited, data sovereignty is required, or procurement simplicity is valued.

## 1. DoD — Department of Defense Education Activity (DoDEA)

**Agency:** Department of Defense, DoDEA
**Schools:** 160+ schools worldwide serving military dependents
**Students:** Approximately 66,000 students across 8 districts in 11 countries

### Use Case

Special education teachers in DoDEA schools track behavioral goals as part of Individualized Education Programs (IEPs). Current tools often require cloud connectivity, which is unreliable or restricted on OCONUS military installations.

### Why wowasticker Fits

- **Offline-first:** OCONUS bases (Germany, Japan, Guam, Bahrain) frequently have limited or restricted internet access. wowasticker requires zero connectivity.
- **On-device data:** Student IEP behavioral data stays on the teacher's device. No data traverses DoD networks or leaves the installation.
- **No ATO burden for cloud:** No server infrastructure means no cloud ATO process. Device-level approval only.
- **FIPS N/A:** No crypto means no FIPS validation requirement.
- **Public domain:** Unlicense eliminates procurement licensing concerns. No per-seat fees. No vendor lock-in.

### Deployment Model

Install on DoDEA-issued teacher tablets (iPad/Android). Data stays on device. No MDM integration needed beyond standard app deployment.

## 2. VA — Department of Veterans Affairs

**Agency:** Veterans Health Administration (VHA), Vocational Rehabilitation and Employment (VR&E)

### Use Case

VA vocational rehabilitation programs track behavioral and occupational goals for veterans transitioning to civilian employment. Counselors work with veterans on goal-setting and accountability, often in VA facilities with inconsistent WiFi.

### Why wowasticker Fits

- **Offline operation:** VA medical centers and regional offices have notoriously unreliable guest/clinical WiFi. Staff WiFi may restrict non-VA applications.
- **Simple data model:** Behavioral goals map directly to wowasticker's sticker chart: daily blocks, scored 0/1/2, with observation notes.
- **No VA network integration needed:** Data stays on the counselor's device. No HL7/FHIR integration required. No VistA dependency.
- **Low training burden:** Sticker chart metaphor is immediately understandable.

### Deployment Model

Install on VA-issued counselor tablets. Export data manually (SQLite file copy) for inclusion in veteran case files as needed.

## 3. HHS — Head Start (Administration for Children and Families)

**Agency:** Department of Health and Human Services, Administration for Children and Families (ACF)
**Program:** Head Start / Early Head Start
**Sites:** 1,600+ grantees operating 31,000+ classrooms

### Use Case

Head Start programs serve children ages 0-5 from low-income families. Teachers track developmental and behavioral milestones. Programs must comply with COPPA when using digital tools.

### Why wowasticker Fits

- **COPPA-compliant by architecture:** No data leaves the device. No "operator" collects children's data. No consent complexity.
- **No cloud dependency:** Many Head Start centers are in under-resourced communities with limited internet.
- **Free:** Public domain license. No grant funding needed for software licenses.
- **Age-appropriate data model:** Simple sticker scores (empty/half/full) map to early childhood behavioral observation scales.

### Deployment Model

Install on classroom tablets. Each teacher tracks their students locally. Data shared with program administrators via SQLite export if needed for reporting.

## 4. BIE — Bureau of Indian Education

**Agency:** Department of the Interior, Bureau of Indian Education
**Schools:** 183 schools on 64 reservations in 23 states
**Students:** Approximately 46,000 students

### Use Case

BIE-operated and tribally controlled schools on reservations provide special education services including behavioral IEP tracking. Many reservation schools have limited or no broadband internet.

### Why wowasticker Fits

- **Offline-first is a requirement, not a feature:** Reservation broadband coverage is well below national averages. Cloud-based tools are non-starters for many BIE schools.
- **On-device data sovereignty:** Student behavioral data stays on tribal/school-controlled devices. No data transmitted to off-reservation servers.
- **IEP behavioral tracking:** Special education teachers track daily behavioral goals — the exact use case for wowasticker's sticker chart model.
- **No recurring costs:** Public domain. No subscription. No vendor relationship to maintain.

### Deployment Model

Install on school-issued teacher devices. Works immediately without internet configuration. Data remains under school/tribal control.

## 5. DOJ — Federal Bureau of Prisons (BOP)

**Agency:** Department of Justice, Federal Bureau of Prisons
**Facilities:** 122 federal prisons
**Inmates in education programs:** Approximately 58,000

### Use Case

Federal prisons operate education programs (literacy, GED, vocational) where instructors track student behavioral goals and program compliance. The First Step Act (2018) requires earned time credits tied to evidence-based programming, which includes behavioral tracking.

### Why wowasticker Fits

- **Air-gapped environments:** Federal prisons are air-gapped by policy. Cloud tools are prohibited. wowasticker's zero-network architecture is a match.
- **No internet required:** Not just offline-first — prisons have no inmate internet access. Teacher devices may have restricted or no network access in classroom areas.
- **Behavioral tracking for programming credits:** First Step Act requires documentation of participation and behavioral compliance. Sticker chart model maps to daily behavioral observation.
- **No security risk:** No network calls means no data exfiltration vector. No crypto means no key management burden. SQLite on a managed device.

### Deployment Model

Install on BOP-issued instructor tablets. Devices managed by facility IT. Data exported via USB or direct file transfer for inclusion in inmate education records.

## Cross-Cutting Benefits for Federal Agencies

| Benefit | Detail |
|---------|--------|
| Zero procurement complexity | Unlicense (public domain) — no license agreement, no per-seat fees, no vendor negotiation |
| Zero infrastructure | No servers, no cloud accounts, no network configuration |
| Zero ongoing cost | No subscription, no maintenance contract, no SaaS renewal |
| Data sovereignty | All data on agency-controlled devices. No third-party data processing. |
| SBOM available | Full software bill of materials in govdocs/SBOM.md |
| Source available | Build from source, audit the code, modify as needed |
| Rust memory safety | Eliminates buffer overflow, use-after-free, and null pointer CVE classes |
