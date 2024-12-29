use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum StoreError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Table not found: {0}")]
    TableNotFound(String),
}

impl From<lance::Error> for StoreError {
    fn from(error: lance::Error) -> Self {
        StoreError::Database(error.to_string())
    }
}