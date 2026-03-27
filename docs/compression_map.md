<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->
# Wowasticker Compression Map

P13 tokenization for traceability. All public symbols use compressed identifiers.

## Functions (fN)

| Token | Human name | Module |
|-------|------------|--------|
| f119 | transcribe_audio | ai |
| f120 | parse_sticker_from_transcription | ai |
| f121 | open (Db) | db |
| f122 | init (Db) | db |
| f123 | ensure_default_schedule | db |
| f124 | list_blocks | db |
| f125 | get_sticker | db |
| f126 | set_sticker | db |
| f127 | get_sticker_today | db |
| f128 | set_sticker_today | db |
| f129 | capture_audio | audio |
| f130 | resample_to_16k | audio |
| f131 | wowasticker_test | bin/wowasticker-test |
| f132 | run_dictation_flow | ui |
| f133 | App | ui |
| f134 | extract_behavior | ai |
| f135 | set_sticker_today_with_note | db |
| f136 | set_sticker_with_note | db |
| f137 | transcribe_audio_sync | ai (candle) |
| f138 | extract_tags | ai |
| f139 | ScheduleCard | ui |
| f140 | ensure_default_student | db |
| f141 | get_student | db |
| f142 | count_stickers_today | db |
| f143 | get_sticker_record | db |
| f144 | list_day_records | db |
| f145 | count_stickers_for_date | db |
| f146 | delete_sticker | db |
| f147 | generate_daily_report | report |

## Types (tN)

| Token | Human name |
|-------|------------|
| t119 | StickerValue |
| t120 | ScheduleBlock |
| t121 | StickerRecord |
| t122 | Student |
| t123 | Db |
| t124 | BehaviorResult |
| t125 | DictationResult |

## Struct fields (sN)

| Token | Type | Field |
|-------|------|-------|
| s0 | t120 | id |
| s1 | t120 | name |
| s2 | t120 | sort_order |
| s3 | t121 | block_id |
| s4 | t121 | date |
| s5 | t121 | value |
| s6 | t122 | id |
| s7 | t122 | name |
| s8 | t122 | goal_stickers |
| s9 | t121 | note |
| s10 | t124 | score |
| s11 | t124 | note |
| s12 | t124 | tags |
| s13 | t125 | block_name |
| s14 | t125 | block_id |
| s15 | t125 | score |
| s16 | t125 | transcription |
| s17 | t125 | tags |

## Test traceability

- `/// f131=wowasticker_test` — bin/wowasticker-test.rs (TRIPLE SIMS via exopack f61_with_args)
- Run `rg '/// [ft][0-9]+=' wowasticker/src` to list coverage.
