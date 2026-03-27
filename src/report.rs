// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f147=generate_daily_report. Plain-text daily sticker report for sharing with parents.

use crate::db::{t119, t120, t121, t122};

fn sticker_label(v: t119) -> &'static str {
    match v {
        t119::Zero => "Needs work",
        t119::One => "Good",
        t119::Two => "Great",
    }
}

fn sticker_emoji(v: t119) -> &'static str {
    match v {
        t119::Zero => "○",
        t119::One => "●",
        t119::Two => "●●",
    }
}

/// f147=generate_daily_report. Format day records into shareable plain text.
pub fn f147(
    student: &t122,
    date: &str,
    records: &[(t120, Option<t121>)],
    earned: i32,
) -> String {
    let mut lines = Vec::new();

    lines.push(format!("{}'s Sticker Report — {}", student.s7, date));
    lines.push(format!(
        "Progress: {} / {} stickers",
        earned, student.s8
    ));
    if earned >= student.s8 {
        lines.push("Goal met!".to_string());
    }
    lines.push(String::new());

    for (block, record) in records {
        match record {
            Some(rec) => {
                let label = sticker_label(rec.s5);
                let emoji = sticker_emoji(rec.s5);
                let mut line = format!("{} {} — {}", emoji, block.s1, label);
                if let Some(ref note) = rec.s9 {
                    if !note.is_empty() {
                        line.push_str(&format!(": {}", note));
                    }
                }
                lines.push(line);
            }
            None => {
                lines.push(format!("○ {} — No observation", block.s1));
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
        let student = t122 {
            s6: 1,
            s7: "Luka".to_string(),
            s8: 10,
        };
        let blocks = vec![
            (
                t120 {
                    s0: 1,
                    s1: "Math".to_string(),
                    s2: 0,
                },
                Some(t121 {
                    s3: 1,
                    s4: "2026-03-27".to_string(),
                    s5: t119::Two,
                    s9: Some("Great focus".to_string()),
                }),
            ),
            (
                t120 {
                    s0: 2,
                    s1: "Recess".to_string(),
                    s2: 1,
                },
                Some(t121 {
                    s3: 2,
                    s4: "2026-03-27".to_string(),
                    s5: t119::One,
                    s9: None,
                }),
            ),
            (
                t120 {
                    s0: 3,
                    s1: "Lunch".to_string(),
                    s2: 2,
                },
                None,
            ),
        ];
        let report = f147(&student, "2026-03-27", &blocks, 3);
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
        let student = t122 {
            s6: 1,
            s7: "Luka".to_string(),
            s8: 2,
        };
        let blocks = vec![(
            t120 {
                s0: 1,
                s1: "Math".to_string(),
                s2: 0,
            },
            Some(t121 {
                s3: 1,
                s4: "2026-03-27".to_string(),
                s5: t119::Two,
                s9: None,
            }),
        )];
        let report = f147(&student, "2026-03-27", &blocks, 2);
        assert!(report.contains("Goal met!"));
    }

    /// f147=generate_daily_report empty day
    #[test]
    fn generate_daily_report_empty_day() {
        let student = t122 {
            s6: 1,
            s7: "Luka".to_string(),
            s8: 10,
        };
        let blocks = vec![(
            t120 {
                s0: 1,
                s1: "Math".to_string(),
                s2: 0,
            },
            None,
        )];
        let report = f147(&student, "2026-03-27", &blocks, 0);
        assert!(report.contains("0 / 10 stickers"));
        assert!(report.contains("○ Math — No observation"));
        assert!(!report.contains("Goal met!"));
    }
}
