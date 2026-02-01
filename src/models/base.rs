//! Base traits for AgenticOptio models.
//!
//! Provides abstract base traits for chat and embedding model implementations.

use crate::core::messages::{AIMessage, Message};
use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;

/// Error type for model operations
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

pub type ModelResult<T> = Result<T, ModelError>;

/// Base trait for all chat models
#[async_trait]
pub trait BaseChatModel: Send + Sync {
    /// Invoke the model asynchronously
    async fn invoke(&self, messages: &[Message]) -> ModelResult<AIMessage>;

    /// Stream response asynchronously
    async fn stream<'a>(
        &'a self,
        messages: &'a [Message],
    ) -> ModelResult<BoxStream<'a, ModelResult<AIMessage>>>;
}

/// Base trait for all embedding models
#[async_trait]
pub trait BaseEmbedding: Send + Sync {
    /// Embed multiple texts asynchronously
    async fn embed(&self, texts: &[String]) -> ModelResult<Vec<Vec<f32>>>;

    /// Embed a single query text
    async fn embed_query(&self, text: &str) -> ModelResult<Vec<f32>> {
        let result = self.embed(&[text.to_string()]).await?;
        result
            .into_iter()
            .next()
            .ok_or_else(|| ModelError::InvalidResponse("No embedding returned".to_string()))
    }

    /// Get embedding dimension
    fn dimension(&self) -> usize {
        1536 // Default to OpenAI dimension
    }
}
