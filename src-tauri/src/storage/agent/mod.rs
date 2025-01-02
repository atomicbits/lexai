use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{params, Result, types::ToSql};
use crate::storage::db::Database;

use rusqlite::OptionalExtension;

pub mod models;
pub use models::Agent;


pub struct AgentStorage<'a> {
    db: &'a Database,
}

impl<'a> AgentStorage<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, agent: &Agent) -> Result<()> {
        self.db.get_connection().await.execute(
            "INSERT INTO agents (id, name, description, model, temperature, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                agent.id,
                agent.name,
                agent.description,
                agent.model,
                agent.temperature,
                agent.created_at.timestamp(),
                agent.updated_at.timestamp(),
            ],
        )?;
        Ok(())
    }

    pub async fn get(&self, agent_id: &str) -> Result<Option<Agent>> {
        let conn = self.db.get_connection().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, model, temperature, created_at, updated_at 
             FROM agents WHERE id = ?1",
        )?;

        let agent = stmt.query_row(params![agent_id], |row| {
            let created_at_ts: i64 = row.get(5)?;
            let updated_at_ts: i64 = row.get(6)?;
            
            Ok(Agent {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                model: row.get(3)?,
                temperature: row.get(4)?,
                created_at: Utc.timestamp_opt(created_at_ts, 0).unwrap(),
                updated_at: Utc.timestamp_opt(updated_at_ts, 0).unwrap(),
            })
        }).optional()?;

        Ok(agent)
    }

    pub async fn list(&self) -> Result<Vec<Agent>> {
        let conn = self.db.get_connection().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, model, temperature, created_at, updated_at FROM agents",
        )?;

        let agents = stmt.query_map([], |row| {
            let created_at_ts: i64 = row.get(5)?;
            let updated_at_ts: i64 = row.get(6)?;
            
            Ok(Agent {
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

        Ok(agents)
    }

    pub async fn update(&self, agent: &Agent) -> Result<()> {
        let now = Utc::now();
        self.db.get_connection().await.execute(
            "UPDATE agents 
             SET name = ?1, description = ?2, model = ?3, temperature = ?4, updated_at = ?5
             WHERE id = ?6",
            params![
                agent.name,
                agent.description,
                agent.model,
                agent.temperature,
                now.timestamp(),
                agent.id,
            ],
        )?;
        Ok(())
    }

    pub async fn delete(&self, agent_id: &str) -> Result<()> {
        self.db.get_connection().await.execute(
            "DELETE FROM agents WHERE id = ?1",
            params![agent_id],
        )?;
        Ok(())
    }
}
