# Testing AgenticOptioRS Locally

## Method 1: Run Examples (Quickest)

```bash
# Make sure Ollama is running
ollama pull llama3.2
ollama pull nomic-embed-text

# Run examples
cargo run --example basic_usage
cargo run --example streaming_example
cargo run --example test_embeddings
```

## Method 2: Create a Separate Test Project

### Step 1: Create a new test project outside this directory

```bash
cd ..
cargo new test-agentic-optio
cd test-agentic-optio
```

### Step 2: Add local dependency in `Cargo.toml`

```toml
[package]
name = "test-agentic-optio"
version = "0.1.0"
edition = "2021"

[dependencies]
# Point to your local library using relative path
agentic_optio_rs = { path = "../agenticoptio-rs" }
tokio = { version = "1", features = ["full"] }
```

### Step 3: Create test code in `src/main.rs`

```rust
use agentic_optio_rs::{OllamaChat, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing local agentic_optio_rs library\n");

    let llm = OllamaChat::new("llama3.2");
    let messages = vec![Message::user("Say hello!")];
    
    let response = llm.invoke(&messages).await?;
    println!("Response: {}", response.content);

    Ok(())
}
```

### Step 4: Run the test

```bash
cargo run
```

## Method 3: Use `cargo test` with Integration Tests

Create `tests/integration_test.rs` in your library:

```rust
use agentic_optio_rs::{OllamaChat, OllamaEmbedding, Message};

#[tokio::test]
async fn test_ollama_chat() {
    let llm = OllamaChat::new("llama3.2");
    let messages = vec![Message::user("Hello")];
    
    let result = llm.invoke(&messages).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ollama_embedding() {
    let embedder = OllamaEmbedding::new("nomic-embed-text");
    let texts = vec!["Hello".to_string()];
    
    let result = embedder.embed(&texts).await;
    assert!(result.is_ok());
}
```

Run with:
```bash
cargo test
```

## Method 4: Use in a Workspace

Create a workspace with your library and test projects:

```toml
# Cargo.toml at workspace root
[workspace]
members = ["agentic_optio", "test-app"]

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
```

Then in `test-app/Cargo.toml`:
```toml
[dependencies]
agentic_optio_rs = { path = "../agentic_optio_rs" }
tokio = { workspace = true }
```

## Method 5: Check Documentation Locally

```bash
# Build and open documentation
cargo doc --open

# Check for documentation issues
cargo doc --no-deps
```

## Method 6: Run Clippy and Format Checks

```bash
# Check for common mistakes
cargo clippy

# Format code
cargo fmt

# Check if code is formatted
cargo fmt -- --check
```

## Method 7: Build in Release Mode

```bash
# Build optimized version
cargo build --release

# Run examples in release mode
cargo run --release --example basic_usage
```

## Prerequisites

Before testing, ensure:
1. Ollama is installed and running
2. Required models are pulled:
   ```bash
   ollama pull llama3.2
   ollama pull nomic-embed-text
   ```

## Troubleshooting

If you get connection errors:
- Check Ollama is running: `ollama list`
- Verify host: `export OLLAMA_HOST=http://localhost:11434`
- Test Ollama directly: `curl http://localhost:11434/api/tags`
