# Accessibility Compliance — Section 508 / WCAG 2.1

This is free and unencumbered software released into the public domain.
See UNLICENSE file in repository root.

Project: wowasticker
Date: 2026-03-27
Standard: Section 508 of the Rehabilitation Act / WCAG 2.1

## Compliance Status: Partial

wowasticker provides some accessibility features but has known gaps. This document is an honest assessment of current state.

## Current Accessibility Features

### Touch Target Size (WCAG 2.5.5 — Level AAA)

- All interactive elements use 44pt minimum touch targets
- Thumb-zone layout: primary actions placed in bottom half of screen for one-handed use
- Sticker buttons are large circular targets (easily tappable)

### Color and Contrast (WCAG 1.4.3 — Level AA)

- High contrast text throughout the interface
- Sticker states use both color and symbol differentiation:
  - Empty: `○` (outline circle)
  - Half: `◐` (half-filled circle)
  - Full: `●` (filled circle)
- Score display uses numeric values alongside visual indicators

### Text Size (WCAG 1.4.4 — Level AA)

- UI text uses relative sizing
- System font scaling respected on mobile platforms

### Voice Input (alternative input method)

- Optional Whisper-based voice input (candle feature) provides an alternative to touch/keyboard
- Enables users with motor impairments to dictate observation notes

## Known Gaps

### No ARIA Labels (WCAG 4.1.2 — Level A) — GAP

- Dioxus WebView rendering does not currently emit ARIA attributes
- Interactive elements lack `role`, `aria-label`, and `aria-describedby` attributes
- Screen readers cannot identify UI controls

### No Screen Reader Support (WCAG 1.3.1 — Level A) — GAP

- Application is not tested with VoiceOver (iOS/macOS) or TalkBack (Android)
- WebView content may not expose accessibility tree to OS screen readers
- Sticker chart grid is not navigable via screen reader

### No Keyboard Navigation (WCAG 2.1.1 — Level A) — GAP

- Tab order is not defined
- No visible focus indicators
- No keyboard shortcuts
- Enter/Space activation of controls not tested

### Sticker Symbols Lack Text Alternatives (WCAG 1.1.1 — Level A) — GAP

- Unicode symbols `○`, `◐`, `●` are used for sticker states
- No `alt` text or `aria-label` describing their meaning ("empty", "half", "full")
- Screen readers may announce these as "circle" or skip them entirely

### No Skip Navigation (WCAG 2.4.1 — Level A) — GAP

- No skip-to-content links
- No landmark regions defined

### No Error Identification (WCAG 3.3.1 — Level A) — GAP

- Form validation errors are not programmatically associated with inputs
- No `aria-invalid` or `aria-errormessage` attributes

## VPAT Summary (Voluntary Product Accessibility Template)

| Criteria | Conformance Level | Notes |
|----------|-------------------|-------|
| 1.1.1 Non-text Content | Does Not Support | Sticker symbols lack text alternatives |
| 1.3.1 Info and Relationships | Does Not Support | No semantic structure exposed to AT |
| 1.4.3 Contrast | Supports | High contrast text used |
| 2.1.1 Keyboard | Does Not Support | No keyboard navigation |
| 2.4.1 Bypass Blocks | Does Not Support | No skip navigation |
| 2.5.5 Target Size | Supports | 44pt minimum touch targets |
| 3.3.1 Error Identification | Does Not Support | No programmatic error association |
| 4.1.2 Name, Role, Value | Does Not Support | No ARIA attributes |

## Remediation Plan

1. Add ARIA labels to all interactive Dioxus components
2. Define tab order and focus indicators
3. Add text alternatives to sticker symbols (`aria-label="full sticker"`)
4. Test with VoiceOver and TalkBack
5. Add landmark regions (`nav`, `main`, `footer`)
6. Add skip-to-content link

## Dioxus Accessibility Roadmap

Dioxus framework is actively developing accessibility support. As Dioxus adds native ARIA support and accessibility tree integration, wowasticker will adopt these features.
