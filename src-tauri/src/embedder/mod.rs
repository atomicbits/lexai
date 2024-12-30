pub mod fastembed;

use ::fastembed::Embedding;

use crate::prelude::*;

pub trait Embedder {
    fn dimensions(&self) -> Option<usize>;
    // fn model_config(&self) -> HashMap<String, String>;
    // fn get_embedding(&self, text: &str) -> Embeddings;
    // fn get_embedding_and_usage(&self, text: &str) -> (Embeddings, Option<HashMap<String, String>>);

    fn embed<S: AsRef<str> + Send + Sync>(&self, texts: Vec<S>) -> Result<Vec<Embedding>>;
    fn query_embed<S: AsRef<str> + Send + Sync>(&self, query: S) -> Result<Embedding>;
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    // Calculate the dot product of the two vectors
    let dot_product = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum::<f32>();

    // Calculate the norm (magnitude) of vectors
    let norm_a = a.iter().map(|&x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|&x| x * x).sum::<f32>().sqrt();

    // Calculate the cosine similarity
    dot_product / (norm_a * norm_b)
}