# AgenticOptioRS

[![Crates.io](https://img.shields.io/crates/v/agentic_optio_rs.svg)](https://crates.io/crates/agentic_optio_rs)
[![Documentation](https://docs.rs/agentic_optio_rs/badge.svg)](https://docs.rs/agentic_optio_rs)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

*Command. Coordinate. Execute.*

AgenticOptioRS is a disciplined AI agent library built for reliable multi-model orchestration. Named after the Roman military **Optio**—the trusted second-in-command who coordinated formations, supervised operations, and stepped in as acting commander—this framework embodies the same principles of tactical coordination, operational resilience, and execution discipline.

This is the **Rust implementation** of AgenticOptio, providing high-performance, type-safe AI agent coordination.

> **Why "Optio"?** In Roman legions, an Optio was the backbone of military precision—responsible for coordination, training supervision, and maintaining formation integrity. They were the reliable officers who transformed strategic vision into flawless tactical execution. AgenticOptio brings this same operational excellence to AI agent coordination.

## Features

### Current (v0.1.0)
- **OllamaChat**: Chat with local Ollama models
- **OllamaEmbedding**: Generate embeddings using local Ollama models
- **Async Support**: Full async/await support using Tokio
- **Streaming**: Real-time streaming responses
- **Type Safety**: Rust's type system ensures correctness at compile time
- **Zero-cost Abstractions**: High performance with minimal overhead

### Coming Soon
- **OpenAI Integration**: GPT-4, GPT-3.5-turbo support
- **Anthropic Claude**: Claude 3.5 Sonnet and other models
- **Google Gemini**: Gemini Pro and Flash models
- **Groq**: Fast inference with Llama and Mixtral
- **Tool Support**: Function calling capabilities

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
agentic_optio_rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

**Requirements:** Rust 1.70+

## Prerequisites

### For Ollama (Current)
1. Install and run [Ollama](https://ollama.ai/)
2. Pull a model: `ollama pull llama3.2`

## Quick Start

### Basic Chat

```rust
use agentic_optio_rs::{OllamaChat, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create chat model
    let llm = OllamaChat::new("llama3.2");
    
    // Send a message
    let messages = vec![Message::user("Hello!")];
    let response = llm.invoke(&messages).await?;
    println!("{}", response.content);
    
    Ok(())
}
```

### Streaming Responses

```rust
use agentic_optio_rs::{OllamaChat, Message};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm = OllamaChat::new("llama3.2");
    let messages = vec![Message::user("Tell me a story")];
    
    let mut stream = llm.stream(&messages).await?;
    
    while let Some(result) = stream.next().await {
        if let Ok(chunk) = result {
            print!("{}", chunk.content);
        }
    }
    
    Ok(())
}
```

### Embeddings

```rust
use agentic_optio_rs::OllamaEmbedding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let embedder = OllamaEmbedding::new("nomic-embed-text");
    
    // Embed multiple texts
    let texts = vec!["Hello world".to_string(), "How are you?".to_string()];
    let embeddings = embedder.embed(&texts).await?;
    
    println!("Generated {} embeddings", embeddings.len());
    println!("Embedding dimension: {}", embeddings[0].len());
    
    Ok(())
}
```

## Configuration

### Custom Ollama Host

```rust
use agentic_optio_rs::OllamaChat;

let llm = OllamaChat::builder("llama3.2")
    .host("http://192.168.1.100:11434")
    .temperature(0.7)
    .max_tokens(1000)
    .build();
```

### Environment Variables

- `OLLAMA_HOST`: Default Ollama host URL (default: `http://localhost:11434`)

## API Reference

### OllamaChat

```rust
// Create with defaults
let llm = OllamaChat::new("llama3.2");

// Create with builder
let llm = OllamaChat::builder("llama3.2")
    .host("http://localhost:11434")
    .temperature(0.0)
    .max_tokens(1000)
    .timeout(Duration::from_secs(60))
    .max_retries(2)
    .build();

// Invoke
let response = llm.invoke(&messages).await?;

// Stream
let stream = llm.stream(&messages).await?;
```

### OllamaEmbedding

```rust
// Create with defaults
let embedder = OllamaEmbedding::new("nomic-embed-text");

// Create with builder
let embedder = OllamaEmbedding::builder("nomic-embed-text")
    .host("http://localhost:11434")
    .batch_size(100)
    .timeout(Duration::from_secs(60))
    .build();

// Embed multiple texts
let embeddings = embedder.embed(&texts).await?;

// Embed single query
let embedding = embedder.embed_query("Hello").await?;
```

## Examples

Run the examples:

```bash
# Basic usage
cargo run --example basic_usage

# Streaming
cargo run --example streaming_example
```

## Roadmap

### v0.2.0 - Strategic Alliance
- OpenAI GPT integration (GPT-4o, GPT-4o-mini, GPT-3.5-turbo)
- OpenAI embedding support
- Unified deployment protocols

### v0.3.0 - Allied Forces
- Anthropic Claude integration
- Google Gemini coordination
- Groq rapid response units

### v0.4.0 - Advanced Tactics
- Standardized tool/function calling protocols
- Enhanced streaming capabilities
- Batch processing for large-scale deployments
- Intelligent rate limiting and resilience patterns

### v1.0.0 - Battle Ready
- Full test coverage
- Comprehensive documentation
- Performance optimizations
- Production stability guarantees

## The Optio Advantage

In Roman legions, the **Optio** was the disciplined officer who transformed strategy into flawless execution. AgenticOptio brings this same operational excellence to AI:

### Command Structure
- **Unified Command**: Single interface governing all model providers
- **Type Safety**: Rust's type system prevents errors at compile time
- **Tactical Flexibility**: Adapt to any model or provider seamlessly

### Operational Discipline  
- **Reliability First**: Battle-tested patterns with comprehensive error handling
- **Resource Management**: Efficient coordination of compute and memory
- **Zero-cost Abstractions**: High performance without runtime overhead

### Strategic Readiness
- **Multi-Theater Operations**: Local models (Ollama) and cloud APIs in unified formation
- **Rapid Deployment**: Minimal dependencies for quick battlefield setup
- **Scalable Command**: From single agents to complex multi-agent orchestrations

## Comparison with Python Version

| Feature | Python | Rust |
|---------|--------|------|
| Type Safety | Runtime | Compile-time |
| Performance | Good | Excellent |
| Memory Safety | GC | Ownership |
| Async | asyncio | Tokio |
| Dependencies | Minimal | Minimal |
| Ease of Use | Very Easy | Easy |

## License

MIT License

## Links

- **Crates.io**: https://crates.io/crates/agentic_optio_rs
- **Documentation**: https://docs.rs/agentic_optio_rs
- **GitHub**: https://github.com/kactlabs/agenticoptio-rs
- **Python Version**: https://pypi.org/project/agenticoptio/
