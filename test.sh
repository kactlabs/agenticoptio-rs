#!/bin/bash

# Test script for agentic_optio_rs library

set -e

echo "🧪 Testing AgenticOptioRS Rust Library"
echo "===================================="
echo ""

# Check if Ollama is running
echo "📡 Checking Ollama connection..."
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "✅ Ollama is running"
else
    echo "❌ Ollama is not running. Please start Ollama first:"
    echo "   brew services start ollama  # or just run 'ollama serve'"
    exit 1
fi

echo ""
echo "🔍 Checking for required models..."
if ollama list | grep -q "llama3.2"; then
    echo "✅ llama3.2 model found"
else
    echo "⚠️  llama3.2 not found. Pulling..."
    ollama pull llama3.2
fi

if ollama list | grep -q "nomic-embed-text"; then
    echo "✅ nomic-embed-text model found"
else
    echo "⚠️  nomic-embed-text not found. Pulling..."
    ollama pull nomic-embed-text
fi

echo ""
echo "🔨 Building library..."
cargo build

echo ""
echo "📝 Running clippy..."
cargo clippy -- -D warnings

echo ""
echo "🎨 Checking formatting..."
cargo fmt -- --check

echo ""
echo "📚 Building documentation..."
cargo doc --no-deps

echo ""
echo "🧪 Running unit tests..."
cargo test --lib

echo ""
echo "🚀 Running examples..."
echo ""
echo "--- Basic Usage Example ---"
cargo run --example basic_usage

echo ""
echo "--- Embeddings Test ---"
cargo run --example test_embeddings

echo ""
echo "✅ All tests passed!"
echo ""
echo "To run integration tests (requires Ollama):"
echo "  cargo test -- --ignored"
echo ""
echo "To run streaming example:"
echo "  cargo run --example streaming_example"
