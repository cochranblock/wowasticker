// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Local SQLite persistence for student profiles, schedules, and sticker counts.
//! Offline-first: all data stored on-device.

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// t119=StickerValue. 0=none, 1=partial, 2=full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum t119 {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
}

/// t120=ScheduleBlock. s0=id, s1=name, s2=sort_order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct t120 {
    #[serde(rename = "id")]
    pub s0: i64,
    #[serde(rename = "name")]
    pub s1: String,
    #[serde(rename = "sort_order")]
    pub s2: i32,
}

/// t121=StickerRecord. s3=block_id, s4=date, s5=value, s9=note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct t121 {
    #[serde(rename = "block_id")]
    pub s3: i64,
    #[serde(rename = "date")]
    pub s4: String,
    #[serde(rename = "value")]
    pub s5: t119,
    #[serde(default)]
    #[serde(rename = "note")]
    pub s9: Option<String>,
}

/// t122=Student. s6=id, s7=name, s8=goal_stickers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct t122 {
    #[serde(rename = "id")]
    pub s6: i64,
    #[serde(rename = "name")]
    pub s7: String,
    #[serde(rename = "goal_stickers")]
    pub s8: i32,
}

/// t123=Db. Thread-safe via Mutex.
pub struct t123(Mutex<Connection>);

impl PartialEq for t123 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl t123 {
    /// f121=db_open. Open or create database at path.
    pub fn f121(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .with_context(|| format!("open db: {}", path.as_ref().display()))?;
        Self::f122(&conn)?;
        Ok(Self(Mutex::new(conn)))
    }

