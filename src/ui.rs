// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f133=App. Dioxus UI: thumb-zone layout, schedule cards, dictation, daily report, history.

use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use wowasticker::{
    ai,
    db::{Db, ScheduleBlock, StickerRecord, StickerValue, Student},
    report,
};

fn sticker_str(v: StickerValue) -> &'static str {
    match v {
        StickerValue::Zero => "○",
        StickerValue::One => "●",
        StickerValue::Two => "●●",
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

/// t125=DictationResult. What was heard, scored, and tagged.
struct DictationResult {
    block_name: String,
    block_id: i64,
    score: StickerValue,
    transcription: String,
    tags: Vec<String>,
}

/// f133=App. Root component: db, student, blocks, dictation flow, history, share.
#[component]
pub fn App() -> Element {
    let db_path =
        std::env::var("WOWASTICKER_DB").unwrap_or_else(|_| "wowasticker.db".to_string());
    let db = use_signal(|| None::<Arc<Db>>);
    let blocks = use_signal(|| Vec::<ScheduleBlock>::new());
    let selected_block = use_signal(|| 0usize);
    let status = use_signal(|| "Tap a block, then dictate.".to_string());
    let processing = use_signal(|| false);
    let last_error = use_signal(|| None::<String>);
    let refresh = use_signal(|| 0u32);
    let student = use_signal(|| Student {
        id: 0,
        name: "Student".to_string(),
        goal_stickers: 15,
    });
    let stickers_earned = use_signal(|| 0i32);
    let view_date = use_signal(today_str);
    let last_dictation = use_signal(|| None::<(i64, String)>); // (block_id, date) for undo
    let share_status = use_signal(|| None::<String>);

    let is_today = view_date() == today_str();

    // Load DB on mount
    use_effect(move || {
        spawn(async move {
            match Db::open(&db_path) {
                Ok(d) => {
                    let d = Arc::new(d);
                    let _ = d.ensure_default_student();
                    if let Ok(Some(s)) = d.get_student() {
                        student.set(s);
                    }
                    if d.ensure_default_schedule().is_ok() {
                        if let Ok(b) = d.list_blocks() {
                            blocks.set(b);
                        }
                    }
                    if let Ok(earned) = d.count_stickers_today() {
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

    let title = format!("{}'s Sticker Chart", student().name);
    let progress = format!("{} / {} Stickers", stickers_earned(), student().goal_stickers);
    let goal_met = stickers_earned() >= student().goal_stickers;
    let date_display = format_date_display(&view_date());
    let can_go_forward = view_date() < today_str();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif;",
            padding_bottom: "env(safe-area-inset-bottom, 20px)",

            // Header
            h1 { style: "margin: 0 0 4px 0; font-size: 1.5rem;", "{title}" }
            h3 {
                style: "margin: 0 0 12px 0; font-size: 1rem; color: {if goal_met { \"#2e7d32\" } else { \"#666\" }};",
                "{progress}"
                if goal_met { " — Goal met!" }
            }

            // Date navigation
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; padding: 8px 0;",
                button {
                    style: "padding: 8px 16px; font-size: 1.1rem; background: none; border: 1px solid #ccc; border-radius: 8px; cursor: pointer;",
                    onclick: move |_| {
                        let new_date = shift_date(&view_date(), -1);
                        view_date.set(new_date.clone());
                        refresh.set(refresh() + 1);
                        if let Some(ref d) = db() {
                            if let Ok(earned) = d.count_stickers_for_date(&new_date) {
                                stickers_earned.set(earned);
                            }
                        }
                        share_status.set(None);
                    },
                    "<"
                }
                span { style: "font-weight: 600; font-size: 1rem;", "{date_display}" }
                button {
                    style: "padding: 8px 16px; font-size: 1.1rem; background: none; border: 1px solid {if can_go_forward { \"#ccc\" } else { \"#eee\" }}; border-radius: 8px; cursor: pointer; color: {if can_go_forward { \"#333\" } else { \"#ccc\" }};",
                    disabled: !can_go_forward,
                    onclick: move |_| {
                        let new_date = shift_date(&view_date(), 1);
                        view_date.set(new_date.clone());
                        refresh.set(refresh() + 1);
                        if let Some(ref d) = db() {
                            if let Ok(earned) = d.count_stickers_for_date(&new_date) {
                                stickers_earned.set(earned);
                            }
                        }
                        share_status.set(None);
                    },
                    ">"
                }
            }

            // Block list
            div {
                style: "flex-grow: 1; overflow-y: auto; -webkit-overflow-scrolling: touch;",
                for (i, block) in blocks.read().iter().enumerate() {
                    ScheduleCard {
                        block: block.clone(),
                        is_selected: is_today && selected_block() == i,
                        on_select: move |_| {
                            if is_today {
                                selected_block.set(i);
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

            // Bottom action area
            div {
                style: "padding: 16px 0; border-top: 1px solid #ccc; flex-shrink: 0;",

                // Status line
                p {
                    style: "margin: 0 0 10px 0; font-size: 0.85rem; color: {if last_error().is_some() { \"#c62828\" } else { \"#666\" }};",
                    "{match () {
                        _ if last_error().is_some() => last_error().as_deref().unwrap_or(\"Error\"),
                        _ if share_status().is_some() => share_status().as_deref().unwrap_or(\"\"),
                        _ if blocks.read().is_empty() && !processing() => \"Loading schedule...\",
                        _ => status().as_str(),
                    }}"
                }

                // Dictate button (only on today)
                if is_today {
                    button {
                        style: "width: 100%; padding: 18px; font-size: 1.1rem; background: {if last_error().is_some() { \"#e65100\" } else { \"#007AFF\" }}; color: white; border-radius: 12px; border: none; cursor: pointer; margin-bottom: 8px;",
                        disabled: processing() || blocks.read().is_empty(),
                        onclick: move |_| {
                            last_error.set(None);
                            share_status.set(None);
                            processing.set(true);
                            let db_clone = db();
                            let sel = selected_block();
                            let blocks_clone = blocks.read().clone();
                            let status_sig = status;
                            let last_err_sig = last_error;
                            let refresh_sig = refresh;
                            let earned_sig = stickers_earned;
                            let undo_sig = last_dictation;
                            let date = view_date();
                            spawn(async move {
                                match run_dictation_flow(db_clone.clone(), sel, &blocks_clone, status_sig).await {
                                    Ok(Some(dr)) => {
                                        last_err_sig.set(None);
                                        undo_sig.set(Some((dr.block_id, date)));
                                        let tag_str = if dr.tags.is_empty() {
                                            String::new()
                                        } else {
                                            format!(" [{}]", dr.tags.join(", "))
                                        };
                                        let heard = if dr.transcription.is_empty() || dr.transcription == "Processed" {
                                            String::new()
                                        } else {
                                            format!(" — \"{}\"", dr.transcription)
                                        };
                                        status_sig.set(format!(
                                            "{}: {} saved!{}{}",
                                            dr.block_name, sticker_str(dr.score), heard, tag_str
                                        ));
                                        if let Some(ref d) = db_clone {
                                            if let Ok(earned) = d.count_stickers_today() {
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
                        "{if last_error().is_some() { \"🔄 Retry\" } else { \"🎤 Dictate Observation\" }}"
                    }
                }

                // Bottom row: Undo + Share
                div {
                    style: "display: flex; gap: 8px;",

                    // Undo button (only when there's something to undo, and on today)
                    if is_today && last_dictation().is_some() {
                        button {
                            style: "flex: 1; padding: 14px; font-size: 0.95rem; background: #f5f5f5; color: #333; border-radius: 10px; border: 1px solid #ccc; cursor: pointer;",
                            disabled: processing(),
                            onclick: move |_| {
                                if let Some((block_id, date)) = last_dictation() {
                                    if let Some(ref d) = db() {
                                        if d.delete_sticker(block_id, &date).unwrap_or(false) {
                                            status.set("Undo: last observation removed.".to_string());
                                            last_dictation.set(None);
                                            if let Ok(earned) = d.count_stickers_today() {
                                                stickers_earned.set(earned);
                                            }
                                            refresh.set(refresh() + 1);
                                        }
                                    }
                                }
                            },
                            "↩ Undo"
                        }
                    }

                    // Share daily report
                    button {
                        style: "flex: 1; padding: 14px; font-size: 0.95rem; background: #e8f5e9; color: #2e7d32; border-radius: 10px; border: 1px solid #a5d6a7; cursor: pointer;",
                        disabled: processing() || blocks.read().is_empty(),
                        onclick: move |_| {
                            if let Some(ref d) = db() {
                                let date = view_date();
                                if let Ok(records) = d.list_day_records(&date) {
                                    let earned = d.count_stickers_for_date(&date).unwrap_or(0);
                                    let text = report::generate_daily_report(
                                        &student(),
                                        &date,
                                        &records,
                                        earned,
                                    );
                                    // Copy to clipboard via eval (WebView)
                                    let js = format!(
                                        "navigator.clipboard.writeText({}).then(() => {{}}).catch(() => {{}})",
                                        serde_json::to_string(&text).unwrap_or_default()
                                    );
                                    eval(&js);
                                    share_status.set(Some("Daily report copied to clipboard!".to_string()));
                                }
                            }
                        },
                        "📋 Share Daily Report"
                    }
                }
            }
        }
    }
}

/// f139=ScheduleCard. Block card with sticker display, note, selection.
#[component]
fn ScheduleCard(
    block: ScheduleBlock,
    is_selected: bool,
    on_select: EventHandler<MouseEvent>,
    db: Option<Arc<Db>>,
    date: String,
    _refresh: u32,
) -> Element {
    let record = use_signal(|| None::<StickerRecord>);
    use_effect(move || {
        if let Some(ref d) = db {
            match d.get_sticker_record(block.id, &date) {
                Ok(r) => record.set(r),
                Err(_) => record.set(None),
            }
        }
    });

    let sticker_val = record().as_ref().map(|r| r.value).unwrap_or(StickerValue::Zero);
    let note_text = record().as_ref().and_then(|r| r.note.clone()).unwrap_or_default();

    let bg = if is_selected { "#e3f2fd" } else { "#f0f0f0" };
    let border = if is_selected {
        "2px solid #007AFF"
    } else {
        "2px solid transparent"
    };

    rsx! {
        div {
            style: "padding: 12px 15px; margin-bottom: 8px; border-radius: 8px; background: {bg}; border: {border};",
            onclick: move |e| on_select.call(e),
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                div { style: "font-weight: 600;", "{block.name}" }
                div { style: "font-size: 1rem;",
                    match sticker_val {
                        StickerValue::Zero => "○",
                        StickerValue::One => "●",
                        StickerValue::Two => "●●",
                    }
                }
            }
            if !note_text.is_empty() {
                div {
                    style: "font-size: 0.8rem; color: #555; margin-top: 4px; font-style: italic;",
                    "\"{note_text}\""
                }
            }
        }
    }
}

/// f132=run_dictation_flow. capture_audio → transcribe → extract_behavior → set_sticker_today_with_note.
async fn run_dictation_flow(
    db: Option<Arc<Db>>,
    selected_idx: usize,
    blocks: &[ScheduleBlock],
    status: Signal<String>,
) -> anyhow::Result<Option<DictationResult>> {
    status.set("Recording... 10s".to_string());
    let (tx, rx) = std::sync::mpsc::channel();
    let countdown_handle = std::thread::spawn(move || {
        for i in (1..=10).rev() {
            status.set(format!("Recording... {}s", i));
            std::thread::sleep(Duration::from_secs(1));
        }
        let _ = tx.send(());
    });
    let samples = tokio::task::spawn_blocking(wowasticker::audio::capture_audio)
        .await
        .map_err(|e| anyhow::anyhow!("spawn_blocking: {}", e))??;
    let _ = rx.recv();
    countdown_handle.join().unwrap();

    status.set("Transcribing...".to_string());
    let model_path = std::env::var("WOWASTICKER_WHISPER_PATH")
        .unwrap_or_else(|_| "whisper-tiny.gguf".to_string());
    let text = ai::transcribe_audio(PathBuf::from(&model_path).as_path(), &samples).await?;

    status.set("Parsing sticker value...".to_string());
    let result = ai::extract_behavior(&text);

    if let (Some(d), Some(block)) = (db, blocks.get(selected_idx)) {
        let note = if result.note.is_empty() {
            None
        } else {
            Some(result.note.as_str())
        };
        d.set_sticker_today_with_note(block.id, result.score, note)?;
        return Ok(Some(DictationResult {
            block_name: block.name.clone(),
            block_id: block.id,
            score: result.score,
            transcription: text,
            tags: result.tags,
        }));
    }

    Ok(None)
}
