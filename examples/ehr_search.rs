use zeroentropy_community::{Client, MetadataValue};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Example demonstrating semantic search over Electronic Health Records (EHR)
/// 
/// This example uses the Medical Transcriptions dataset from Kaggle
/// (https://www.kaggle.com/datasets/tboyle10/medicaltranscriptions)
/// containing ~5000 de-identified medical transcriptions across various specialties.
///
/// Usage:
///   1. Download the dataset:
///      wget -O /tmp/medical_transcriptions.zip "https://www.kaggle.com/api/v1/datasets/download/tboyle10/medicaltranscriptions"
///      unzip /tmp/medical_transcriptions.zip -d /tmp
///   
///   2. Set your API key and run:
///      export ZEROENTROPY_API_KEY="your-api-key"
///      cargo run --example ehr_search
///
/// This demonstrates how ZeroEntropy enables semantic search across medical records,
/// finding relevant information by meaning rather than exact keyword matches.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    let collection = "medical_transcriptions";

    println!("\n=== ZeroEntropy EHR Search Demo ===\n");

    // Create collection
    println!("Creating collection '{}'...", collection);
    match client.collections().add(collection).await {
        Ok(_) => println!("✓ Collection created"),
        Err(zeroentropy_community::Error::Conflict(_)) => {
            println!("✓ Collection already exists")
        }
        Err(e) => return Err(e.into()),
    }

    // Check if we need to index documents
    let doc_list = client.documents().get_info_list(collection, Some(1), None).await?;
    
    if doc_list.documents.is_empty() {
        println!("\n=== Indexing Medical Transcriptions ===");
        println!("Reading CSV from /tmp/mtsamples.csv...");
        
        // Read and parse CSV
        let csv_content = std::fs::read_to_string("/tmp/mtsamples.csv")?;
        let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
        
        let mut count = 0;
        let max_docs = 100; // Index first 100 for demo (adjust as needed)
        
        println!("Indexing first {} transcriptions...", max_docs);
        
        for (idx, result) in rdr.records().enumerate() {
            if count >= max_docs {
                break;
            }
            
            let record = result?;
            
            // CSV columns: description, medical_specialty, sample_name, transcription, keywords
            let description = record.get(0).unwrap_or("");
            let specialty = record.get(1).unwrap_or("");
            let sample_name = record.get(2).unwrap_or("");
            let transcription = record.get(3).unwrap_or("");
            let keywords = record.get(4).unwrap_or("");
            
            if transcription.is_empty() {
                continue;
            }
            
            // Create metadata
            let mut metadata = HashMap::new();
            metadata.insert(
                "specialty".to_string(),
                MetadataValue::String(specialty.to_string()),
            );
            metadata.insert(
                "description".to_string(),
                MetadataValue::String(description.to_string()),
            );
            if !keywords.is_empty() {
                metadata.insert(
                    "keywords".to_string(),
                    MetadataValue::String(keywords.to_string()),
                );
            }
            
            // Add document
            let doc_id = format!("record_{:04}", idx);
            match client
                .documents()
                .add_text(collection, &doc_id, transcription, Some(metadata))
                .await
            {
                Ok(_) => {
                    count += 1;
                    if count % 10 == 0 {
                        print!(".");
                        std::io::Write::flush(&mut std::io::stdout())?;
                    }
                }
                Err(zeroentropy_community::Error::Conflict(_)) => {
                    count += 1; // Already exists
                }
                Err(e) => eprintln!("\nWarning: Failed to index {}: {}", doc_id, e),
            }
        }
        
        println!("\n✓ Indexed {} medical transcriptions", count);
        println!("Waiting for indexing to complete...");
        sleep(Duration::from_secs(5)).await;
    } else {
        println!("✓ Collection already contains documents");
    }

    // Example clinical queries demonstrating semantic search capabilities
    let queries = vec![
        (
            "Patient with chest pain and shortness of breath",
            "Finding cardiovascular symptoms across different documentation styles"
        ),
        (
            "History of diabetes and kidney problems",
            "Finding related chronic conditions even with varied terminology"
        ),
        (
            "Postoperative complications and wound care",
            "Surgical follow-up documentation"
        ),
        (
            "Mental health assessment and depression screening",
            "Psychiatric and behavioral health notes"
        ),
        (
            "Imaging findings showing mass or lesion",
            "Radiology and pathology reports"
        ),
    ];

    println!("\n=== Clinical Query Examples ===\n");

    for (query, description) in queries {
        println!("Query: \"{}\"", query);
        println!("Use case: {}", description);
        
        let results = client
            .queries()
            .top_snippets(
                collection,
                query,
                3, // top 3 results
                None, // no filter
                Some(true), // include metadata
                Some(true), // precise responses for longer context
                None, // default reranker
            )
            .await?;

        if results.results.is_empty() {
            println!("  No results found\n");
            continue;
        }

        for (i, result) in results.results.iter().enumerate() {
            let specialty = result
                .metadata
                .as_ref()
                .and_then(|m| m.get("specialty"))
                .and_then(|v| match v {
                    MetadataValue::String(s) => Some(s.as_str()),
                    _ => None,
                })
                .unwrap_or("Unknown");

            println!("\n  {}. [{}] Score: {:.4}", i + 1, specialty, result.score);
            
            // Show snippet (truncate if too long)
            let snippet = if result.content.len() > 200 {
                format!("{}...", &result.content[..200])
            } else {
                result.content.clone()
            };
            println!("     {}", snippet.replace('\n', " "));
        }
        
        println!("\n{}", "─".repeat(80));
    }

    println!("\n=== Specialty Filtering Example ===\n");
    
    // Demonstrate metadata filtering
    println!("Query: 'patient assessment' filtered to Cardiology specialty");
    
    let filter = serde_json::json!({
        "specialty": { "$eq": "Cardiovascular / Pulmonary" }
    });
    
    let results = client
        .queries()
        .top_snippets(
            collection,
            "patient assessment",
            3,
            Some(filter.as_object().unwrap().clone()),
            Some(true),
            None,
            None,
        )
        .await?;

    println!("Found {} cardiology records:", results.results.len());
    for (i, result) in results.results.iter().enumerate() {
        println!("  {}. Score: {:.4}", i + 1, result.score);
        let snippet = &result.content[..result.content.len().min(150)];
        println!("     {}...", snippet.replace('\n', " "));
    }

    println!("\n=== Demo Complete ===");
    println!("\nKey capabilities demonstrated:");
    println!("  ✓ Semantic search - finds by meaning, not just keywords");
    println!("  ✓ Cross-specialty search - works across all medical domains");
    println!("  ✓ Metadata filtering - narrow searches by specialty, date, etc.");
    println!("  ✓ Ranked results - most relevant findings first");
    println!("\nPotential real-world applications:");
    println!("  • Clinical decision support - find similar cases");
    println!("  • Quality assurance - audit documentation patterns");
    println!("  • Research cohort building - identify eligible patients");
    println!("  • Prior authorization - find supporting documentation");
    println!("  • Medical-legal review - locate relevant encounters");

    Ok(())
}
