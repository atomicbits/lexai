use std::sync::Arc;

use lancedb::connection::Connection;
use lancedb::{connect, Table};
use lancedb::error::Error;
use tokio::sync::Mutex;
use arrow_schema::{DataType, Field, Schema};


pub struct DataStore {
    uri: String,
    db_connection: Mutex<Option<Connection>>,
}

impl DataStore {

    const TABLE_NAME: &'static str = "data_table";
    const EMBEDDING_SIZE: i32 = 64; // depend on the embedder, Python code: len(self.embedder.get_embedding("test"))

    pub fn new(uri: &str) -> DataStore {
        DataStore {
                uri: String::from(uri),
                db_connection: Mutex::new(None)
            }
    }

    pub async fn db(&self) -> Result<Connection, Error> {
        let mut conn_guard = self.db_connection.lock().await;

        if conn_guard.is_none() {
            *conn_guard = Some(connect(&self.uri).execute().await?);
        }
        Ok(conn_guard.as_ref().unwrap().clone()) // unwrap() cannot fail here
    }

    pub async fn create_empty_table(&self, table_name: &str, schema: Arc<Schema>) -> Result<Table, Error> {
        let db = &self.db().await?;
        db.create_empty_table(table_name, schema).execute().await
    }

    pub async fn create(&self) -> Result<(), Error> {
        if !self.exists().await? {
            self.init_table().await.map(|_| ())
        } else {
            Ok(())
        }
    }

    async fn init_table(&self) -> Result<Table, Error> {
        let db = &self.db().await?;
        let schema = Arc::new(
            Schema::new(
                vec![
                    Field::new(
                        "vector", 
                        DataType::FixedSizeList(
                            Arc::new(Field::new("item", DataType::Float32, false)), 
                            Self::EMBEDDING_SIZE
                        ),
                        false
                    ),
                    Field::new("id", DataType::Utf8, false),
                    Field::new("payload", DataType::Utf8, false),
                    ]
                )
            );
        db.create_empty_table(Self::TABLE_NAME, schema).execute().await
    }

    async fn exists(&self) -> Result<bool, Error> {
        let db = &self.db().await?;
        if let Ok(table_names) = db.table_names().execute().await {
            Ok(table_names.contains(&String::from(Self::TABLE_NAME)))
        } else {
            Ok(false)
        }
    }

}
