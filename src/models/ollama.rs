//! Ollama models for AgenticOptio.
//!
//! Ollama runs LLMs locally. Supports Llama, Mistral, Qwen, and other models.

use crate::core::messages::{AIMessage, Message, ToolCall};
use crate::models::base::{BaseChatModel, BaseEmbedding, BoxStream, ModelError, ModelResult};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_HOST: &str = "http://localhost:11434";

/// OpenAI-compatible chat completion request
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<serde_json::Value>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// OpenAI-compatible chat completion response
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ResponseToolCall>,
}

#[derive(Debug, Deserialize)]
struct ResponseToolCall {
    id: String,
    function: FunctionCall,
}

#[derive(Debug, Deserialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}

/// Streaming chunk response
#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: Delta,
}

#[derive(Debug, Deserialize)]
struct Delta {
    content: Option<String>,
}

/// Embedding request
#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
}

/// Embedding response
#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

/// Ollama chat model
///
/// # Examples
///
/// ```no_run
/// use agentic_optio_rs::{OllamaChat, Message};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let llm = OllamaChat::new("llama3.2");
///     let messages = vec![Message::user("Hello!")];
///     let response = llm.invoke(&messages).await?;
///     println!("{}", response.content);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OllamaChat {
    model: String,
    host: String,
    temperature: f32,
    max_tokens: Option<u32>,
    #[allow(dead_code)]
    timeout: Duration,
    #[allow(dead_code)]
    max_retries: u32,
    client: Client,
}

impl OllamaChat {
    /// Create a new Ollama chat model
    pub fn new(model: impl Into<String>) -> Self {
        Self::builder(model).build()
    }

    /// Create a builder for configuring the model
    pub fn builder(model: impl Into<String>) -> OllamaChatBuilder {
        OllamaChatBuilder::new(model)
    }

    fn parse_response(response: ChatResponse) -> ModelResult<AIMessage> {
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| ModelError::InvalidResponse("No choices in response".to_string()))?;

        let message = choice.message;
        let content = message.content.unwrap_or_default();

        let tool_calls: Vec<ToolCall> = message
            .tool_calls
            .into_iter()
            .map(|tc| {
                let args = serde_json::from_str(&tc.function.arguments).unwrap_or_default();
                ToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    args,
                }
            })
            .collect();

        Ok(AIMessage::with_tool_calls(content, tool_calls))
    }
}

#[async_trait]
impl BaseChatModel for OllamaChat {
    async fn invoke(&self, messages: &[Message]) -> ModelResult<AIMessage> {
        let url = format!("{}/v1/chat/completions", self.host.trim_end_matches('/'));

        let messages_dict: Vec<serde_json::Value> = messages.iter().map(|m| m.to_dict()).collect();

        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages_dict,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            tools: None,
            stream: None,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<ChatResponse>()
            .await?;

        Self::parse_response(response)
    }

    async fn stream<'a>(
        &'a self,
        messages: &'a [Message],
    ) -> ModelResult<BoxStream<'a, ModelResult<AIMessage>>> {
        let url = format!("{}/v1/chat/completions", self.host.trim_end_matches('/'));

        let messages_dict: Vec<serde_json::Value> = messages.iter().map(|m| m.to_dict()).collect();

        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages_dict,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            tools: None,
            stream: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        use bytes::Bytes;
        use futures::stream::TryStreamExt;

        let stream = response
            .bytes_stream()
            .map_err(ModelError::HttpError)
            .and_then(|bytes: Bytes| async move {
                let text = String::from_utf8_lossy(&bytes);

                // Parse SSE format: "data: {...}\n\n"
                for line in text.lines() {
                    if let Some(json_str) = line.strip_prefix("data: ") {
                        if json_str == "[DONE]" {
                            continue;
                        }
                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(json_str) {
                            if let Some(choice) = chunk.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    return Ok(AIMessage::new(content.clone()));
                                }
                            }
                        }
                    }
                }

                Ok(AIMessage::new(""))
            });

        Ok(Box::pin(stream))
    }
}

/// Builder for OllamaChat
pub struct OllamaChatBuilder {
    model: String,
    host: String,
    temperature: f32,
    max_tokens: Option<u32>,
    timeout: Duration,
    max_retries: u32,
}

impl OllamaChatBuilder {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            host: std::env::var("OLLAMA_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string()),
            temperature: 0.0,
            max_tokens: None,
            timeout: Duration::from_secs(60),
            max_retries: 2,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn build(self) -> OllamaChat {
        let client = Client::builder()
            .timeout(self.timeout)
            .build()
            .expect("Failed to build HTTP client");

        OllamaChat {
            model: self.model,
            host: self.host,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            timeout: self.timeout,
            max_retries: self.max_retries,
            client,
        }
    }
}

/// Ollama embedding model
///
/// # Examples
///
/// ```no_run
/// use agentic_optio_rs::OllamaEmbedding;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let embedder = OllamaEmbedding::new("nomic-embed-text");
///     let texts = vec!["Hello".to_string(), "World".to_string()];
///     let embeddings = embedder.embed(&texts).await?;
///     println!("Generated {} embeddings", embeddings.len());
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OllamaEmbedding {
    model: String,
    host: String,
    #[allow(dead_code)]
    timeout: Duration,
    #[allow(dead_code)]
    max_retries: u32,
    batch_size: usize,
    client: Client,
}

impl OllamaEmbedding {
    /// Create a new Ollama embedding model
    pub fn new(model: impl Into<String>) -> Self {
        Self::builder(model).build()
    }

    /// Create a builder for configuring the model
    pub fn builder(model: impl Into<String>) -> OllamaEmbeddingBuilder {
        OllamaEmbeddingBuilder::new(model)
    }
}

#[async_trait]
impl BaseEmbedding for OllamaEmbedding {
    async fn embed(&self, texts: &[String]) -> ModelResult<Vec<Vec<f32>>> {
        let url = format!("{}/v1/embeddings", self.host.trim_end_matches('/'));

        let mut all_embeddings = Vec::new();

        for chunk in texts.chunks(self.batch_size) {
            let request = EmbeddingRequest {
                model: self.model.clone(),
                input: chunk.to_vec(),
            };

            let mut response = self
                .client
                .post(&url)
                .json(&request)
                .send()
                .await?
                .error_for_status()?
                .json::<EmbeddingResponse>()
                .await?;

            // Sort by index to maintain order
            response.data.sort_by_key(|d| d.index);

            for data in response.data {
                all_embeddings.push(data.embedding);
            }
        }

        Ok(all_embeddings)
    }
}

/// Builder for OllamaEmbedding
pub struct OllamaEmbeddingBuilder {
    model: String,
    host: String,
    timeout: Duration,
    max_retries: u32,
    batch_size: usize,
}

impl OllamaEmbeddingBuilder {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            host: std::env::var("OLLAMA_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string()),
            timeout: Duration::from_secs(60),
            max_retries: 2,
            batch_size: 100,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn build(self) -> OllamaEmbedding {
        let client = Client::builder()
            .timeout(self.timeout)
            .build()
            .expect("Failed to build HTTP client");

        OllamaEmbedding {
            model: self.model,
            host: self.host,
            timeout: self.timeout,
            max_retries: self.max_retries,
            batch_size: self.batch_size,
            client,
        }
    }
}
