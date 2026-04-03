// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f133=App. Dioxus UI: thumb-zone layout, schedule cards, dictation, daily report, history.

use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use wowasticker::{
    ai,
    db::{t119, t120, t121, t122, t123},
    report,
};

fn sticker_str(v: t119) -> &'static str {
    match v {
        t119::Zero => "○",
        t119::One => "●",
        t119::Two => "●●",
    }
}

fn today_str() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn format_date_display(date: &str) -> String {
    if date == today_str() {
        return "Today".to_string();
    }
    // Parse YYYY-MM-DD and format as "Mon, Mar 27"
    if let Ok(d) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        return d.format("%a, %b %d").to_string();
    }
    date.to_string()
}

fn shift_date(date: &str, days: i64) -> String {
    if let Ok(d) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        return (d + chrono::Duration::days(days))
            .format("%Y-%m-%d")
            .to_string();
    }
    date.to_string()
}

/// t125=t125. What was heard, scored, and tagged.
#[allow(non_camel_case_types)]
struct t125 {
    s13: String,
    s14: i64,
    s15: t119,
    s16: String,
    s17: Vec<String>,
}

/// f133=App. Root component: db, student, blocks, dictation flow, history, share.
#[component]
pub fn App() -> Element {
    let db_path =
        std::env::var("WOWASTICKER_DB").unwrap_or_else(|_| "wowasticker.db".to_string());
    let mut db = use_signal(|| None::<Arc<t123>>);
    let mut blocks = use_signal(|| Vec::<t120>::new());
    let mut selected_block = use_signal(|| 0usize);
    let mut status = use_signal(|| "Tap a block, then dictate.".to_string());
    let mut processing = use_signal(|| false);
    let mut last_error = use_signal(|| None::<String>);
    let mut refresh = use_signal(|| 0u32);
    let mut student = use_signal(|| t122 {
        s6: 0,
        s7: "Student".to_string(),
        s8: 15,
    });
    let mut stickers_earned = use_signal(|| 0i32);
    let mut view_date = use_signal(today_str);
    let mut last_dictation = use_signal(|| None::<(i64, String)>);
    let mut share_status = use_signal(|| None::<String>);

    let is_today = view_date() == today_str();

    // Load DB on mount
    use_effect(move || {
        let path = db_path.clone();
        spawn(async move {
            match t123::f121(&path) {
                Ok(d) => {
                    let d = Arc::new(d);
                    let _ = d.f140();
                    if let Ok(Some(s)) = d.f141() {
                        student.set(s);
                    }
                    if d.f123().is_ok() {
                        if let Ok(b) = d.f124() {
                            blocks.set(b);
                        }
                    }
                    if let Ok(earned) = d.f142() {
                        stickers_earned.set(earned);
                    }
                    db.set(Some(d));
                }
                Err(e) => {
                    status.set(format!("DB error: {}", e));
                }
            }
        });
    });

    let title = format!("{}'s Sticker Chart", student().s7);
    let goal_met = stickers_earned() >= student().s8;
    let progress_str = if goal_met {
        format!(
            "{} / {} Stickers — Goal met!",
            stickers_earned(),
            student().s8
        )
    } else {
        format!("{} / {} Stickers", stickers_earned(), student().s8)
    };
    let date_display = format_date_display(&view_date());
    let can_go_forward = view_date() < today_str();

    let goal_color = if goal_met { "#2e7d32" } else { "#666" };
    let fwd_border = if can_go_forward { "#ccc" } else { "#eee" };
    let fwd_color = if can_go_forward { "#333" } else { "#ccc" };
    let status_color = if last_error().is_some() {
        "#c62828"
    } else {
        "#666"
    };
    let status_text = if last_error().is_some() {
        last_error().unwrap_or_default()
    } else if share_status().is_some() {
        share_status().unwrap_or_default()
    } else if blocks.read().is_empty() && !processing() {
        "Loading schedule...".to_string()
    } else {
        status()
    };
    let dictate_bg = if last_error().is_some() {
        "#e65100"
    } else {
        "#007AFF"
    };
    let dictate_label = if last_error().is_some() {
        "Retry"
    } else {
        "Dictate Observation"
    };
    let fwd_style = format!("padding: 8px 16px; font-size: 1.1rem; background: none; border: 1px solid {fwd_border}; border-radius: 8px; cursor: pointer; color: {fwd_color};");
    let goal_style = format!("margin: 0 0 12px 0; font-size: 1rem; color: {goal_color};");
    let status_style = format!("margin: 0 0 10px 0; font-size: 0.85rem; color: {status_color};");
    let dictate_style = format!("width: 100%; padding: 18px; font-size: 1.1rem; background: {dictate_bg}; color: white; border-radius: 12px; border: none; cursor: pointer; margin-bottom: 8px;");
    let show_undo = is_today && last_dictation().is_some();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif;",
            padding_bottom: "env(safe-area-inset-bottom, 20px)",

            h1 { style: "margin: 0 0 4px 0; font-size: 1.5rem;", "{title}" }
            h3 { style: "{goal_style}", "{progress_str}" }

            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; padding: 8px 0;",
                button {
                    style: "padding: 8px 16px; font-size: 1.1rem; background: none; border: 1px solid #ccc; border-radius: 8px; cursor: pointer;",
                    onclick: move |_| {
                        let new_date = shift_date(&view_date(), -1);
                        view_date.set(new_date.clone());
                        refresh.set(refresh() + 1);
                        if let Some(ref d) = db() {
                            if let Ok(earned) = d.f145(&new_date) {
                                stickers_earned.set(earned);
                            }
                        }
                        share_status.set(None);
                    },
                    "<"
                }
                span { style: "font-weight: 600; font-size: 1rem;", "{date_display}" }
                button {
                    style: "{fwd_style}",
                    disabled: !can_go_forward,
                    onclick: move |_| {
                        let new_date = shift_date(&view_date(), 1);
                        view_date.set(new_date.clone());
                        refresh.set(refresh() + 1);
                        if let Some(ref d) = db() {
                            if let Ok(earned) = d.f145(&new_date) {
                                stickers_earned.set(earned);
                            }
                        }
                        share_status.set(None);
                    },
                    ">"
                }
            }

            div {
                style: "flex-grow: 1; overflow-y: auto; -webkit-overflow-scrolling: touch;",
                for (i, block) in blocks.read().iter().enumerate() {
                    ScheduleCard {
                        block: block.clone(),
                        is_selected: is_today && selected_block() == i,
                        is_today: is_today,
                        card_select: move |_| {
                            if is_today {
                                selected_block.set(i);
                            }
                        },
                        card_score: move |val: t119| {
                            let blk = blocks.read()[i].clone();
                            if let Some(ref d) = db() {
                                let _ = d.f135(blk.s0, val, None);
                                if let Ok(earned) = d.f142() {
                                    stickers_earned.set(earned);
                                }
                                refresh.set(refresh() + 1);
                                status.set(format!("{}: {} saved!", blk.s1, sticker_str(val)));
                            }
                        },
                        db: db(),
                        date: view_date(),
                        _refresh: refresh(),
                    }
                }
                if blocks.read().is_empty() {
                    div {
                        style: "padding: 40px 20px; text-align: center; color: #999;",
                        "Loading schedule..."
                    }
                }
            }

            div {
                style: "padding: 16px 0; border-top: 1px solid #ccc; flex-shrink: 0;",
                p { style: "{status_style}", "{status_text}" }

                if is_today {
                    button {
                        style: "{dictate_style}",
                        disabled: processing() || blocks.read().is_empty(),
                        onclick: move |_| {
                            last_error.set(None);
                            share_status.set(None);
                            processing.set(true);
                            let db_clone = db();
                            let sel = selected_block();
                            let blocks_clone = blocks.read().clone();
                            let mut status_sig = status;
                            let mut last_err_sig = last_error;
                            let mut refresh_sig = refresh;
                            let mut earned_sig = stickers_earned;
                            let mut undo_sig = last_dictation;
                            let date = view_date();
                            spawn(async move {
                                match f132(db_clone.clone(), sel, &blocks_clone, status_sig).await {
                                    Ok(Some(dr)) => {
                                        last_err_sig.set(None);
                                        undo_sig.set(Some((dr.s14, date)));
                                        let tag_str = if dr.s17.is_empty() {
                                            String::new()
                                        } else {
                                            format!(" [{}]", dr.s17.join(", "))
                                        };
                                        let heard = if dr.s16.is_empty() || dr.s16 == "Processed" {
                                            String::new()
                                        } else {
                                            format!(" — \"{}\"", dr.s16)
                                        };
                                        status_sig.set(format!(
                                            "{}: {} saved!{}{}",
                                            dr.s13, sticker_str(dr.s15), heard, tag_str
                                        ));
                                        if let Some(ref d) = db_clone {
                                            if let Ok(earned) = d.f142() {
                                                earned_sig.set(earned);
                                            }
                                        }
                                        refresh_sig.set(refresh_sig() + 1);
                                    }
                                    Ok(None) => {
                                        status_sig.set("Done.".to_string());
                                    }
                                    Err(e) => {
                                        let msg = format!("Error: {}", e);
                                        status_sig.set(msg.clone());
                                        last_err_sig.set(Some(msg));
                                    }
                                }
                                processing.set(false);
                            });
                        },
                        "{dictate_label}"
                    }
                }

                div {
                    style: "display: flex; gap: 8px;",
                    if show_undo {
                        button {
                            style: "flex: 1; padding: 14px; font-size: 0.95rem; background: #f5f5f5; color: #333; border-radius: 10px; border: 1px solid #ccc; cursor: pointer;",
                            disabled: processing(),
                            onclick: move |_| {
                                if let Some((block_id, date)) = last_dictation() {
                                    if let Some(ref d) = db() {
                                        if d.f146(block_id, &date).unwrap_or(false) {
                                            status.set("Undo: last observation removed.".to_string());
                                            last_dictation.set(None);
                                            if let Ok(earned) = d.f142() {
                                                stickers_earned.set(earned);
                                            }
                                            refresh.set(refresh() + 1);
                                        }
                                    }
                                }
                            },
                            "Undo"
                        }
                    }
                    button {
                        style: "flex: 1; padding: 14px; font-size: 0.95rem; background: #e8f5e9; color: #2e7d32; border-radius: 10px; border: 1px solid #a5d6a7; cursor: pointer;",
                        disabled: processing() || blocks.read().is_empty(),
                        onclick: move |_| {
                            if let Some(ref d) = db() {
                                let date = view_date();
                                if let Ok(records) = d.f144(&date) {
                                    let earned = d.f145(&date).unwrap_or(0);
                                    let text = report::f147(
                                        &student(),
                                        &date,
                                        &records,
                                        earned,
                                    );
                                    let js = format!(
                                        "navigator.clipboard.writeText({}).then(function(){{}}).catch(function(){{}})",
                                        serde_json::to_string(&text).unwrap_or_default()
                                    );
                                    eval(&js);
                                    share_status.set(Some("Daily report copied to clipboard!".to_string()));
                                }
                            }
                        },
                        "Share Daily Report"
                    }
                }
            }
        }
    }
}

