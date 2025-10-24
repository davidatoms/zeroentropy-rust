# ZeroEntropy Rust SDK

[![Crates.io](https://img.shields.io/crates/v/zeroentropy-community.svg)](https://crates.io/crates/zeroentropy-community)
[![Documentation](https://docs.rs/zeroentropy-community/badge.svg)](https://docs.rs/zeroentropy-community)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Rust client library for the [ZeroEntropy API](https://zeroentropy.dev) - a powerful semantic search and document retrieval service.

## Features

This library provides a complete implementation of all ZeroEntropy API endpoints. It uses strong typing with comprehensive error handling and is built on Tokio for async operations. The client includes configurable retry logic with exponential backoff and provides an ergonomic API with builder patterns.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zeroentropy-community = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use zeroentropy::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from ZEROENTROPY_API_KEY environment variable
    let client = Client::from_env()?;

    // Create a collection
    client.collections().add("my_collection").await?;

    // Add a document
    client.documents().add_text(
        "my_collection",
        "doc1.txt",
        "Rust is a systems programming language.",
        None,
    ).await?;

    // Search documents
    let results = client.queries().top_snippets(
        "my_collection",
        "systems programming",
        10,
        None,
        None,
        None,
        None,
    ).await?;

    for result in results.results {
        println!("{}: {}", result.path, result.content);
    }

    Ok(())
}
```

## Configuration

### Environment Variables

- `ZEROENTROPY_API_KEY` - Your API key (required)
- `ZEROENTROPY_BASE_URL` - Custom API base URL (optional)

### Client Builder

For advanced configuration, use the client builder:

```rust
use zeroentropy::Client;
use std::time::Duration;

let client = Client::builder()
    .api_key("your-api-key")
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .build()?;
```

## Usage Examples

### Collections

```rust
// Create a collection
client.collections().add("my_collection").await?;

// List all collections
let collections = client.collections().get_list().await?;
for name in collections.collections {
    println!("Collection: {}", name);
}

// Delete a collection
client.collections().delete("my_collection").await?;
```

### Documents

#### Adding Text Documents

```rust
client.documents().add_text(
    "my_collection",
    "document.txt",
    "Your document text here",
    None,
).await?;
```

#### Adding Documents with Metadata

```rust
use std::collections::HashMap;
use zeroentropy::MetadataValue;

let mut metadata = HashMap::new();
metadata.insert(
    "category".to_string(),
    MetadataValue::String("tutorial".to_string()),
);

client.documents().add_text(
    "my_collection",
    "tutorial.txt",
    "Tutorial content",
    Some(metadata),
).await?;
```

#### Adding PDF Documents

```rust
// From base64 encoded data
client.documents().add_pdf(
    "my_collection",
    "document.pdf",
    base64_pdf_data,
    None,
).await?;

// Directly from file
client.documents().add_pdf_file(
    "my_collection",
    "document.pdf",
    "/path/to/file.pdf",
    None,
).await?;
```

#### Managing Documents

```rust
// Get document info
let info = client.documents().get_info(
    "my_collection",
    "document.txt",
    Some(true), // include content
).await?;

// Update document metadata
client.documents().update(
    "my_collection",
    "document.txt",
    Some(new_metadata),
    None,
).await?;

// Delete document
client.documents().delete(
    "my_collection",
    "document.txt",
).await?;
```

### Queries

#### Top Documents

```rust
let results = client.queries().top_documents(
    "my_collection",
    "your search query",
    10, // number of results
    None, // filter
    Some(true), // include metadata
    None, // latency mode
    None, // reranker
).await?;

for doc in results.results {
    println!("{}: score {}", doc.path, doc.score);
}
```

#### Top Snippets

```rust
let results = client.queries().top_snippets(
    "my_collection",
    "your search query",
    10,
    None, // filter
    Some(true), // include document metadata
    Some(true), // precise responses (longer snippets)
    None, // reranker
).await?;

for snippet in results.results {
    println!("{}:\n{}\n", snippet.path, snippet.content);
}
```

#### Top Pages

```rust
let results = client.queries().top_pages(
    "my_collection",
    "your search query",
    10,
    None, // filter
    Some(true), // include content
    None, // latency mode
).await?;

for page in results.results {
    println!("Page {} of {}", page.page_number, page.path);
}
```

### Filtering

Use metadata filters to narrow down search results:

```rust
use serde_json::json;

let filter = json!({
    "category": { "$eq": "tutorial" }
}).as_object().unwrap().clone();

let results = client.queries().top_snippets(
    "my_collection",
    "search query",
    10,
    Some(filter),
    None,
    None,
    None,
).await?;
```

### Reranking

Improve search result quality with reranking:

```rust
use zeroentropy::RerankDocument;

let documents = vec![
    RerankDocument {
        id: "doc1".to_string(),
        text: "First document text".to_string(),
    },
    RerankDocument {
        id: "doc2".to_string(),
        text: "Second document text".to_string(),
    },
];

let results = client.models().rerank(
    "your query",
    documents,
    None, // model_id (uses default)
    Some(5), // top_k
).await?;

for result in results.results {
    println!("{}: score {}", result.id, result.score);
}
```

## Error Handling

The SDK provides specific error types for different failure scenarios:

```rust
use zeroentropy::Error;

match client.collections().add("my_collection").await {
    Ok(_) => println!("Success!"),
    Err(Error::Conflict(msg)) => println!("Already exists: {}", msg),
    Err(Error::NotFound(msg)) => println!("Not found: {}", msg),
    Err(Error::AuthenticationError(msg)) => println!("Auth failed: {}", msg),
    Err(Error::RateLimitExceeded(msg)) => println!("Rate limited: {}", msg),
    Err(e) => println!("Error: {}", e),
}
```

## Examples

Check out the [examples](examples/) directory for more complete examples:

- [basic.rs](examples/basic.rs) - Complete workflow from collection creation to search
- [arxiv_search.rs](examples/arxiv_search.rs) - Download and search arXiv papers with PDF support

Run an example:

```bash
export ZEROENTROPY_API_KEY="your-api-key"
cargo run --example basic
cargo run --example arxiv_search
```

## API Documentation

For detailed API documentation, visit:
- [ZeroEntropy API Docs](https://docs.zeroentropy.dev/api-reference/)
- [Rust SDK Docs](https://docs.rs/zeroentropy-community)

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
export ZEROENTROPY_API_KEY="your-api-key"
cargo run --example basic
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [docs.zeroentropy.dev](https://docs.zeroentropy.dev)
- **Email**: founders@zeroentropy.dev
- **Issues**: [GitHub Issues](https://github.com/zeroentropy-ai/zeroentropy-rust/issues)

## Related Projects

- [Python SDK](https://github.com/zeroentropy-ai/zeroentropy-python)
- [ZeroEntropy Cookbook](https://github.com/zeroentropy-ai/zcookbook)
