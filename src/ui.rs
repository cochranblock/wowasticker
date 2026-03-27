// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f133=App. Dioxus UI: thumb-zone layout, schedule cards, dictation button.

use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use wowasticker::{ai, db::Db, db::ScheduleBlock, db::StickerValue};

fn sticker_str(v: StickerValue) -> &'static str {
    match v {
        StickerValue::Zero => "○",
        StickerValue::One => "●",
        StickerValue::Two => "●●",
    }
}

/// t125=DictationResult. What was heard, scored, and tagged.
struct DictationResult {
    block_name: String,
    score: StickerValue,
    transcription: String,
    tags: Vec<String>,
}

/// f133=App. Root component: db, student, blocks, dictation flow.
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
    let student_name = use_signal(|| "Student".to_string());
    let goal_stickers = use_signal(|| 15i32);
    let stickers_earned = use_signal(|| 0i32);

    use_effect(move || {
        spawn(async move {
            match Db::open(&db_path) {
                Ok(d) => {
                    let d = Arc::new(d);
                    let _ = d.ensure_default_student();
                    if let Ok(Some(s)) = d.get_student() {
                        student_name.set(s.name);
                        goal_stickers.set(s.goal_stickers);
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

    let title = format!("{}'s Sticker Chart", student_name());
    let progress = format!("{} / {} Stickers", stickers_earned(), goal_stickers());
    let goal_met = stickers_earned() >= goal_stickers();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif;",
            padding_bottom: "env(safe-area-inset-bottom, 20px)",

            h1 { style: "margin: 0 0 4px 0; font-size: 1.5rem;", "{title}" }
            h3 {
                style: "margin: 0 0 16px 0; font-size: 1rem; color: {if goal_met { \"#2e7d32\" } else { \"#666\" }};",
                "{progress}"
                if goal_met { " — Goal met!" }
            }

            div {
                style: "flex-grow: 1; overflow-y: auto; -webkit-overflow-scrolling: touch;",
                for (i, block) in blocks.read().iter().enumerate() {
                    ScheduleCard {
                        block: block.clone(),
                        is_selected: selected_block() == i,
                        on_select: move |_| selected_block.set(i),
                        db: db(),
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
                style: "padding: 20px 0; border-top: 1px solid #ccc; flex-shrink: 0;",
                p {
                    style: "margin: 0 0 12px 0; font-size: 0.9rem; color: {if last_error().is_some() { \"#c62828\" } else { \"#666\" }};",
                    "{match () {
                        _ if last_error().is_some() => last_error().as_deref().unwrap_or(\"Error\"),
                        _ if blocks.read().is_empty() && !processing() => \"Loading schedule...\",
                        _ => status().as_str(),
                    }}"
                }
                button {
                    style: "width: 100%; padding: 20px; font-size: 1.2rem; background: {if last_error().is_some() { \"#e65100\" } else { \"#007AFF\" }}; color: white; border-radius: 12px; border: none; cursor: pointer;",
                    disabled: processing() || blocks.read().is_empty(),
                    onclick: move |_| {
                        last_error.set(None);
                        processing.set(true);
                        let db_clone = db();
                        let sel = selected_block();
                        let blocks_clone = blocks.read().clone();
                        let status_sig = status;
                        let last_err_sig = last_error;
                        let refresh_sig = refresh;
                        let earned_sig = stickers_earned;
                        spawn(async move {
                            match run_dictation_flow(db_clone.clone(), sel, &blocks_clone, status_sig).await {
                                Ok(Some(dr)) => {
                                    last_err_sig.set(None);
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
        }
    }
}

/// f139=ScheduleCard. Block card with sticker display, selection.
#[component]
fn ScheduleCard(
    block: ScheduleBlock,
    is_selected: bool,
    on_select: EventHandler<MouseEvent>,
    db: Option<Arc<Db>>,
    _refresh: u32,
) -> Element {
    let sticker = use_signal(|| StickerValue::Zero);
    use_effect(move || {
        if let Some(ref d) = db {
            if let Ok(v) = d.get_sticker_today(block.id) {
                sticker.set(v);
            }
        }
    });

    let bg = if is_selected { "#e3f2fd" } else { "#f0f0f0" };
    let border = if is_selected { "2px solid #007AFF" } else { "2px solid transparent" };

    rsx! {
        div {
            style: "padding: 15px; margin-bottom: 10px; border-radius: 8px; background: {bg}; border: {border};",
            onclick: move |e| on_select.call(e),
            div { style: "font-weight: 600;", "{block.name}" }
            div { style: "font-size: 0.9rem; color: #666; margin-top: 4px;",
                "Stickers: "
                match sticker() {
                    StickerValue::Zero => "○",
                    StickerValue::One => "●",
                    StickerValue::Two => "●●",
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
            score: result.score,
            transcription: text,
            tags: result.tags,
        }));
    }

    Ok(None)
}
