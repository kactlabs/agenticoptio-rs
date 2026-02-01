//! Models for AgenticOptio.
//!
//! This module contains all model implementations and base classes.

pub mod base;
pub mod ollama;

pub use base::{BaseChatModel, BaseEmbedding};
pub use ollama::{OllamaChat, OllamaEmbedding};
