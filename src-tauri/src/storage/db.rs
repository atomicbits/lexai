
use rusqlite::{Connection, Error};
use std::path::PathBuf;
use tokio::sync::{Mutex, MutexGuard};


pub struct Database {
    conn: Mutex<Connection>,
    path:PathBuf,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self, Error> {
        Ok(Self { 
            conn: Mutex::new(Connection::open(db_path.clone())?), 
            path: db_path
        })
    }

    pub async fn init(&self) -> Result<(), Error> {
        let guard = self.get_connection().await;
        let conn: &Connection = &*guard;  // Deref the guard to get a reference
        self.init_internal(conn) // other_function must take &Connection
    }

    pub fn init_internal(&self, conn: &Connection) -> Result<(), Error> {
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                model TEXT NOT NULL,
                temperature REAL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS assistants (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                model TEXT NOT NULL,
                temperature REAL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub async fn get_connection(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().await
    }
}
