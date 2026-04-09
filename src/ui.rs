// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f133=App. Dioxus UI: onboarding, sticker chart, settings. Thumb-zone layout.

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
#[allow(non_camel_case_types)]
struct t125 {
    s13: String,
    s14: i64,
    s15: t119,
    s16: String,
    s17: Vec<String>,
}

/// t126=ViewMode. Which screen is active.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum t126 {
    Onboarding,
    Main,
    Settings,
}

/// f133=App. Root component: onboarding → main → settings.
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
    let mut student = use_signal(|| None::<t122>);
    let mut stickers_earned = use_signal(|| 0i32);
    let mut view_date = use_signal(today_str);
    let mut last_dictation = use_signal(|| None::<(i64, String)>);
    let mut share_status = use_signal(|| None::<String>);
    let mut view_mode = use_signal(|| t126::Main); // will switch to Onboarding if no student

    // Load DB on mount
    use_effect(move || {
        let path = db_path.clone();
        spawn(async move {
            match t123::f121(&path) {
                Ok(d) => {
                    let d = Arc::new(d);
                    match d.f141() {
                        Ok(Some(s)) => {
                            student.set(Some(s));
                            // Existing student — load schedule
                            if d.f123().is_ok() {
                                if let Ok(b) = d.f124() {
                                    blocks.set(b);
                                }
                            }
                            if let Ok(earned) = d.f142() {
                                stickers_earned.set(earned);
                            }
                            view_mode.set(t126::Main);
                        }
                        Ok(None) => {
                            // No student — show onboarding
                            view_mode.set(t126::Onboarding);
                        }
                        Err(e) => {
                            status.set(format!("DB error: {}", e));
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

    match view_mode() {
        t126::Onboarding => rsx! {
            OnboardingView {
                db: db(),
                on_complete: move |(s, b): (t122, Vec<t120>)| {
                    student.set(Some(s));
                    blocks.set(b);
                    if let Some(ref d) = db() {
                        if let Ok(earned) = d.f142() {
                            stickers_earned.set(earned);
                        }
                    }
                    view_mode.set(t126::Main);
                },
            }
        },
        t126::Settings => rsx! {
            SettingsView {
                db: db(),
                student: student().unwrap_or(t122 { s6: 0, s7: "Student".to_string(), s8: 15 }),
                blocks: blocks(),
                on_back: move |_: ()| {
                    // Reload student and blocks from DB
                    if let Some(ref d) = db() {
                        if let Ok(Some(s)) = d.f141() {
                            student.set(Some(s));
                        }
                        if let Ok(b) = d.f124() {
                            blocks.set(b);
                        }
                        if let Ok(earned) = d.f142() {
                            stickers_earned.set(earned);
                        }
                    }
                    refresh.set(refresh() + 1);
                    view_mode.set(t126::Main);
                },
            }
        },
        t126::Main => {
            let s = student().unwrap_or(t122 { s6: 0, s7: "Student".to_string(), s8: 15 });
            let is_today = view_date() == today_str();
            let title = format!("{}'s Sticker Chart", s.s7);
            let goal_met = stickers_earned() >= s.s8;
            let progress_str = if goal_met {
                format!("{} / {} Stickers — Goal met!", stickers_earned(), s.s8)
            } else {
                format!("{} / {} Stickers", stickers_earned(), s.s8)
            };
            let date_display = format_date_display(&view_date());
            let can_go_forward = view_date() < today_str();
            let goal_color = if goal_met { "#2e7d32" } else { "#666" };
            let fwd_border = if can_go_forward { "#ccc" } else { "#eee" };
            let fwd_color = if can_go_forward { "#333" } else { "#ccc" };
            let status_color = if last_error().is_some() { "#c62828" } else { "#666" };
            let status_text = if last_error().is_some() {
                last_error().unwrap_or_default()
            } else if share_status().is_some() {
                share_status().unwrap_or_default()
            } else if blocks.read().is_empty() && !processing() {
                "Loading schedule...".to_string()
            } else {
                status()
            };
            let dictate_bg = if last_error().is_some() { "#e65100" } else { "#007AFF" };
            let dictate_label = if last_error().is_some() { "Retry" } else { "Dictate Observation" };
            let fwd_style = format!("padding: 8px 16px; font-size: 1.1rem; background: none; border: 1px solid {fwd_border}; border-radius: 8px; cursor: pointer; color: {fwd_color};");
            let goal_style = format!("margin: 0 0 12px 0; font-size: 1rem; color: {goal_color};");
            let status_style = format!("margin: 0 0 10px 0; font-size: 0.85rem; color: {status_color};");
            let dictate_style = format!("width: 100%; padding: 18px; font-size: 1.1rem; background: {dictate_bg}; color: white; border-radius: 12px; border: none; cursor: pointer; margin-bottom: 8px;");
            let show_undo = is_today && last_dictation().is_some();

            rsx! {
                div {
                    style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif;",
                    padding_bottom: "env(safe-area-inset-bottom, 20px)",

                    // Header with title and gear icon
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        h1 { style: "margin: 0 0 4px 0; font-size: 1.5rem;", "{title}" }
                        button {
                            style: "background: none; border: none; font-size: 1.5rem; cursor: pointer; padding: 4px 8px;",
                            onclick: move |_| {
                                view_mode.set(t126::Settings);
                            },
                            "⚙"
                        }
                    }
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
                                            let s_val = student().unwrap_or(t122 { s6: 0, s7: "Student".to_string(), s8: 15 });
                                            let text = report::f147(
                                                &s_val,
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
        },
    }
}

/// f157=OnboardingView. First-run setup: enter student name and sticker goal.
#[component]
fn OnboardingView(
    db: Option<Arc<t123>>,
    on_complete: EventHandler<(t122, Vec<t120>)>,
) -> Element {
    let mut name_input = use_signal(|| String::new());
    let mut goal_input = use_signal(|| "15".to_string());
    let mut error_msg = use_signal(|| None::<String>);

    let name_val = name_input();
    let can_submit = !name_val.trim().is_empty();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif; justify-content: center;",

            div {
                style: "max-width: 400px; margin: 0 auto; width: 100%;",

                h1 { style: "text-align: center; margin-bottom: 8px; font-size: 1.8rem;", "WowaSticker" }
                p { style: "text-align: center; color: #666; margin-bottom: 32px;", "Set up your student's sticker chart" }

                label { style: "display: block; font-weight: 600; margin-bottom: 6px;", "Student's first name" }
                input {
                    style: "width: 100%; padding: 14px; font-size: 1.1rem; border: 2px solid #ccc; border-radius: 10px; margin-bottom: 20px; box-sizing: border-box;",
                    r#type: "text",
                    placeholder: "Enter name...",
                    value: "{name_input}",
                    autofocus: true,
                    oninput: move |e| {
                        name_input.set(e.value());
                        error_msg.set(None);
                    },
                }

                label { style: "display: block; font-weight: 600; margin-bottom: 6px;", "Daily sticker goal" }
                input {
                    style: "width: 100%; padding: 14px; font-size: 1.1rem; border: 2px solid #ccc; border-radius: 10px; margin-bottom: 24px; box-sizing: border-box;",
                    r#type: "number",
                    min: "1",
                    max: "100",
                    value: "{goal_input}",
                    oninput: move |e| {
                        goal_input.set(e.value());
                    },
                }

                if let Some(ref msg) = error_msg() {
                    p { style: "color: #c62828; margin-bottom: 12px;", "{msg}" }
                }

                button {
                    style: "width: 100%; padding: 18px; font-size: 1.2rem; background: #007AFF; color: white; border-radius: 12px; border: none; cursor: pointer;",
                    disabled: !can_submit,
                    onclick: move |_| {
                        let name = name_input().trim().to_string();
                        if name.is_empty() {
                            error_msg.set(Some("Please enter a name.".to_string()));
                            return;
                        }
                        let goal: i32 = goal_input().parse().unwrap_or(15).max(1);

                        if let Some(ref d) = db {
                            match d.f156(&name, goal) {
                                Ok(id) => {
                                    let s = t122 { s6: id, s7: name, s8: goal };
                                    let _ = d.f123(); // create default schedule
                                    let b = d.f124().unwrap_or_default();
                                    on_complete.call((s, b));
                                }
                                Err(e) => {
                                    error_msg.set(Some(format!("Error: {}", e)));
                                }
                            }
                        } else {
                            error_msg.set(Some("Database not ready.".to_string()));
                        }
                    },
                    "Start"
                }
            }
        }
    }
}

/// f158=SettingsView. Edit student name, goal, add/rename/delete blocks.
#[component]
fn SettingsView(
    db: Option<Arc<t123>>,
    student: t122,
    blocks: Vec<t120>,
    on_back: EventHandler<()>,
) -> Element {
    // Store db in a signal so multiple closures can access it without moving.
    let db = use_signal(move || db);
    let mut name_input = use_signal(move || student.s7.clone());
    let mut goal_input = use_signal(move || student.s8.to_string());
    let mut local_blocks = use_signal(move || blocks.clone());
    let mut new_block_name = use_signal(|| String::new());
    let mut save_status = use_signal(|| None::<String>);
    let mut editing_block = use_signal(|| None::<(i64, String)>); // (id, current_name)

    let student_id = student.s6;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 20px; font-family: system-ui, sans-serif;",

            // Header
            div {
                style: "display: flex; align-items: center; margin-bottom: 20px;",
                button {
                    style: "background: none; border: none; font-size: 1.3rem; cursor: pointer; padding: 4px 12px 4px 0;",
                    onclick: move |_| {
                        on_back.call(());
                    },
                    "< Back"
                }
                h1 { style: "margin: 0; font-size: 1.5rem;", "Settings" }
            }

            div {
                style: "flex-grow: 1; overflow-y: auto;",

                // Student section
                h2 { style: "font-size: 1.1rem; margin: 0 0 12px 0; color: #333;", "Student" }

                label { style: "display: block; font-weight: 600; margin-bottom: 4px; font-size: 0.9rem;", "Name" }
                input {
                    style: "width: 100%; padding: 12px; font-size: 1rem; border: 2px solid #ccc; border-radius: 8px; margin-bottom: 12px; box-sizing: border-box;",
                    r#type: "text",
                    value: "{name_input}",
                    oninput: move |e| {
                        name_input.set(e.value());
                        save_status.set(None);
                    },
                }

                label { style: "display: block; font-weight: 600; margin-bottom: 4px; font-size: 0.9rem;", "Daily sticker goal" }
                input {
                    style: "width: 100%; padding: 12px; font-size: 1rem; border: 2px solid #ccc; border-radius: 8px; margin-bottom: 12px; box-sizing: border-box;",
                    r#type: "number",
                    min: "1",
                    max: "100",
                    value: "{goal_input}",
                    oninput: move |e| {
                        goal_input.set(e.value());
                        save_status.set(None);
                    },
                }

                button {
                    style: "padding: 12px 24px; font-size: 1rem; background: #007AFF; color: white; border-radius: 8px; border: none; cursor: pointer; margin-bottom: 24px;",
                    onclick: move |_| {
                        let name = name_input().trim().to_string();
                        if name.is_empty() {
                            save_status.set(Some("Name cannot be empty.".to_string()));
                            return;
                        }
                        let goal: i32 = goal_input().parse().unwrap_or(15).max(1);
                        if let Some(d) = db() {
                            match d.f152(student_id, &name, goal) {
                                Ok(()) => save_status.set(Some("Saved!".to_string())),
                                Err(e) => save_status.set(Some(format!("Error: {}", e))),
                            }
                        }
                    },
                    "Save Student"
                }

                if let Some(ref msg) = save_status() {
                    p {
                        style: if msg.starts_with("Error") { "color: #c62828; margin: 0 0 16px 0;" } else { "color: #2e7d32; margin: 0 0 16px 0;" },
                        "{msg}"
                    }
                }

                // Schedule blocks section
                h2 { style: "font-size: 1.1rem; margin: 0 0 12px 0; color: #333; border-top: 1px solid #eee; padding-top: 16px;", "Schedule Blocks" }

                for block in local_blocks.read().iter() {
                    {
                        let block_id = block.s0;
                        let block_name = block.s1.clone();
                        let is_editing = editing_block().map(|(id, _)| id == block_id).unwrap_or(false);
                        rsx! {
                            div {
                                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",

                                if is_editing {
                                    input {
                                        style: "flex: 1; padding: 10px; font-size: 1rem; border: 2px solid #007AFF; border-radius: 8px; box-sizing: border-box;",
                                        r#type: "text",
                                        value: "{editing_block().map(|(_, n)| n).unwrap_or_default()}",
                                        autofocus: true,
                                        oninput: move |e| {
                                            editing_block.set(Some((block_id, e.value())));
                                        },
                                    }
                                    button {
                                        style: "padding: 10px 14px; background: #007AFF; color: white; border: none; border-radius: 8px; cursor: pointer;",
                                        onclick: move |_| {
                                            if let Some((id, new_name)) = editing_block() {
                                                let trimmed = new_name.trim().to_string();
                                                if !trimmed.is_empty() {
                                                    if let Some(d) = db() {
                                                        let _ = d.f154(id, &trimmed);
                                                        if let Ok(b) = d.f124() {
                                                            local_blocks.set(b);
                                                        }
                                                    }
                                                }
                                            }
                                            editing_block.set(None);
                                        },
                                        "Save"
                                    }
                                    button {
                                        style: "padding: 10px 14px; background: #f5f5f5; color: #333; border: 1px solid #ccc; border-radius: 8px; cursor: pointer;",
                                        onclick: move |_| {
                                            editing_block.set(None);
                                        },
                                        "Cancel"
                                    }
                                } else {
                                    div {
                                        style: "flex: 1; padding: 10px 12px; background: #f5f5f5; border-radius: 8px; font-size: 1rem;",
                                        "{block_name}"
                                    }
                                    button {
                                        style: "padding: 8px 12px; background: none; border: 1px solid #ccc; border-radius: 8px; cursor: pointer; font-size: 0.85rem;",
                                        onclick: move |_| {
                                            editing_block.set(Some((block_id, block_name.clone())));
                                        },
                                        "Rename"
                                    }
                                    button {
                                        style: "padding: 8px 12px; background: #ffebee; color: #c62828; border: 1px solid #ef9a9a; border-radius: 8px; cursor: pointer; font-size: 0.85rem;",
                                        onclick: move |_| {
                                            if let Some(d) = db() {
                                                let _ = d.f155(block_id);
                                                if let Ok(b) = d.f124() {
                                                    local_blocks.set(b);
                                                }
                                            }
                                        },
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }

                // Add block
                div {
                    style: "display: flex; gap: 8px; margin-top: 12px;",
                    input {
                        style: "flex: 1; padding: 10px; font-size: 1rem; border: 2px solid #ccc; border-radius: 8px; box-sizing: border-box;",
                        r#type: "text",
                        placeholder: "New block name...",
                        value: "{new_block_name}",
                        oninput: move |e| {
                            new_block_name.set(e.value());
                        },
                    }
                    button {
                        style: "padding: 10px 18px; background: #007AFF; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 1rem;",
                        disabled: new_block_name().trim().is_empty(),
                        onclick: move |_| {
                            let name = new_block_name().trim().to_string();
                            if !name.is_empty() {
                                if let Some(d) = db() {
                                    let _ = d.f153(&name);
                                    if let Ok(b) = d.f124() {
                                        local_blocks.set(b);
                                    }
                                    new_block_name.set(String::new());
                                }
                            }
                        },
                        "Add"
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

    #[test]
    fn view_mode_enum_equality() {
        assert_eq!(t126::Onboarding, t126::Onboarding);
        assert_eq!(t126::Main, t126::Main);
        assert_eq!(t126::Settings, t126::Settings);
        assert_ne!(t126::Onboarding, t126::Main);
        assert_ne!(t126::Main, t126::Settings);
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
