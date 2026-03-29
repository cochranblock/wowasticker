// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f150=JNI bridge for Android. Exposes DB functions to Java/WebView.
//! Feature-gated: only compiled with --features jni.

#[cfg(feature = "jni")]
mod imp {
    use crate::db::{t119, t123};
    use crate::report;
    use jni::objects::{JClass, JString};
    use jni::sys::jstring;
    use jni::JNIEnv;
    use std::sync::OnceLock;

    static DB: OnceLock<t123> = OnceLock::new();

    fn db() -> &'static t123 {
        DB.get_or_init(|| {
            let path = "/data/data/org.cochranblock.wowasticker/wowasticker.db";
            let db = t123::f121(path).expect("open db");
            let _ = db.f140();
            let _ = db.f123();
            db
        })
    }

    #[no_mangle]
    pub extern "system" fn Java_org_cochranblock_wowasticker_MainActivity_jniGetBlocks(
        mut env: JNIEnv,
        _class: JClass,
    ) -> jstring {
        let db = db();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let blocks = db.f124().unwrap_or_default();
        let mut arr = Vec::new();
        for b in &blocks {
            let rec = db.f143(b.s0, &today).ok().flatten();
            let value = rec.as_ref().map(|r| r.s5 as i32).unwrap_or(0);
            let note = rec.as_ref().and_then(|r| r.s9.clone()).unwrap_or_default();
            arr.push(format!(
                r#"{{"id":{},"name":"{}","value":{},"note":"{}"}}"#,
                b.s0,
                b.s1.replace('"', "\\\""),
                value,
                note.replace('"', "\\\"")
            ));
        }
        let json = format!("[{}]", arr.join(","));
        env.new_string(json).unwrap().into_raw()
    }

    #[no_mangle]
    pub extern "system" fn Java_org_cochranblock_wowasticker_MainActivity_jniSetSticker(
        mut env: JNIEnv,
        _class: JClass,
        block_id: i64,
        value: i32,
    ) -> jstring {
        let db = db();
        let sv = match value {
            1 => t119::One,
            2 => t119::Two,
            _ => t119::Zero,
        };
        match db.f135(block_id, sv, None) {
            Ok(()) => env.new_string(r#"{"ok":true}"#).unwrap().into_raw(),
            Err(e) => env
                .new_string(format!(r#"{{"error":"{}"}}"#, e))
                .unwrap()
                .into_raw(),
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_org_cochranblock_wowasticker_MainActivity_jniGetReport(
        mut env: JNIEnv,
        _class: JClass,
        date: JString,
    ) -> jstring {
        let db = db();
        let date_str: String = env.get_string(&date).unwrap().into();
        let student = db.f141().ok().flatten().unwrap_or(crate::db::t122 {
            s6: 0,
            s7: "Student".to_string(),
            s8: 15,
        });
        let records = db.f144(&date_str).unwrap_or_default();
        let earned = db.f145(&date_str).unwrap_or(0);
        let text = report::f147(&student, &date_str, &records, earned);
        env.new_string(text).unwrap().into_raw()
    }

    #[no_mangle]
    pub extern "system" fn Java_org_cochranblock_wowasticker_MainActivity_jniGetProgress(
        mut env: JNIEnv,
        _class: JClass,
    ) -> jstring {
        let db = db();
        let student = db.f141().ok().flatten();
        let earned = db.f142().unwrap_or(0);
        let goal = student.as_ref().map(|s| s.s8).unwrap_or(15);
        let msg = if earned >= goal {
            format!("{} / {} Stickers — Goal met!", earned, goal)
        } else {
            format!("{} / {} Stickers", earned, goal)
        };
        env.new_string(msg).unwrap().into_raw()
    }

    #[no_mangle]
    pub extern "system" fn Java_org_cochranblock_wowasticker_MainActivity_jniRunDemo(
        mut env: JNIEnv,
        _class: JClass,
    ) -> jstring {
        env.new_string("Demo runs via CLI").unwrap().into_raw()
    }
}
