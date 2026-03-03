//! Local SQLite persistence for student profiles, schedules, and sticker counts.
//! Offline-first: all data stored on-device.

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Sticker value: 0 (none), 1 (partial), 2 (full)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StickerValue {
    Zero = 0,
    One = 1,
    Two = 2,
}

impl Default for StickerValue {
    fn default() -> Self {
        Self::Zero
    }
}

/// A schedule block (e.g. Cultural Arts, Math, Recess)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleBlock {
    pub id: i64,
    pub name: String,
    pub sort_order: i32,
}

/// Sticker record for a block on a given day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerRecord {
    pub block_id: i64,
    pub date: String, // YYYY-MM-DD
    pub value: StickerValue,
}

/// Student profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub goal_stickers: i32,
}

/// Database handle. Thread-safe via Mutex.
pub struct Db(Mutex<Connection>);

impl Db {
    /// Open or create database at path
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .with_context(|| format!("open db: {}", path.as_ref().display()))?;
        Self::init(&conn)?;
        Ok(Self(Mutex::new(conn)))
    }

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
                PRIMARY KEY (block_id, date),
                FOREIGN KEY (block_id) REFERENCES schedule_blocks(id)
            );
            CREATE INDEX IF NOT EXISTS idx_sticker_date ON sticker_records(date);
            "#,
        )?;
        Ok(())
    }

    /// Insert default schedule blocks if none exist
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

    /// List schedule blocks
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

    /// Get sticker value for a block on a date
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

    /// Set sticker value for a block on a date
    pub fn set_sticker(&self, block_id: i64, date: &str, value: StickerValue) -> Result<()> {
        let conn = self.0.lock().map_err(|e| anyhow::anyhow!("lock: {}", e))?;
        conn.execute(
            r#"
            INSERT INTO sticker_records (block_id, date, value)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(block_id, date) DO UPDATE SET value = ?3
            "#,
            params![block_id, date, value as i32],
        )?;
        Ok(())
    }

    /// Get sticker value for a block on today
    pub fn get_sticker_today(&self, block_id: i64) -> Result<StickerValue> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.get_sticker(block_id, &date)
    }

    /// Set sticker for today
    pub fn set_sticker_today(&self, block_id: i64, value: StickerValue) -> Result<()> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.set_sticker(block_id, &date, value)
    }
}
