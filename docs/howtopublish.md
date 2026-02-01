# How to Publish AgenticOptioRS to Crates.io

This guide walks you through publishing the AgenticOptioRS Rust library to crates.io.

## Prerequisites

### 1. Create a Crates.io Account

1. Go to [crates.io](https://crates.io/)
2. Sign in with your GitHub account
3. Go to Account Settings → API Tokens
4. Generate a new API token
5. Save the token securely

### 2. Login to Cargo

```bash
cargo login <your-api-token>
```

This stores your credentials in `~/.cargo/credentials.toml`

## Pre-Publication Checklist

### 1. Verify Package Metadata

Check `Cargo.toml` has all required fields:

```toml
[package]
name = "agentic_optio_rs"
version = "0.1.1"
edition = "2021"
authors = ["Raja CSP Raman <raja.csp@gmail.com>"]
description = "Production-grade multi-agent framework with minimal dependencies - Rust implementation"
license = "MIT"
repository = "https://github.com/kactlabs/agenticoptio-rs"
documentation = "https://docs.rs/agentic_optio_rs"
keywords = ["ai", "llm", "chat", "embeddings", "ollama"]
categories = ["api-bindings", "asynchronous"]
readme = "README.md"
```

### 2. Run All Tests

```bash
# Run the comprehensive test script
./test.sh

# Or run tests manually:
cargo test
cargo test -- --ignored  # Integration tests (requires Ollama)
cargo clippy -- -D warnings
cargo fmt --check
```

### 3. Build Documentation

```bash
cargo doc --no-deps --open
```

Review the generated documentation to ensure everything looks correct.

### 4. Check Package Contents

```bash
cargo package --list
```

This shows what files will be included in the published package.

### 5. Verify Package Builds

```bash
cargo package --allow-dirty
```

This creates a `.crate` file in `target/package/` without publishing.

### 6. Test the Package Locally

```bash
# Extract and test the packaged crate
cd target/package
tar xvf agentic_optio_rs-0.1.1.crate
cd agentic_optio_rs-0.1.1
cargo test
```

## Publishing Steps

### 1. Dry Run (Recommended)

```bash
cargo publish --dry-run
```

This simulates publishing without actually uploading. Review the output carefully.

### 2. Publish to Crates.io

```bash
cargo publish
```

If successful, you'll see:
```
Uploading agentic_optio_rs v0.1.1 (...)
```

### 3. Verify Publication

1. Visit https://crates.io/crates/agentic_optio_rs
2. Check that the version appears
3. Verify documentation at https://docs.rs/agentic_optio_rs

## Post-Publication

### 1. Tag the Release in Git

```bash
git tag -a v0.1.1 -m "Release version 0.1.1"
git push origin v0.1.1
```

### 2. Create GitHub Release

1. Go to your GitHub repository
2. Click "Releases" → "Create a new release"
3. Select the tag you just created
4. Add release notes
5. Publish the release

### 3. Update Documentation

Update the README.md to reflect the published version:

```markdown
## Installation

Add to your `Cargo.toml`:

\`\`\`toml
[dependencies]
agentic_optio_rs = "0.1.1"
tokio = { version = "1", features = ["full"] }
\`\`\`
```

## Publishing Updates

### Patch Version (0.1.1 → 0.1.2)

For bug fixes:

```bash
# Update version in Cargo.toml
# version = "0.1.2"

cargo test
cargo publish
git tag -a v0.1.2 -m "Release version 0.1.2"
git push origin v0.1.2
```

### Minor Version (0.1.x → 0.2.0)

For new features (backward compatible):

```bash
# Update version in Cargo.toml
# version = "0.2.0"

cargo test
cargo publish
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

### Major Version (0.x.x → 1.0.0)

For breaking changes:

```bash
# Update version in Cargo.toml
# version = "1.0.0"

# Update CHANGELOG.md with breaking changes
cargo test
cargo publish
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

## Yanking a Release

If you need to remove a published version:

```bash
# Yank a specific version (prevents new projects from using it)
cargo yank --vers 0.1.1

# Un-yank if needed
cargo yank --vers 0.1.1 --undo
```

**Note:** Yanking doesn't delete the version, it just prevents new projects from depending on it.

## Troubleshooting

### Error: "crate name is already taken"

The crate name `agentic_optio_rs` must be unique on crates.io. If taken, choose a different name.

### Error: "failed to verify package tarball"

Run `cargo package` and check the output for issues. Common causes:
- Missing files in the package
- Path dependencies that aren't published
- Build failures

### Error: "authentication required"

Run `cargo login` again with a valid API token.

### Documentation Not Building

Check that all doc comments are valid:

```bash
cargo doc --no-deps
```

Fix any warnings or errors before publishing.

## Best Practices

1. **Semantic Versioning**: Follow [SemVer](https://semver.org/)
   - MAJOR: Breaking changes
   - MINOR: New features (backward compatible)
   - PATCH: Bug fixes

2. **Changelog**: Maintain a CHANGELOG.md file

3. **Testing**: Always run full test suite before publishing

4. **Documentation**: Keep README.md and doc comments up to date

5. **Dependencies**: Minimize dependencies and keep them updated

6. **Examples**: Provide working examples for common use cases

7. **License**: Ensure LICENSE file is included

## Useful Commands

```bash
# Check current version
cargo pkgid

# Search for your package
cargo search agentic_optio_rs

# View package info
cargo info agentic_optio_rs

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated  # Requires cargo-outdated plugin
```

## Resources

- [Crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Semantic Versioning](https://semver.org/)

## Support

If you encounter issues:
1. Check [Cargo documentation](https://doc.rust-lang.org/cargo/)
2. Ask on [Rust Users Forum](https://users.rust-lang.org/)
3. File an issue on the [GitHub repository](https://github.com/kactlabs/agenticoptio-rs)
