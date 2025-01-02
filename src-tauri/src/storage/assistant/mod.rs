mod models;
pub use models::Assistant;

use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{params, Result};
use crate::storage::db::Database;

use rusqlite::OptionalExtension;

pub struct AssistantStorage<'a> {
    db: &'a Database,
}

impl<'a> AssistantStorage<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, assistant: &Assistant) -> Result<()> {
        self.db.get_connection().await.execute(
            "INSERT INTO assistants (id, name, description, model, temperature, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                assistant.id,
                assistant.name,
                assistant.description,
                assistant.model,
                assistant.temperature,
                assistant.created_at.timestamp(),
                assistant.updated_at.timestamp(),
            ],
        )?;
        Ok(())
    }

    pub async fn get(&self, assistant_id: &str) -> Result<Option<Assistant>> {
        let conn = self.db.get_connection().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, model, temperature, created_at, updated_at 
             FROM assistants WHERE id = ?1",
        )?;

        let assistant = stmt.query_row(params![assistant_id], |row| {
            let created_at_ts: i64 = row.get(5)?;
            let updated_at_ts: i64 = row.get(6)?;
            
            Ok(Assistant {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                model: row.get(3)?,
                temperature: row.get(4)?,
                created_at: Utc.timestamp_opt(created_at_ts, 0).unwrap(),
                updated_at: Utc.timestamp_opt(updated_at_ts, 0).unwrap(),
            })
        }).optional()?;

        Ok(assistant)
    }

    pub async fn list(&self) -> Result<Vec<Assistant>> {
        let conn = self.db.get_connection().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, model, temperature, created_at, updated_at FROM assistants",
        )?;

        let assistants = stmt.query_map([], |row| {
            let created_at_ts: i64 = row.get(5)?;
            let updated_at_ts: i64 = row.get(6)?;
            
            Ok(Assistant {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                model: row.get(3)?,
                temperature: row.get(4)?,
                created_at: Utc.timestamp_opt(created_at_ts, 0).unwrap(),
                updated_at: Utc.timestamp_opt(updated_at_ts, 0).unwrap(),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(assistants)
    }

    pub async fn update(&self, assistant: &Assistant) -> Result<()> {
        let now = Utc::now();
        self.db.get_connection().await.execute(
            "UPDATE assistants 
             SET name = ?1, description = ?2, model = ?3, temperature = ?4, updated_at = ?5
             WHERE id = ?6",
            params![
                assistant.name,
                assistant.description,
                assistant.model,
                assistant.temperature,
                now.timestamp(),
                assistant.id,
            ],
        )?;
        Ok(())
    }

    pub async fn delete(&self, assistant_id: &str) -> Result<()> {
        self.db.get_connection().await.execute(
            "DELETE FROM assistants WHERE id = ?1",
            params![assistant_id],
        )?;
        Ok(())
    }
}
