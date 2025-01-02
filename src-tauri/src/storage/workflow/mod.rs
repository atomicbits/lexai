mod models;
pub use models::Workflow;

use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{params, Result};
use crate::storage::db::Database;

use rusqlite::OptionalExtension;

pub struct WorkflowStorage<'a> {
    db: &'a Database,
}

impl<'a> WorkflowStorage<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, workflow: &Workflow) -> Result<()> {
        self.db.get_connection().await.execute(
            "INSERT INTO workflows (id, name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                workflow.id,
                workflow.name,
                workflow.description,
                workflow.created_at.timestamp(),
                workflow.updated_at.timestamp(),
            ],
        )?;
        Ok(())
    }

    pub async fn get(&self, workflow_id: &str) -> Result<Option<Workflow>> {
        let conn = self.db.get_connection().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at 
             FROM workflows WHERE id = ?1",
        )?;

        let workflow = stmt.query_row(params![workflow_id], |row| {
            let created_at_ts: i64 = row.get(3)?;
            let updated_at_ts: i64 = row.get(4)?;
            
            Ok(Workflow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: Utc.timestamp_opt(created_at_ts, 0).unwrap(),
                updated_at: Utc.timestamp_opt(updated_at_ts, 0).unwrap(),
            })
        }).optional()?;

        Ok(workflow)
    }

    pub async fn list(&self) -> Result<Vec<Workflow>> {
        let conn = self.db.get_connection().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM workflows",
        )?;

        let workflows = stmt.query_map([], |row| {
            let created_at_ts: i64 = row.get(3)?;
            let updated_at_ts: i64 = row.get(4)?;
            
            Ok(Workflow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: Utc.timestamp_opt(created_at_ts, 0).unwrap(),
                updated_at: Utc.timestamp_opt(updated_at_ts, 0).unwrap(),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(workflows)
    }

    pub async fn update(&self, workflow: &Workflow) -> Result<()> {
        let now = Utc::now();
        self.db.get_connection().await.execute(
            "UPDATE workflows 
             SET name = ?1, description = ?2, updated_at = ?3
             WHERE id = ?4",
            params![
                workflow.name,
                workflow.description,
                now.timestamp(),
                workflow.id,
            ],
        )?;
        Ok(())
    }

    pub async fn delete(&self, workflow_id: &str) -> Result<()> {
        self.db.get_connection().await.execute(
            "DELETE FROM workflows WHERE id = ?1",
            params![workflow_id],
        )?;
        Ok(())
    }
}
