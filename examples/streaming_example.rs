//! Streaming example for AgenticOptioRS
//!
//! This example demonstrates streaming responses from Ollama.

use agentic_optio_rs::{BaseChatModel, Message, OllamaChat};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AgenticOptioRS - Streaming Example\n");

    // Create chat model
    let llm = OllamaChat::new("llama3.2");

    // Send a message with streaming
    let messages = vec![Message::user("Tell me a short story about a robot.")];

    println!("Streaming response from Ollama...\n");

    let mut stream = llm.stream(&messages).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                if !chunk.content.is_empty() {
                    print!("{}", chunk.content);
                    use std::io::Write;
                    std::io::stdout().flush()?;
                }
            }
            Err(e) => {
                eprintln!("\nError: {}", e);
                break;
            }
        }
    }

    println!("\n\nStreaming complete!");

    Ok(())
}
