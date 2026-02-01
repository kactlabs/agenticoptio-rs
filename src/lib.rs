//! AgenticOptioRS - Disciplined AI agent coordination library.
//!
//! Named after the Roman Optio, this framework brings military-grade coordination,
//! resilience, and execution discipline to multi-model AI operations.
//!
//! Currently supports Ollama with OpenAI, Anthropic, and other providers coming soon.
//!
//! # Examples
//!
//! ```no_run
//! use agentic_optio_rs::{OllamaChat, Message};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let llm = OllamaChat::new("llama3.2");
//!     let messages = vec![Message::user("Hello!")];
//!     let response = llm.invoke(&messages).await?;
//!     println!("{}", response.content);
//!     Ok(())
//! }
//! ```

pub mod core;
pub mod models;

// Re-export main types
pub use core::messages::{
    AIMessage, BaseMessage, HumanMessage, Message, SystemMessage, ToolMessage,
};
pub use models::base::{BaseChatModel, BaseEmbedding};
pub use models::ollama::{OllamaChat, OllamaEmbedding};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
