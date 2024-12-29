use fastembed::{TextEmbedding, InitOptions};
use crate::prelude::*;

use super::{Embeddings, Embedder};


pub struct FastEmbedEmbedder {

    pub model: TextEmbedding,
    pub model_name: fastembed::EmbeddingModel,

}

impl FastEmbedEmbedder {
    pub fn try_new(model_name: fastembed::EmbeddingModel) -> Result<Self> {
        let mut options = InitOptions::default();
        options.model_name = model_name.clone();
        // fastembed::EmbeddingModel::AllMiniLML6V2
        let model = TextEmbedding::try_new(options)?; 
        Ok(Self { model, model_name })
    }
}

impl Embedder for FastEmbedEmbedder {

    fn dimensions(&self) -> Option<usize> {
        TextEmbedding::get_model_info(&self.model_name).ok().map(|m|  m.dim)
    }

    // fn get_embedding_and_usage(&self, text: &str) -> (Embeddings, Option<HashMap<String, String>>) {}

    fn embed<S: AsRef<str> + Send + Sync>(&self, texts: Vec<S>) -> Result<Vec<Embeddings>> {
        self.model.embed(texts, None)
    }

    fn query_embed<S: AsRef<str> + Send + Sync>(&self, query: S) -> Result<Embeddings>{
        let query = format!("query: {}.", query.as_ref());
        Ok(self.model.embed(vec![query], None)?[0].clone())
    }

}