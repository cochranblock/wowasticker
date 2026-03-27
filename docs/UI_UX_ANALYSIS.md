<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->
# Wowasticker UI/UX Analysis

## Overview

Luka's Sticker Chart is a mobile-first behavioral tracking app: parents/educators dictate observations, AI parses sentiment, and stickers are assigned to schedule blocks. Target: one-handed use during school pickup, in-the-moment capture.

---

## Strengths

### 1. **Thumb-zone layout**
- Primary action (Dictate) is bottom-anchored with `flex-shrink: 0` — reachable without stretching
- `padding_bottom: env(safe-area-inset-bottom, 20px)` respects notches/home indicators
- Scrollable block list above keeps content in reach

### 2. **Clear hierarchy**
- Title → Goal → Blocks → Action. Linear, predictable
- Schedule cards use weight (600) and color (#666) to separate label from value
- Sticker states (○ ● ●●) are simple and scannable

### 3. **Selection feedback**
- Selected card: `#e3f2fd` background, `2px solid #007AFF` border — visible but not loud
- Transparent border when unselected avoids layout shift

### 4. **Status communication**
- Status text updates through flow: "Recording...", "Transcribing...", "Parsing...", "Done" / "Error"
- User knows where they are in a 10s+ async flow

### 5. **Disabled state during processing**
- Button `disabled: processing()` prevents double-tap and accidental re-recording

---

## Gaps & Risks

### 1. ~~No loading / progress for 10s recording~~ **RESOLVED**
- **Implemented:** Countdown thread updates status "Recording... 10s" → "Recording... 1s" during capture
- **Remaining:** Consider haptic feedback at start/end

### 2. ~~Selection affordance is weak~~ **PARTIALLY RESOLVED**
- **Implemented:** Default status text reads "Tap a block, then dictate."
- **Remaining:** Consider visual tutorial or animation for first-time users

### 3. ~~Goal is static~~ **RESOLVED**
- **Implemented:** Goal pulled from `Student.goal_stickers` via `get_student()`. Progress counter shows "4 / 15 Stickers" with green "Goal met!" state

### 4. ~~Error recovery is minimal~~ **RESOLVED**
- **Implemented:** Button changes to "Retry" (orange) on error. `last_error` signal drives visual state
- **Remaining:** Surface common causes (e.g. mic permission denied)

### 5. ~~Empty state is placeholder-only~~ **RESOLVED**
- **Implemented:** Replaced fake gray blocks with centered "Loading schedule..." text. Dictate button disabled until blocks load

### 6. ~~No confirmation of what was saved~~ **RESOLVED**
- **Implemented:** Status shows "Math: ●● — saved!" with block name + sticker score after save

### 7. **Accessibility**
- No `aria-label` or `role` on button/cards
- Sticker symbols (○ ● ●●) may need `aria-label` or text alternative for screen readers
- **Suggestion:** Add `aria-label="Dictate observation for selected block"` and semantic labels for sticker states

### 8. **Touch target size**
- Button padding (20px) is good
- Card tap area is full card — good
- Ensure minimum 44×44pt touch targets for all interactive elements

### 9. **No undo**
- Mis-dictation or wrong block → no way to revert
- **Suggestion:** Undo snackbar after "Done" or edit flow for last entry

### 10. ~~Transcription opacity~~ **RESOLVED**
- **Implemented:** After save, status shows transcription text and behavior tags: `Math: ●● saved! — "He did great today!" [positive]`

---

## Summary Table

| Area           | Status | Priority |
|----------------|--------|----------|
| Thumb-zone     | ✅     | —        |
| Selection UX   | ✅     | Done     |
| Recording feedback | ✅  | Done     |
| Error recovery| ✅     | Done     |
| Goal display   | ✅     | Done     |
| Empty/loading  | ✅     | Done     |
| Confirmation   | ✅     | Done     |
| Accessibility  | ❌     | High     |
| Undo           | ❌     | Medium   |

---

## Quick Wins

1. ~~Recording countdown~~ **Done** — countdown thread updates status each second
2. ~~Retry on error~~ **Done** — button turns orange "Retry" on error
3. ~~Richer "Done" message~~ **Done** — "Math: ●● — saved!"
4. ~~First-tap hint~~ **Done** — default status: "Tap a block, then dictate."
5. **Accessibility** — add aria-labels and semantic roles
6. **Undo** — snackbar or edit flow for last entry
