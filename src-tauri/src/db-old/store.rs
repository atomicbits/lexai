use std::sync::Mutex;
use std::collections::BTreeMap;  // Use BTreeMap instead of IndexMap
use lance::Dataset;
use lance::datatypes::{Schema, DataType};
use super::{Document, SearchResult, StoreError};

pub struct LanceStore {
    dataset: Mutex<Dataset>,
}

impl LanceStore {
    pub fn new() -> Result<Self, StoreError> {
        let dataset = Dataset::create_in_memory()?;
        Self::init_schema(&dataset)?;
        
        Ok(Self {
            dataset: Mutex::new(dataset),
        })
    }

    fn init_schema(dataset: &Dataset) -> Result<(), StoreError> {
        let schema = Arc::new(Schema::new(vec![
            
            Field::new("name", DataType::Utf8, true),
            Field::new("content", DataType::Utf8, true),
            Field::new("metadata", DataType::Utf8, true)
        ])
            .with_vector_field("embedding", 384) // Adjust dimension as needed
            .with_field("name", lance::datatypes::Field::new(lance::datatypes::, true))
            .with_field("content", lance::datatypes::Field::new(lance::datatypes::Utf8, true))
            .with_field("metadata", lance::datatypes::Field::new(lance::datatypes::Utf8, true)));

        dataset
            .create_table("documents", schema)
            .map_err(StoreError::from)?;

        Ok(())
    }

    pub fn insert(&self, document: Document) -> Result<(), StoreError> {
        let dataset = self.dataset.lock().unwrap();
        let table = dataset.open_table("documents")?;

        let data = vec![(
            document.embedding,
            document.name,
            document.content,
            serde_json::to_string(&document.metadata).map_err(|e| StoreError::InvalidData(e.to_string()))?,
        )];

        table.add(data)?;
        Ok(())
    }

    pub fn search(&self, query_vector: Vec<f32>, limit: usize) -> Result<Vec<SearchResult>, StoreError> {
        let dataset = self.dataset.lock().unwrap();
        let table = dataset.open_table("documents")?;

        let results = table
            .search(query_vector)
            .limit(limit)
            .execute()?;

        let search_results = results
            .iter()
            .map(|row| {
                let metadata: Option<serde_json::Value> = row
                    .get("metadata")
                    .and_then(|m| serde_json::from_str(m).ok());

                SearchResult {
                    name: row.get("name").unwrap_or_default(),
                    content: row.get("content").unwrap_or_default(),
                    metadata,
                    score: row.score(),
                }
            })
            .collect();

        Ok(search_results)
    }
}