/// f139=ScheduleCard. Block card with sticker display, note, tap-to-score.
#[component]
fn ScheduleCard(
    block: t120,
    is_selected: bool,
    is_today: bool,
    card_select: EventHandler<MouseEvent>,
    card_score: EventHandler<t119>,
    db: Option<Arc<t123>>,
    date: String,
    _refresh: u32,
) -> Element {
    let mut record = use_signal(|| None::<t121>);
    use_effect(move || {
        if let Some(ref d) = db {
            match d.f143(block.s0, &date) {
                Ok(r) => record.set(r),
                Err(_) => record.set(None),
            }
        }
    });

    let sticker_val = record().as_ref().map(|r| r.s5).unwrap_or(t119::Zero);
    let note_text = record()
        .as_ref()
        .and_then(|r| r.s9.clone())
        .unwrap_or_default();

    let bg = if is_selected { "#e3f2fd" } else { "#f0f0f0" };
    let border = if is_selected {
        "2px solid #007AFF"
    } else {
        "2px solid transparent"
    };
    let show_score_buttons = is_selected && is_today;
    let bg_zero = if sticker_val == t119::Zero { "#ffcdd2" } else { "#fff" };
    let bg_one = if sticker_val == t119::One { "#fff9c4" } else { "#fff" };
    let bg_two = if sticker_val == t119::Two { "#c8e6c9" } else { "#fff" };
    let btn_base = "flex: 1; padding: 8px; border-radius: 6px; border: 1px solid #ccc; cursor: pointer; font-size: 0.85rem;";
    let sticker_display = sticker_str(sticker_val);

    rsx! {
        div {
            style: "padding: 12px 15px; margin-bottom: 8px; border-radius: 8px; background: {bg}; border: {border};",
            onclick: move |e| card_select.call(e),
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                div { style: "font-weight: 600;", "{block.s1}" }
                div { style: "font-size: 1rem;", "{sticker_display}" }
            }
            if !note_text.is_empty() {
                div {
                    style: "font-size: 0.8rem; color: #555; margin-top: 4px; font-style: italic;",
                    "\"{note_text}\""
                }
            }
            if show_score_buttons {
                div {
                    style: "display: flex; gap: 8px; margin-top: 8px;",
                    button {
                        style: "{btn_base} background: {bg_zero};",
                        onclick: move |evt| { evt.stop_propagation(); card_score.call(t119::Zero); },
                        "0"
                    }
                    button {
                        style: "{btn_base} background: {bg_one};",
                        onclick: move |evt| { evt.stop_propagation(); card_score.call(t119::One); },
                        "1"
                    }
                    button {
                        style: "{btn_base} background: {bg_two};",
                        onclick: move |evt| { evt.stop_propagation(); card_score.call(t119::Two); },
                        "2"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sticker_str_all_values() {
        assert_eq!(sticker_str(t119::Zero), "○");
        assert_eq!(sticker_str(t119::One), "●");
        assert_eq!(sticker_str(t119::Two), "●●");
    }

    #[test]
    fn today_str_format() {
        let s = today_str();
        // YYYY-MM-DD
        assert_eq!(s.len(), 10);
        assert_eq!(s.as_bytes()[4], b'-');
        assert_eq!(s.as_bytes()[7], b'-');
    }

    #[test]
    fn format_date_display_today() {
        assert_eq!(format_date_display(&today_str()), "Today");
    }

    #[test]
    fn format_date_display_other_date() {
        let s = format_date_display("2026-03-27");
        // Should be "Fri, Mar 27" or similar
        assert!(s.contains("Mar"));
        assert!(s.contains("27"));
    }

    #[test]
    fn format_date_display_invalid_date() {
        assert_eq!(format_date_display("not-a-date"), "not-a-date");
    }

    #[test]
    fn shift_date_forward() {
        assert_eq!(shift_date("2026-03-27", 1), "2026-03-28");
    }

    #[test]
    fn shift_date_backward() {
        assert_eq!(shift_date("2026-03-01", -1), "2026-02-28");
    }

    #[test]
    fn shift_date_across_month_boundary() {
        assert_eq!(shift_date("2026-01-31", 1), "2026-02-01");
    }

    #[test]
    fn shift_date_across_year_boundary() {
        assert_eq!(shift_date("2025-12-31", 1), "2026-01-01");
    }

    #[test]
    fn shift_date_invalid() {
        assert_eq!(shift_date("bad", 1), "bad");
    }

    #[test]
    fn shift_date_zero() {
        assert_eq!(shift_date("2026-04-01", 0), "2026-04-01");
    }
}

/// f132=f132. capture_audio → transcribe → extract_behavior → set_sticker_today_with_note.
async fn f132(
    db: Option<Arc<t123>>,
    selected_idx: usize,
    blocks: &[t120],
    mut status: Signal<String>,
) -> anyhow::Result<Option<t125>> {
    status.set("Recording... 10s".to_string());
    let capture_handle = tokio::task::spawn_blocking(wowasticker::audio::f129);
    for i in (1..=10).rev() {
        status.set(format!("Recording... {}s", i));
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    let samples = capture_handle
        .await
        .map_err(|e| anyhow::anyhow!("spawn_blocking: {}", e))??;

    status.set("Transcribing...".to_string());
    let model_path = std::env::var("WOWASTICKER_WHISPER_PATH")
        .unwrap_or_else(|_| "whisper-tiny.gguf".to_string());
    let text = ai::f119(PathBuf::from(&model_path).as_path(), &samples).await?;

    status.set("Parsing sticker value...".to_string());
    let result = ai::f134(&text);

    if let (Some(d), Some(block)) = (db, blocks.get(selected_idx)) {
        let note = if result.s11.is_empty() {
            None
        } else {
            Some(result.s11.as_str())
        };
        d.f135(block.s0, result.s10, note)?;
        return Ok(Some(t125 {
            s13: block.s1.clone(),
            s14: block.s0,
            s15: result.s10,
            s16: text,
            s17: result.s12,
        }));
    }

    Ok(None)
}
