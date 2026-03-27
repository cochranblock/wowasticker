// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! wowasticker: Pure Rust, offline-first mobile app for student behavioral goals.
//! Local AI dictation via Candle Whisper, SQLite persistence, thumb-zone UI.

#![allow(non_snake_case)]

mod ui;

fn main() {
    dioxus::launch(ui::App);
}