    /// f122=db_init. Create tables if not exist. Migration: add note column.
    fn f122(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                goal_stickers INTEGER NOT NULL DEFAULT 15
            );
            CREATE TABLE IF NOT EXISTS schedule_blocks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                sort_order INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS sticker_records (
                block_id INTEGER NOT NULL,
                date TEXT NOT NULL,
                value INTEGER NOT NULL,
                note TEXT,
                PRIMARY KEY (block_id, date),
                FOREIGN KEY (block_id) REFERENCES schedule_blocks(id)
            );
            CREATE INDEX IF NOT EXISTS idx_sticker_date ON sticker_records(date);
            "#,
        )?;
        // Migration: add note column (ignored if exists)
        let _ = conn.execute("ALTER TABLE sticker_records ADD COLUMN note TEXT", []);
        Ok(())
    }

    /// f123=ensure_default_schedule. Insert default blocks if none exist.
    pub fn f123(&self) -> Result<()> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM schedule_blocks", [], |r| r.get(0))?;
        if count > 0 {
            return Ok(());
        }
        let blocks = [
            "Cultural Arts",
            "Community Circle",
            "Math",
            "Recess",
            "Lunch",
        ];
        for (i, name) in blocks.iter().enumerate() {
            conn.execute(
                "INSERT INTO schedule_blocks (name, sort_order) VALUES (?1, ?2)",
                params![name, i as i32],
            )?;
        }
        Ok(())
    }

    /// f124=list_blocks. List schedule blocks by sort_order.
    pub fn f124(&self) -> Result<Vec<t120>> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, sort_order FROM schedule_blocks ORDER BY sort_order",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(t120 {
                s0: r.get(0)?,
                s1: r.get(1)?,
                s2: r.get(2)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// f125=get_sticker. Get sticker value for block on date.
    pub fn f125(&self, block_id: i64, date: &str) -> Result<t119> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT value FROM sticker_records WHERE block_id = ?1 AND date = ?2",
        )?;
        let mut rows = stmt.query(params![block_id, date])?;
        let v: i32 = match rows.next()? {
            Some(r) => r.get(0)?,
            None => return Ok(t119::Zero),
        };
        Ok(match v {
            1 => t119::One,
            2 => t119::Two,
            _ => t119::Zero,
        })
    }

    /// f126=set_sticker. Set sticker value for block on date.
    pub fn f126(&self, block_id: i64, date: &str, value: t119) -> Result<()> {
        self.f136(block_id, date, value, None)
    }

    /// f136=set_sticker_with_note. Set sticker and optional note.
    pub fn f136(
        &self,
        block_id: i64,
        date: &str,
        value: t119,
        note: Option<&str>,
    ) -> Result<()> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        conn.execute(
            r#"
            INSERT INTO sticker_records (block_id, date, value, note)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(block_id, date) DO UPDATE SET value = ?3, note = ?4
            "#,
            params![block_id, date, value as i32, note],
        )?;
        Ok(())
    }

    /// f127=get_sticker_today. Get sticker for block on today.
    pub fn f127(&self, block_id: i64) -> Result<t119> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.f125(block_id, &date)
    }

    /// f128=set_sticker_today. Set sticker for block on today.
    pub fn f128(&self, block_id: i64, value: t119) -> Result<()> {
        self.f135(block_id, value, None)
    }

    /// f135=set_sticker_today_with_note. Set sticker and note for today.
    pub fn f135(
        &self,
        block_id: i64,
        value: t119,
        note: Option<&str>,
    ) -> Result<()> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.f136(block_id, &date, value, note)
    }

    /// f140=ensure_default_student. Create default student if none exists.
    pub fn f140(&self) -> Result<()> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM students", [], |r| r.get(0))?;
        if count > 0 {
            return Ok(());
        }
        conn.execute(
            "INSERT INTO students (name, goal_stickers) VALUES (?1, ?2)",
            params!["Luka", 15],
        )?;
        Ok(())
    }

    /// f141=get_student. Get first student (single-student mode).
    pub fn f141(&self) -> Result<Option<t122>> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt =
            conn.prepare("SELECT id, name, goal_stickers FROM students LIMIT 1")?;
        let mut rows = stmt.query([])?;
        match rows.next()? {
            Some(r) => Ok(Some(t122 {
                s6: r.get(0)?,
                s7: r.get(1)?,
                s8: r.get(2)?,
            })),
            None => Ok(None),
        }
    }

    /// f142=count_stickers_today. Sum sticker values for today across all blocks.
    pub fn f142(&self) -> Result<i32> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let total: i32 = conn.query_row(
            "SELECT COALESCE(SUM(value), 0) FROM sticker_records WHERE date = ?1",
            params![date],
            |r| r.get(0),
        )?;
        Ok(total)
    }

    /// f143=get_sticker_record. Get full t121 (value + note) for block on date.
    pub fn f143(&self, block_id: i64, date: &str) -> Result<Option<t121>> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT block_id, date, value, note FROM sticker_records WHERE block_id = ?1 AND date = ?2",
        )?;
        let mut rows = stmt.query(params![block_id, date])?;
        match rows.next()? {
            Some(r) => {
                let v: i32 = r.get(2)?;
                Ok(Some(t121 {
                    s3: r.get(0)?,
                    s4: r.get(1)?,
                    s5: match v {
                        1 => t119::One,
                        2 => t119::Two,
                        _ => t119::Zero,
                    },
                    s9: r.get(3)?,
                }))
            }
            None => Ok(None),
        }
    }

    /// f144=list_day_records. All blocks with their sticker records for a date.
    pub fn f144(&self, date: &str) -> Result<Vec<(t120, Option<t121>)>> {
        let blocks = self.f124()?;
        let mut out = Vec::with_capacity(blocks.len());
        for block in blocks {
            let record = self.f143(block.s0, date)?;
            out.push((block, record));
        }
        Ok(out)
    }

    /// f145=count_stickers_for_date. Sum sticker values for a specific date.
    pub fn f145(&self, date: &str) -> Result<i32> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let total: i32 = conn.query_row(
            "SELECT COALESCE(SUM(value), 0) FROM sticker_records WHERE date = ?1",
            params![date],
            |r| r.get(0),
        )?;
        Ok(total)
    }

    /// f152=update_student. Update name and/or goal for a student.
    pub fn f152(&self, id: i64, name: &str, goal: i32) -> Result<()> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        conn.execute(
            "UPDATE students SET name = ?1, goal_stickers = ?2 WHERE id = ?3",
            params![name, goal, id],
        )?;
        Ok(())
    }

    /// f153=add_block. Insert a new schedule block. Returns its id.
    pub fn f153(&self, name: &str) -> Result<i64> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let max_order: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), -1) FROM schedule_blocks",
                [],
                |r| r.get(0),
            )
            .unwrap_or(-1);
        conn.execute(
            "INSERT INTO schedule_blocks (name, sort_order) VALUES (?1, ?2)",
            params![name, max_order + 1],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// f154=rename_block. Rename a schedule block.
    pub fn f154(&self, id: i64, name: &str) -> Result<()> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        conn.execute(
            "UPDATE schedule_blocks SET name = ?1 WHERE id = ?2",
            params![name, id],
        )?;
        Ok(())
    }

    /// f155=delete_block. Remove a schedule block and its sticker records.
    pub fn f155(&self, id: i64) -> Result<bool> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        conn.execute(
            "DELETE FROM sticker_records WHERE block_id = ?1",
            params![id],
        )?;
        let deleted = conn.execute(
            "DELETE FROM schedule_blocks WHERE id = ?1",
            params![id],
        )?;
        Ok(deleted > 0)
    }

    /// f146=delete_sticker. Remove a sticker record for block on date (undo).
    pub fn f146(&self, block_id: i64, date: &str) -> Result<bool> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let deleted = conn.execute(
            "DELETE FROM sticker_records WHERE block_id = ?1 AND date = ?2",
            params![block_id, date],
        )?;
        Ok(deleted > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// f121/f123=db_open_and_ensure_schedule
    #[test]
    fn db_open_and_ensure_schedule() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        assert!(!blocks.is_empty());
        assert_eq!(blocks[0].s1, "Cultural Arts");
    }

    /// f125/f126=db_set_and_get_sticker
    #[test]
    fn db_set_and_get_sticker() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let block_id = blocks[0].s0;
        let date = "2026-03-03";
        db.f126(block_id, date, t119::Two).unwrap();
        assert_eq!(db.f125(block_id, date).unwrap(), t119::Two);
    }

    /// f125=get_sticker returns Zero when no record
    #[test]
    fn db_get_sticker_returns_zero_when_no_record() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let block_id = blocks[0].s0;
        assert_eq!(db.f125(block_id, "2026-01-01").unwrap(), t119::Zero);
    }

    /// f123=ensure_default_schedule idempotent
    #[test]
    fn db_ensure_default_schedule_idempotent() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        assert_eq!(blocks.len(), 5);
    }

    /// f136=set_sticker_with_note with note
    #[test]
    fn db_set_sticker_with_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let block_id = blocks[0].s0;
        let date = "2026-03-03";
        db.f136(block_id, date, t119::One, Some("Good focus")).unwrap();
        db.f126(block_id, date, t119::Two).unwrap(); // overwrite value
        assert_eq!(db.f125(block_id, date).unwrap(), t119::Two);
    }

    /// f135=set_sticker_today_with_note
    #[test]
    fn db_set_sticker_today_with_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let block_id = blocks[0].s0;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        db.f135(block_id, t119::Two, Some("Excellent!")).unwrap();
        assert_eq!(db.f125(block_id, &today).unwrap(), t119::Two);
    }

    /// f140=ensure_default_student creates student
    #[test]
    fn db_ensure_default_student() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        assert_eq!(s.s7, "Luka");
        assert_eq!(s.s8, 15);
    }

    /// f140=ensure_default_student idempotent
    #[test]
    fn db_ensure_default_student_idempotent() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        assert_eq!(s.s7, "Luka");
    }

    /// f141=get_student returns None when empty
    #[test]
    fn db_get_student_returns_none_when_empty() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        assert!(db.f141().unwrap().is_none());
    }

    /// f142=count_stickers_today
    #[test]
    fn db_count_stickers_today() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        assert_eq!(db.f142().unwrap(), 0);
        db.f128(blocks[0].s0, t119::Two).unwrap();
        db.f128(blocks[1].s0, t119::One).unwrap();
        assert_eq!(db.f142().unwrap(), 3); // 2 + 1
    }

    /// f143=get_sticker_record returns full record with note
    #[test]
    fn db_get_sticker_record_with_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let block_id = blocks[0].s0;
        let date = "2026-03-27";
        db.f136(block_id, date, t119::Two, Some("Great focus"))
            .unwrap();
        let rec = db.f143(block_id, date).unwrap().unwrap();
        assert_eq!(rec.s5, t119::Two);
        assert_eq!(rec.s9.as_deref(), Some("Great focus"));
    }

    /// f143=get_sticker_record returns None when no record
    #[test]
    fn db_get_sticker_record_returns_none() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        assert!(db.f143(blocks[0].s0, "2026-01-01").unwrap().is_none());
    }

    /// f144=list_day_records returns all blocks with records
    #[test]
    fn db_list_day_records() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let date = "2026-03-27";
        db.f136(blocks[0].s0, date, t119::Two, Some("Great"))
            .unwrap();
        db.f126(blocks[2].s0, date, t119::One).unwrap();
        let records = db.f144(date).unwrap();
        assert_eq!(records.len(), 5); // all blocks present
        assert!(records[0].1.is_some()); // Cultural Arts has record
        assert!(records[1].1.is_none()); // Community Circle has no record
        assert!(records[2].1.is_some()); // Math has record
    }

    /// f145=count_stickers_for_date
    #[test]
    fn db_count_stickers_for_date() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let date = "2026-03-27";
        assert_eq!(db.f145(date).unwrap(), 0);
        db.f126(blocks[0].s0, date, t119::Two).unwrap();
        db.f126(blocks[1].s0, date, t119::One).unwrap();
        assert_eq!(db.f145(date).unwrap(), 3);
    }

    /// f146=delete_sticker removes record
    #[test]
    fn db_delete_sticker() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let date = "2026-03-27";
        db.f126(blocks[0].s0, date, t119::Two).unwrap();
        assert_eq!(db.f125(blocks[0].s0, date).unwrap(), t119::Two);
        assert!(db.f146(blocks[0].s0, date).unwrap());
        assert_eq!(db.f125(blocks[0].s0, date).unwrap(), t119::Zero);
    }

    /// f146=delete_sticker returns false when no record
    #[test]
    fn db_delete_sticker_returns_false_when_none() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        assert!(!db.f146(blocks[0].s0, "2026-01-01").unwrap());
    }

    /// f152=update_student changes name and goal
    #[test]
    fn db_update_student() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        assert_eq!(s.s7, "Luka");
        assert_eq!(s.s8, 15);
        db.f152(s.s6, "Maya", 20).unwrap();
        let s2 = db.f141().unwrap().unwrap();
        assert_eq!(s2.s7, "Maya");
        assert_eq!(s2.s8, 20);
    }

    /// f153=add_block inserts with auto sort_order
    #[test]
    fn db_add_block() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let before = db.f124().unwrap().len();
        let id = db.f153("Science").unwrap();
        assert!(id > 0);
        let after = db.f124().unwrap();
        assert_eq!(after.len(), before + 1);
        assert_eq!(after.last().unwrap().s1, "Science");
    }

    /// f154=rename_block changes name
    #[test]
    fn db_rename_block() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        db.f154(blocks[0].s0, "Art Class").unwrap();
        let updated = db.f124().unwrap();
        assert_eq!(updated[0].s1, "Art Class");
    }

    /// f155=delete_block removes block and its sticker records
    #[test]
    fn db_delete_block() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        db.f126(id, "2026-04-01", t119::Two).unwrap();
        assert!(db.f155(id).unwrap());
        let remaining = db.f124().unwrap();
        assert_eq!(remaining.len(), blocks.len() - 1);
        assert_eq!(db.f125(id, "2026-04-01").unwrap(), t119::Zero);
    }

    // ===== EDGE CASES: db_open =====

    #[test]
    fn db_open_invalid_path() {
        let result = t123::f121("/nonexistent/deeply/nested/path/db.sqlite");
        assert!(result.is_err());
    }

    // ===== EDGE CASES: sticker values =====

    #[test]
    fn db_sticker_overwrite_updates_value() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        db.f126(id, "2026-04-01", t119::One).unwrap();
        assert_eq!(db.f125(id, "2026-04-01").unwrap(), t119::One);
        db.f126(id, "2026-04-01", t119::Two).unwrap();
        assert_eq!(db.f125(id, "2026-04-01").unwrap(), t119::Two);
    }

    #[test]
    fn db_sticker_multiple_blocks_same_date() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let date = "2026-04-01";
        db.f126(blocks[0].s0, date, t119::Two).unwrap();
        db.f126(blocks[1].s0, date, t119::One).unwrap();
        db.f126(blocks[2].s0, date, t119::Two).unwrap();
        assert_eq!(db.f145(date).unwrap(), 5); // 2+1+2
    }

    #[test]
    fn db_sticker_same_block_different_dates() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        db.f126(id, "2026-04-01", t119::Two).unwrap();
        db.f126(id, "2026-04-02", t119::One).unwrap();
        assert_eq!(db.f125(id, "2026-04-01").unwrap(), t119::Two);
        assert_eq!(db.f125(id, "2026-04-02").unwrap(), t119::One);
        assert_eq!(db.f145("2026-04-01").unwrap(), 2);
        assert_eq!(db.f145("2026-04-02").unwrap(), 1);
    }

    #[test]
    fn db_get_sticker_negative_block_id() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        assert_eq!(db.f125(-1, "2026-04-01").unwrap(), t119::Zero);
    }

    #[test]
    fn db_count_stickers_empty_date() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        assert_eq!(db.f145("2099-01-01").unwrap(), 0);
    }

    // ===== EDGE CASES: notes =====

    #[test]
    fn db_note_empty_string() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        db.f136(blocks[0].s0, "2026-04-01", t119::Two, Some("")).unwrap();
        let rec = db.f143(blocks[0].s0, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9, Some("".to_string()));
    }

    #[test]
    fn db_note_sql_injection() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let malicious = "'; DROP TABLE sticker_records; --";
        db.f136(blocks[0].s0, "2026-04-01", t119::One, Some(malicious)).unwrap();
        let rec = db.f143(blocks[0].s0, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9.as_deref(), Some(malicious));
        // table must still work
        assert!(db.f124().is_ok());
    }

    #[test]
    fn db_note_with_quotes() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let note = r#"Said "Hello" and 'goodbye'"#;
        db.f136(blocks[0].s0, "2026-04-01", t119::Two, Some(note)).unwrap();
        let rec = db.f143(blocks[0].s0, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9.as_deref(), Some(note));
    }

    #[test]
    fn db_note_with_newlines() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let note = "Line 1\nLine 2\nLine 3";
        db.f136(blocks[0].s0, "2026-04-01", t119::One, Some(note)).unwrap();
        let rec = db.f143(blocks[0].s0, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9.as_deref(), Some(note));
    }

    #[test]
    fn db_note_with_unicode() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let note = "Très bien! 👍 Excelente trabajo 🌟";
        db.f136(blocks[0].s0, "2026-04-01", t119::Two, Some(note)).unwrap();
        let rec = db.f143(blocks[0].s0, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9.as_deref(), Some(note));
    }

    #[test]
    fn db_note_very_long() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let long_note = "x".repeat(10_000);
        db.f136(blocks[0].s0, "2026-04-01", t119::Two, Some(&long_note)).unwrap();
        let rec = db.f143(blocks[0].s0, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9.as_deref(), Some(long_note.as_str()));
    }

    #[test]
    fn db_note_overwrite_replaces_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        db.f136(id, "2026-04-01", t119::One, Some("first")).unwrap();
        db.f136(id, "2026-04-01", t119::Two, Some("second")).unwrap();
        let rec = db.f143(id, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s5, t119::Two);
        assert_eq!(rec.s9.as_deref(), Some("second"));
    }

    #[test]
    fn db_note_none_clears_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        db.f136(id, "2026-04-01", t119::Two, Some("a note")).unwrap();
        db.f136(id, "2026-04-01", t119::Two, None).unwrap();
        let rec = db.f143(id, "2026-04-01").unwrap().unwrap();
        assert_eq!(rec.s9, None);
    }

    // ===== EDGE CASES: student =====

    #[test]
    fn db_update_student_empty_name() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        db.f152(s.s6, "", 15).unwrap();
        let s2 = db.f141().unwrap().unwrap();
        assert_eq!(s2.s7, "");
    }

    #[test]
    fn db_update_student_zero_goal() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        db.f152(s.s6, "Test", 0).unwrap();
        assert_eq!(db.f141().unwrap().unwrap().s8, 0);
    }

    #[test]
    fn db_update_student_negative_goal() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        db.f152(s.s6, "Test", -5).unwrap();
        assert_eq!(db.f141().unwrap().unwrap().s8, -5);
    }

    #[test]
    fn db_update_student_max_goal() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        db.f152(s.s6, "Test", i32::MAX).unwrap();
        assert_eq!(db.f141().unwrap().unwrap().s8, i32::MAX);
    }

    #[test]
    fn db_update_student_nonexistent_id() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f152(999, "Ghost", 20).unwrap(); // no error, no student created
        assert!(db.f141().unwrap().is_none());
    }

    #[test]
    fn db_update_student_unicode_name() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f140().unwrap();
        let s = db.f141().unwrap().unwrap();
        db.f152(s.s6, "José García-López", 10).unwrap();
        assert_eq!(db.f141().unwrap().unwrap().s7, "José García-López");
    }

    // ===== EDGE CASES: blocks =====

    #[test]
    fn db_add_block_empty_name() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        let id = db.f153("").unwrap();
        assert!(id > 0);
        let blocks = db.f124().unwrap();
        assert_eq!(blocks[0].s1, "");
    }

    #[test]
    fn db_add_block_sql_injection() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        let name = "Block'; DROP TABLE schedule_blocks;--";
        db.f153(name).unwrap();
        let blocks = db.f124().unwrap();
        assert_eq!(blocks[0].s1, name);
        assert!(db.f124().is_ok());
    }

    #[test]
    fn db_add_block_unicode() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        let name = "Classe de Français 🎓";
        db.f153(name).unwrap();
        let blocks = db.f124().unwrap();
        assert_eq!(blocks[0].s1, name);
    }

    #[test]
    fn db_add_block_very_long_name() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        let name = "x".repeat(10_000);
        db.f153(&name).unwrap();
        let blocks = db.f124().unwrap();
        assert_eq!(blocks[0].s1, name);
    }

    #[test]
    fn db_add_multiple_blocks_sort_order() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f153("A").unwrap();
        db.f153("B").unwrap();
        db.f153("C").unwrap();
        let blocks = db.f124().unwrap();
        assert_eq!(blocks.len(), 3);
        assert!(blocks[0].s2 < blocks[1].s2);
        assert!(blocks[1].s2 < blocks[2].s2);
    }

    #[test]
    fn db_rename_block_to_empty() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        db.f154(blocks[0].s0, "").unwrap();
        assert_eq!(db.f124().unwrap()[0].s1, "");
    }

    #[test]
    fn db_rename_nonexistent_block() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f154(999, "NewName").unwrap(); // no error
    }

    #[test]
    fn db_delete_nonexistent_block() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        assert!(!db.f155(999).unwrap());
    }

    #[test]
    fn db_delete_block_with_many_records() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        for day in 1..=30 {
            let date = format!("2026-04-{:02}", day);
            db.f126(id, &date, t119::Two).unwrap();
        }
        assert!(db.f155(id).unwrap());
        for day in 1..=30 {
            let date = format!("2026-04-{:02}", day);
            assert_eq!(db.f125(id, &date).unwrap(), t119::Zero);
        }
    }

    // ===== EDGE CASES: list_day_records =====

    #[test]
    fn db_list_day_records_no_blocks() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        // no default schedule
        let records = db.f144("2026-04-01").unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn db_list_day_records_all_empty() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let records = db.f144("2026-04-01").unwrap();
        assert_eq!(records.len(), 5);
        for (_, rec) in &records {
            assert!(rec.is_none());
        }
    }

    #[test]
    fn db_list_day_records_mixed() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        db.f126(blocks[0].s0, "2026-04-01", t119::Two).unwrap();
        db.f136(blocks[2].s0, "2026-04-01", t119::One, Some("ok")).unwrap();
        let records = db.f144("2026-04-01").unwrap();
        assert!(records[0].1.is_some());
        assert!(records[1].1.is_none());
        assert!(records[2].1.is_some());
        assert!(records[3].1.is_none());
        assert!(records[4].1.is_none());
    }

    // ===== EDGE CASES: sort_order =====

    #[test]
    fn db_blocks_maintain_sort_order() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        for i in 0..blocks.len() - 1 {
            assert!(blocks[i].s2 <= blocks[i + 1].s2);
        }
    }

    // ===== EDGE CASES: record retrieval =====

    #[test]
    fn db_get_record_nonexistent_block() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        let rec = db.f143(999, "2026-04-01").unwrap();
        assert!(rec.is_none());
    }

    #[test]
    fn db_get_record_preserves_all_fields() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = t123::f121(&path).unwrap();
        db.f123().unwrap();
        let blocks = db.f124().unwrap();
        let id = blocks[0].s0;
        let date = "2026-04-01";
        db.f136(id, date, t119::Two, Some("Great work")).unwrap();
        let rec = db.f143(id, date).unwrap().unwrap();
        assert_eq!(rec.s3, id);
        assert_eq!(rec.s4, date);
        assert_eq!(rec.s5, t119::Two);
        assert_eq!(rec.s9, Some("Great work".to_string()));
    }
}
