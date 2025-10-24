use zeroentropy_community::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from ZEROENTROPY_API_KEY environment variable
    let client = Client::from_env()?;

    // Create a collection
    println!("Creating collection...");
    match client.collections().add("rust_example").await {
        Ok(response) => println!("{}", response.message),
        Err(zeroentropy_community::Error::Conflict(_)) => println!("Collection already exists"),
        Err(e) => return Err(e.into()),
    }

    // Add some text documents
    println!("\nAdding documents...");
    client.documents().add_text(
        "rust_example",
        "doc1.txt",
        "Rust is a systems programming language focused on safety and performance.",
        None,
    ).await?;

    client.documents().add_text(
        "rust_example",
        "doc2.txt",
        "The Rust compiler prevents many common programming errors at compile time.",
        None,
    ).await?;

    // Add a document with metadata
    let mut metadata = HashMap::new();
    metadata.insert(
        "category".to_string(),
        zeroentropy_community::MetadataValue::String("tutorial".to_string()),
    );
    
    client.documents().add_text(
        "rust_example",
        "doc3.txt",
        "Cargo is Rust's build system and package manager.",
        Some(metadata),
    ).await?;

    println!("Documents added successfully!");

    // Search for documents
    println!("\nSearching for 'performance'...");
    let results = client.queries().top_snippets(
        "rust_example",
        "performance",
        5,
        None,
        Some(true), // include metadata
        None,
        None,
    ).await?;

    println!("Found {} results:", results.results.len());
    for (i, result) in results.results.iter().enumerate() {
        println!("\n{}. {} (score: {:.4})", i + 1, result.path, result.score);
        println!("   Content: {}", result.content);
        if let Some(metadata) = &result.metadata {
            println!("   Metadata: {:?}", metadata);
        }
    }

    // List all documents in collection
    println!("\n\nListing all documents...");
    let doc_list = client.documents().get_info_list("rust_example", Some(10), None).await?;
    
    for doc in doc_list.documents {
        println!("- {} (status: {:?})", doc.path, doc.index_status);
    }

    // Clean up - delete collection
    println!("\nCleaning up...");
    client.collections().delete("rust_example").await?;
    println!("Collection deleted!");

    Ok(())
}
