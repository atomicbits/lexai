use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assistant {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub model: String,
    pub temperature: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Assistant {
    pub fn new(name: String, description: Option<String>, model: String, temperature: Option<f64>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            model,
            temperature,
            created_at: now,
            updated_at: now,
        }
    }
}
