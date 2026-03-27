// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f147=generate_daily_report. Plain-text daily sticker report for sharing with parents.

use crate::db::{ScheduleBlock, StickerRecord, StickerValue, Student};

fn sticker_label(v: StickerValue) -> &'static str {
    match v {
        StickerValue::Zero => "Needs work",
        StickerValue::One => "Good",
        StickerValue::Two => "Great",
    }
}

fn sticker_emoji(v: StickerValue) -> &'static str {
    match v {
        StickerValue::Zero => "○",
        StickerValue::One => "●",
        StickerValue::Two => "●●",
    }
}

/// f147=generate_daily_report. Format day records into shareable plain text.
pub fn generate_daily_report(
    student: &Student,
    date: &str,
    records: &[(ScheduleBlock, Option<StickerRecord>)],
    earned: i32,
) -> String {
    let mut lines = Vec::new();

    lines.push(format!("{}'s Sticker Report — {}", student.name, date));
    lines.push(format!(
        "Progress: {} / {} stickers",
        earned, student.goal_stickers
    ));
    if earned >= student.goal_stickers {
        lines.push("Goal met!".to_string());
    }
    lines.push(String::new());

    for (block, record) in records {
        match record {
            Some(rec) => {
                let label = sticker_label(rec.value);
                let emoji = sticker_emoji(rec.value);
                let mut line = format!("{} {} — {}", emoji, block.name, label);
                if let Some(ref note) = rec.note {
                    if !note.is_empty() {
                        line.push_str(&format!(": {}", note));
                    }
                }
                lines.push(line);
            }
            None => {
                lines.push(format!("○ {} — No observation", block.name));
            }
        }
    }

    lines.push(String::new());
    lines.push("— Sent from WowaSticker".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// f147=generate_daily_report formats correctly
    #[test]
    fn generate_daily_report_basic() {
        let student = Student {
            id: 1,
            name: "Luka".to_string(),
            goal_stickers: 10,
        };
        let blocks = vec![
            (
                ScheduleBlock {
                    id: 1,
                    name: "Math".to_string(),
                    sort_order: 0,
                },
                Some(StickerRecord {
                    block_id: 1,
                    date: "2026-03-27".to_string(),
                    value: StickerValue::Two,
                    note: Some("Great focus".to_string()),
                }),
            ),
            (
                ScheduleBlock {
                    id: 2,
                    name: "Recess".to_string(),
                    sort_order: 1,
                },
                Some(StickerRecord {
                    block_id: 2,
                    date: "2026-03-27".to_string(),
                    value: StickerValue::One,
                    note: None,
                }),
            ),
            (
                ScheduleBlock {
                    id: 3,
                    name: "Lunch".to_string(),
                    sort_order: 2,
                },
                None,
            ),
        ];
        let report = generate_daily_report(&student, "2026-03-27", &blocks, 3);
        assert!(report.contains("Luka's Sticker Report — 2026-03-27"));
        assert!(report.contains("3 / 10 stickers"));
        assert!(report.contains("●● Math — Great: Great focus"));
        assert!(report.contains("● Recess — Good"));
        assert!(report.contains("○ Lunch — No observation"));
        assert!(report.contains("Sent from WowaSticker"));
    }

    /// f147=generate_daily_report shows goal met
    #[test]
    fn generate_daily_report_goal_met() {
        let student = Student {
            id: 1,
            name: "Luka".to_string(),
            goal_stickers: 2,
        };
        let blocks = vec![(
            ScheduleBlock {
                id: 1,
                name: "Math".to_string(),
                sort_order: 0,
            },
            Some(StickerRecord {
                block_id: 1,
                date: "2026-03-27".to_string(),
                value: StickerValue::Two,
                note: None,
            }),
        )];
        let report = generate_daily_report(&student, "2026-03-27", &blocks, 2);
        assert!(report.contains("Goal met!"));
    }

    /// f147=generate_daily_report empty day
    #[test]
    fn generate_daily_report_empty_day() {
        let student = Student {
            id: 1,
            name: "Luka".to_string(),
            goal_stickers: 10,
        };
        let blocks = vec![(
            ScheduleBlock {
                id: 1,
                name: "Math".to_string(),
                sort_order: 0,
            },
            None,
        )];
        let report = generate_daily_report(&student, "2026-03-27", &blocks, 0);
        assert!(report.contains("0 / 10 stickers"));
        assert!(report.contains("○ Math — No observation"));
        assert!(!report.contains("Goal met!"));
    }
}
