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
pub enum StickerValue {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
}

/// t120=ScheduleBlock. s0=id, s1=name, s2=sort_order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleBlock {
    pub id: i64,
    pub name: String,
    pub sort_order: i32,
}

/// t121=StickerRecord. s3=block_id, s4=date, s5=value, s9=note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickerRecord {
    pub block_id: i64,
    pub date: String,
    pub value: StickerValue,
    #[serde(default)]
    pub note: Option<String>,
}

/// t122=Student. s6=id, s7=name, s8=goal_stickers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub goal_stickers: i32,
}

/// t123=Db. Thread-safe via Mutex.
pub struct Db(Mutex<Connection>);

impl PartialEq for Db {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Db {
    /// f121=db_open. Open or create database at path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .with_context(|| format!("open db: {}", path.as_ref().display()))?;
        Self::init(&conn)?;
        Ok(Self(Mutex::new(conn)))
    }

    /// f122=db_init. Create tables if not exist. Migration: add note column.
    fn init(conn: &Connection) -> Result<()> {
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
    pub fn ensure_default_schedule(&self) -> Result<()> {
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
    pub fn list_blocks(&self) -> Result<Vec<ScheduleBlock>> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, sort_order FROM schedule_blocks ORDER BY sort_order",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(ScheduleBlock {
                id: r.get(0)?,
                name: r.get(1)?,
                sort_order: r.get(2)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// f125=get_sticker. Get sticker value for block on date.
    pub fn get_sticker(&self, block_id: i64, date: &str) -> Result<StickerValue> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT value FROM sticker_records WHERE block_id = ?1 AND date = ?2",
        )?;
        let mut rows = stmt.query(params![block_id, date])?;
        let v: i32 = match rows.next()? {
            Some(r) => r.get(0)?,
            None => return Ok(StickerValue::Zero),
        };
        Ok(match v {
            1 => StickerValue::One,
            2 => StickerValue::Two,
            _ => StickerValue::Zero,
        })
    }

    /// f126=set_sticker. Set sticker value for block on date.
    pub fn set_sticker(&self, block_id: i64, date: &str, value: StickerValue) -> Result<()> {
        self.set_sticker_with_note(block_id, date, value, None)
    }

    /// f136=set_sticker_with_note. Set sticker and optional note.
    pub fn set_sticker_with_note(
        &self,
        block_id: i64,
        date: &str,
        value: StickerValue,
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
    pub fn get_sticker_today(&self, block_id: i64) -> Result<StickerValue> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.get_sticker(block_id, &date)
    }

    /// f128=set_sticker_today. Set sticker for block on today.
    pub fn set_sticker_today(&self, block_id: i64, value: StickerValue) -> Result<()> {
        self.set_sticker_today_with_note(block_id, value, None)
    }

