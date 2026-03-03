//! f133=App. Dioxus UI: thumb-zone layout, schedule cards, dictation button.

use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use wowasticker::{ai, db::Db, db::ScheduleBlock, db::StickerValue};

/// Default schedule blocks (used before DB init)
const DEFAULT_BLOCKS: &[&str] = &[
    "Cultural Arts",
    "Community Circle",
    "Math",
    "Recess",
    "Lunch",
];

/// f133=App. Root component: db, blocks, dictation flow.
#[component]
pub fn App() -> Element {
    let db_path = std::env::var("WOWASTICKER_DB")
        .unwrap_or_else(|_| "wowasticker.db".to_string());
    let db = use_signal(|| None::<Arc<Db>>);
    let blocks = use_signal(|| Vec::<ScheduleBlock>::new());
    let selected_block = use_signal(|| 0usize);
    let status = use_signal(|| "Ready to record observation...".to_string());
    let processing = use_signal(|| false);
    let refresh = use_signal(|| 0u32);

    use_effect(move || {
        spawn(async move {
            match Db::open(&db_path) {
                Ok(d) => {
                    let d = Arc::new(d);
                    if d.ensure_default_schedule().is_ok() {
                        if let Ok(b) = d.list_blocks() {
                            blocks.set(b);
                        }
                    }
                    db.set(Some(d));
                }
                Err(e) => {
                    status.set(format!("DB error: {}", e));
                }
            }
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif;",
            padding_bottom: "env(safe-area-inset-bottom, 20px)",

            h1 { style: "margin: 0 0 4px 0; font-size: 1.5rem;", "Luka's Sticker Chart" }
            h3 { style: "margin: 0 0 16px 0; font-size: 1rem; color: #666;", "Goal: 15 Stickers" }

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
                    for (i, name) in DEFAULT_BLOCKS.iter().enumerate() {
                        div {
                            style: "padding: 15px; margin-bottom: 10px; border-radius: 8px; background: #f0f0f0;",
                            key: "{i}",
                            "{name}"
                        }
                    }
                }
            }

            div {
                style: "padding: 20px 0; border-top: 1px solid #ccc; flex-shrink: 0;",
                p { style: "margin: 0 0 12px 0; font-size: 0.9rem; color: #666;", "{status}" }
                button {
                    style: "width: 100%; padding: 20px; font-size: 1.2rem; background: #007AFF; color: white; border-radius: 12px; border: none; cursor: pointer;",
                    disabled: processing(),
                    onclick: move |_| {
                        processing.set(true);
                        status.set("Recording 10 seconds...".to_string());
                        let db_clone = db();
                        let sel = selected_block();
                        let blocks_clone = blocks.read().clone();
                        spawn(async move {
                            match run_dictation_flow(db_clone, sel, &blocks_clone, status).await {
                                Ok(()) => {
                                    status.set("Done. Sticker updated.".to_string());
                                    refresh.set(refresh() + 1);
                                }
                                Err(e) => status.set(format!("Error: {}", e)),
                            }
                            processing.set(false);
                        });
                    },
                    "🎤 Dictate Observation"
                }
            }
        }
    }
}

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
) -> anyhow::Result<()> {
    status.set("Capturing audio...".to_string());
    let samples = wowasticker::audio::capture_audio()?;

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
    }

    Ok(())
}
