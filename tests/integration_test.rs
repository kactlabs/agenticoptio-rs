//! Integration tests for agentic_optio_rs
//!
//! Note: These tests require Ollama to be running with the appropriate models.
//! Run: ollama pull llama3.2 && ollama pull nomic-embed-text

use agentic_optio_rs::{Message, OllamaChat, OllamaEmbedding};

#[tokio::test]
#[ignore] // Ignore by default since it requires Ollama running
async fn test_ollama_chat_invoke() {
    let llm = OllamaChat::new("llama3.2");
    let messages = vec![Message::user("Say 'test passed' and nothing else")];

    let result = llm.invoke(&messages).await;
    assert!(result.is_ok(), "Chat invoke should succeed");

    let response = result.unwrap();
    assert!(!response.content.is_empty(), "Response should not be empty");
}

#[tokio::test]
#[ignore]
async fn test_ollama_chat_builder() {
    let llm = OllamaChat::builder("llama3.2")
        .temperature(0.7)
        .max_tokens(100)
        .build();

    let messages = vec![Message::user("Hello")];
    let result = llm.invoke(&messages).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_ollama_embedding() {
    let embedder = OllamaEmbedding::new("nomic-embed-text");
    let texts = vec!["Hello world".to_string(), "Test embedding".to_string()];

    let result = embedder.embed(&texts).await;
    assert!(result.is_ok(), "Embedding should succeed");

    let embeddings = result.unwrap();
    assert_eq!(embeddings.len(), 2, "Should return 2 embeddings");
    assert!(!embeddings[0].is_empty(), "Embeddings should not be empty");
}

#[tokio::test]
#[ignore]
async fn test_ollama_embedding_query() {
    let embedder = OllamaEmbedding::new("nomic-embed-text");

    let result = embedder.embed_query("Single query test").await;
    assert!(result.is_ok(), "Single query embedding should succeed");

    let embedding = result.unwrap();
    assert!(!embedding.is_empty(), "Embedding should not be empty");
}

#[tokio::test]
#[ignore]
async fn test_message_types() {
    let system_msg = Message::system("You are helpful");
    assert_eq!(system_msg.role(), "system");

    let user_msg = Message::user("Hello");
    assert_eq!(user_msg.role(), "user");

    let assistant_msg = Message::assistant("Hi there");
    assert_eq!(assistant_msg.role(), "assistant");
}

#[test]
fn test_message_serialization() {
    let msg = Message::user("Test message");
    let dict = msg.to_dict();

    assert_eq!(dict["role"], "user");
    assert_eq!(dict["content"], "Test message");
}