    /// f135=set_sticker_today_with_note. Set sticker and note for today.
    pub fn set_sticker_today_with_note(
        &self,
        block_id: i64,
        value: StickerValue,
        note: Option<&str>,
    ) -> Result<()> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.set_sticker_with_note(block_id, &date, value, note)
    }

    /// f140=ensure_default_student. Create default student if none exists.
    pub fn ensure_default_student(&self) -> Result<()> {
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
    pub fn get_student(&self) -> Result<Option<Student>> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt =
            conn.prepare("SELECT id, name, goal_stickers FROM students LIMIT 1")?;
        let mut rows = stmt.query([])?;
        match rows.next()? {
            Some(r) => Ok(Some(Student {
                id: r.get(0)?,
                name: r.get(1)?,
                goal_stickers: r.get(2)?,
            })),
            None => Ok(None),
        }
    }

    /// f142=count_stickers_today. Sum sticker values for today across all blocks.
    pub fn count_stickers_today(&self) -> Result<i32> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let total: i32 = conn.query_row(
            "SELECT COALESCE(SUM(value), 0) FROM sticker_records WHERE date = ?1",
            params![date],
            |r| r.get(0),
        )?;
        Ok(total)
    }

    /// f143=get_sticker_record. Get full StickerRecord (value + note) for block on date.
    pub fn get_sticker_record(&self, block_id: i64, date: &str) -> Result<Option<StickerRecord>> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT block_id, date, value, note FROM sticker_records WHERE block_id = ?1 AND date = ?2",
        )?;
        let mut rows = stmt.query(params![block_id, date])?;
        match rows.next()? {
            Some(r) => {
                let v: i32 = r.get(2)?;
                Ok(Some(StickerRecord {
                    block_id: r.get(0)?,
                    date: r.get(1)?,
                    value: match v {
                        1 => StickerValue::One,
                        2 => StickerValue::Two,
                        _ => StickerValue::Zero,
                    },
                    note: r.get(3)?,
                }))
            }
            None => Ok(None),
        }
    }

    /// f144=list_day_records. All blocks with their sticker records for a date.
    pub fn list_day_records(&self, date: &str) -> Result<Vec<(ScheduleBlock, Option<StickerRecord>)>> {
        let blocks = self.list_blocks()?;
        let mut out = Vec::with_capacity(blocks.len());
        for block in blocks {
            let record = self.get_sticker_record(block.id, date)?;
            out.push((block, record));
        }
        Ok(out)
    }

    /// f145=count_stickers_for_date. Sum sticker values for a specific date.
    pub fn count_stickers_for_date(&self, date: &str) -> Result<i32> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        let total: i32 = conn.query_row(
            "SELECT COALESCE(SUM(value), 0) FROM sticker_records WHERE date = ?1",
            params![date],
            |r| r.get(0),
        )?;
        Ok(total)
    }

    /// f146=delete_sticker. Remove a sticker record for block on date (undo).
    pub fn delete_sticker(&self, block_id: i64, date: &str) -> Result<bool> {
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
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        assert!(!blocks.is_empty());
        assert_eq!(blocks[0].name, "Cultural Arts");
    }

    /// f125/f126=db_set_and_get_sticker
    #[test]
    fn db_set_and_get_sticker() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let block_id = blocks[0].id;
        let date = "2026-03-03";
        db.set_sticker(block_id, date, StickerValue::Two).unwrap();
        assert_eq!(db.get_sticker(block_id, date).unwrap(), StickerValue::Two);
    }

    /// f125=get_sticker returns Zero when no record
    #[test]
    fn db_get_sticker_returns_zero_when_no_record() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let block_id = blocks[0].id;
        assert_eq!(db.get_sticker(block_id, "2026-01-01").unwrap(), StickerValue::Zero);
    }

    /// f123=ensure_default_schedule idempotent
    #[test]
    fn db_ensure_default_schedule_idempotent() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        assert_eq!(blocks.len(), 5);
    }

    /// f136=set_sticker_with_note with note
    #[test]
    fn db_set_sticker_with_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let block_id = blocks[0].id;
        let date = "2026-03-03";
        db.set_sticker_with_note(block_id, date, StickerValue::One, Some("Good focus")).unwrap();
        db.set_sticker(block_id, date, StickerValue::Two).unwrap(); // overwrite value
        assert_eq!(db.get_sticker(block_id, date).unwrap(), StickerValue::Two);
    }

    /// f135=set_sticker_today_with_note
    #[test]
    fn db_set_sticker_today_with_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let block_id = blocks[0].id;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        db.set_sticker_today_with_note(block_id, StickerValue::Two, Some("Excellent!")).unwrap();
        assert_eq!(db.get_sticker(block_id, &today).unwrap(), StickerValue::Two);
    }

    /// f140=ensure_default_student creates student
    #[test]
    fn db_ensure_default_student() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_student().unwrap();
        let s = db.get_student().unwrap().unwrap();
        assert_eq!(s.name, "Luka");
        assert_eq!(s.goal_stickers, 15);
    }

    /// f140=ensure_default_student idempotent
    #[test]
    fn db_ensure_default_student_idempotent() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_student().unwrap();
        db.ensure_default_student().unwrap();
        let s = db.get_student().unwrap().unwrap();
        assert_eq!(s.name, "Luka");
    }

    /// f141=get_student returns None when empty
    #[test]
    fn db_get_student_returns_none_when_empty() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        assert!(db.get_student().unwrap().is_none());
    }

    /// f142=count_stickers_today
    #[test]
    fn db_count_stickers_today() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        assert_eq!(db.count_stickers_today().unwrap(), 0);
        db.set_sticker_today(blocks[0].id, StickerValue::Two).unwrap();
        db.set_sticker_today(blocks[1].id, StickerValue::One).unwrap();
        assert_eq!(db.count_stickers_today().unwrap(), 3); // 2 + 1
    }

    /// f143=get_sticker_record returns full record with note
    #[test]
    fn db_get_sticker_record_with_note() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let block_id = blocks[0].id;
        let date = "2026-03-27";
        db.set_sticker_with_note(block_id, date, StickerValue::Two, Some("Great focus"))
            .unwrap();
        let rec = db.get_sticker_record(block_id, date).unwrap().unwrap();
        assert_eq!(rec.value, StickerValue::Two);
        assert_eq!(rec.note.as_deref(), Some("Great focus"));
    }

    /// f143=get_sticker_record returns None when no record
    #[test]
    fn db_get_sticker_record_returns_none() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        assert!(db.get_sticker_record(blocks[0].id, "2026-01-01").unwrap().is_none());
    }

    /// f144=list_day_records returns all blocks with records
    #[test]
    fn db_list_day_records() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let date = "2026-03-27";
        db.set_sticker_with_note(blocks[0].id, date, StickerValue::Two, Some("Great"))
            .unwrap();
        db.set_sticker(blocks[2].id, date, StickerValue::One).unwrap();
        let records = db.list_day_records(date).unwrap();
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
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let date = "2026-03-27";
        assert_eq!(db.count_stickers_for_date(date).unwrap(), 0);
        db.set_sticker(blocks[0].id, date, StickerValue::Two).unwrap();
        db.set_sticker(blocks[1].id, date, StickerValue::One).unwrap();
        assert_eq!(db.count_stickers_for_date(date).unwrap(), 3);
    }

    /// f146=delete_sticker removes record
    #[test]
    fn db_delete_sticker() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        let date = "2026-03-27";
        db.set_sticker(blocks[0].id, date, StickerValue::Two).unwrap();
        assert_eq!(db.get_sticker(blocks[0].id, date).unwrap(), StickerValue::Two);
        assert!(db.delete_sticker(blocks[0].id, date).unwrap());
        assert_eq!(db.get_sticker(blocks[0].id, date).unwrap(), StickerValue::Zero);
    }

    /// f146=delete_sticker returns false when no record
    #[test]
    fn db_delete_sticker_returns_false_when_none() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().with_extension("db");
        let db = Db::open(&path).unwrap();
        db.ensure_default_schedule().unwrap();
        let blocks = db.list_blocks().unwrap();
        assert!(!db.delete_sticker(blocks[0].id, "2026-01-01").unwrap());
    }
}
