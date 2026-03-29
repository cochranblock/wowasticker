// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f151=WASM bridge for PWA. Exposes DB functions to JavaScript via wasm-bindgen.
//! Feature-gated: only compiled with --features wasm.

#[cfg(feature = "wasm")]
mod imp {
    use crate::db::{t119, t122, t123};
    use crate::report;
    use std::sync::Mutex;
    use wasm_bindgen::prelude::*;

    static DB: Mutex<Option<t123>> = Mutex::new(None);

    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&t123) -> R,
    {
        let mut guard = DB.lock().unwrap();
        if guard.is_none() {
            let db = t123::f121(":memory:").expect("open in-memory db");
            let _ = db.f140();
            let _ = db.f123();
            *guard = Some(db);
        }
        f(guard.as_ref().unwrap())
    }

    #[wasm_bindgen]
    pub fn wasm_get_blocks() -> String {
        with_db(|db| {
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let blocks = db.f124().unwrap_or_default();
            let mut arr = Vec::new();
            for b in &blocks {
                let rec = db.f143(b.s0, &today).ok().flatten();
                let value = rec.as_ref().map(|r| r.s5 as i32).unwrap_or(0);
                let note = rec
                    .as_ref()
                    .and_then(|r| r.s9.clone())
                    .unwrap_or_default();
                arr.push(format!(
                    r#"{{"id":{},"name":"{}","value":{},"note":"{}"}}"#,
                    b.s0,
                    b.s1.replace('"', "\\\""),
                    value,
                    note.replace('"', "\\\"")
                ));
            }
            format!("[{}]", arr.join(","))
        })
    }

    #[wasm_bindgen]
    pub fn wasm_set_sticker(block_id: i64, value: i32) -> String {
        with_db(|db| {
            let sv = match value {
                1 => t119::One,
                2 => t119::Two,
                _ => t119::Zero,
            };
            match db.f135(block_id, sv, None) {
                Ok(()) => r#"{"ok":true}"#.to_string(),
                Err(e) => format!(r#"{{"error":"{}"}}"#, e),
            }
        })
    }

    #[wasm_bindgen]
    pub fn wasm_get_report(date: &str) -> String {
        with_db(|db| {
            let student = db.f141().ok().flatten().unwrap_or(t122 {
                s6: 0,
                s7: "Student".to_string(),
                s8: 15,
            });
            let records = db.f144(date).unwrap_or_default();
            let earned = db.f145(date).unwrap_or(0);
            report::f147(&student, date, &records, earned)
        })
    }

    #[wasm_bindgen]
    pub fn wasm_get_progress() -> String {
        with_db(|db| {
            let student = db.f141().ok().flatten();
            let earned = db.f142().unwrap_or(0);
            let goal = student.as_ref().map(|s| s.s8).unwrap_or(15);
            if earned >= goal {
                format!("{} / {} Stickers — Goal met!", earned, goal)
            } else {
                format!("{} / {} Stickers", earned, goal)
            }
        })
    }
}
