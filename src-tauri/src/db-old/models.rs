use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub name: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub embedding: Vec<f32>,
}

