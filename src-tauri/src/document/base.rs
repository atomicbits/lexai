use std::collections::HashMap;

use fastembed::Embedding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub content: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub meta_data: HashMap<String, String>, // Option<serde_json::Value>,
    pub embedder: Option<String>,
    pub embedding: Option<Embedding>,
    pub usage: Option<HashMap<String, String>>,
}
