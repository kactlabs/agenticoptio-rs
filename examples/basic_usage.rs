//! Basic usage example for AgenticOptioRS
//!
//! This example demonstrates simple chat interaction with Ollama.

use agentic_optio_rs::{BaseChatModel, Message, OllamaChat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AgenticOptioRS - Basic Usage Example\n");

    // Create chat model
    let llm = OllamaChat::new("llama3.2");

    // Send a message
    let messages = vec![Message::user("Hello! Tell me a short joke.")];

    println!("Sending message to Ollama...");
    let response = llm.invoke(&messages).await?;

    println!("\nResponse:");
    println!("{}", response.content);

    // Continue conversation
    let conversation = vec![
        Message::user("Hello! Tell me a short joke."),
        Message::AI(response),
        Message::user("That's funny! Tell me another one."),
    ];

    println!("\n\nSending follow-up message...");
    let response2 = llm.invoke(&conversation).await?;

    println!("\nResponse:");
    println!("{}", response2.content);

    Ok(())
}
