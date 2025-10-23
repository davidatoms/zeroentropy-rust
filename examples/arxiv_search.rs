use zeroentropy::{Client, MetadataValue};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Simplified example of downloading and searching arXiv papers
/// 
/// Usage:
///   export ZEROENTROPY_API_KEY="your-api-key"
///   cargo run --example arxiv_search
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    let collection = "arxiv_demo";

    // Create collection
    println!("\n=== Creating Collection ===");
    match client.collections().add(collection).await {
        Ok(_) => println!("Collection '{}' created", collection),
        Err(zeroentropy::Error::Conflict(_)) => {
            println!("Collection '{}' already exists", collection)
        }
        Err(e) => return Err(e.into()),
    }

    // Download a classic paper: "Attention Is All You Need"
    let arxiv_id = "1706.03762";
    let pdf_url = format!("https://arxiv.org/pdf/{}.pdf", arxiv_id);
    
    println!("\n=== Downloading Paper ===");
    println!("Paper: Attention Is All You Need ({})", arxiv_id);
    println!("Downloading from: {}", pdf_url);

    let http_client = reqwest::Client::new();
    let pdf_bytes = http_client
        .get(&pdf_url)
        .send()
        .await?
        .bytes()
        .await?;
    
    // Save PDF temporarily
    let pdf_path = format!("/tmp/arxiv_{}.pdf", arxiv_id);
    std::fs::write(&pdf_path, pdf_bytes)?;
    println!("Downloaded to: {}", pdf_path);

    // Index the paper with metadata
    println!("\n=== Indexing Paper ===");
    let mut metadata = HashMap::new();
    metadata.insert(
        "title".to_string(),
        MetadataValue::String("Attention Is All You Need".to_string()),
    );
    metadata.insert(
        "authors".to_string(),
        MetadataValue::String("Vaswani et al.".to_string()),
    );
    metadata.insert(
        "arxiv_id".to_string(),
        MetadataValue::String(arxiv_id.to_string()),
    );
    metadata.insert(
        "published".to_string(),
        MetadataValue::String("2017-06-12".to_string()),
    );

    match client
        .documents()
        .add_pdf_file(
            collection,
            &format!("arxiv_{}.pdf", arxiv_id),
            &pdf_path,
            Some(metadata),
        )
        .await
    {
        Ok(_) => {
            println!("Paper indexed successfully");
            // Wait a moment for the document to be fully processed
            println!("Waiting for document processing...");
            sleep(Duration::from_secs(3)).await;
        }
        Err(zeroentropy::Error::Conflict(_)) => {
            println!("Paper already indexed, using existing version");
        }
        Err(e) => return Err(e.into()),
    }

    // Search the paper
    println!("\n=== Searching Paper ===");
    let query = "How does multi-head attention work?";
    println!("Query: \"{}\"", query);

    let results = client
        .queries()
        .top_snippets(
            collection,
            query,
            5, // top 5 results
            None, // no filter
            Some(true), // include metadata
            Some(true), // precise responses
            None, // default reranker
        )
        .await?;

    println!("\nFound {} results:\n", results.results.len());

    for (i, result) in results.results.iter().enumerate() {
        println!("{}. Score: {:.4}", i + 1, result.score);
        println!("   {}\n", result.content.trim());
        println!("{}", "─".repeat(80));
    }

    // Clean up
    std::fs::remove_file(&pdf_path)?;
    println!("\nCleaned up temporary PDF");

    Ok(())
}
