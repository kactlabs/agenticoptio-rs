//! Test embeddings functionality

use agentic_optio_rs::{BaseEmbedding, OllamaEmbedding};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing OllamaEmbedding\n");

    let embedder = OllamaEmbedding::builder("nomic-embed-text")
        .batch_size(10)
        .build();

    // Test single embedding
    println!("Testing single embedding...");
    let embedding = embedder.embed_query("Hello world").await?;
    println!("✓ Single embedding dimension: {}", embedding.len());

    // Test batch embeddings
    println!("\nTesting batch embeddings...");
    let texts = vec![
        "Hello world".to_string(),
        "How are you?".to_string(),
        "Rust is awesome".to_string(),
    ];
    let embeddings = embedder.embed(&texts).await?;
    println!("✓ Generated {} embeddings", embeddings.len());
    println!("✓ Each embedding has {} dimensions", embeddings[0].len());

    println!("\n✅ All tests passed!");
    Ok(())
}
