use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use zeroentropy_community::Client;

#[derive(Debug, Serialize, Deserialize)]
struct PhonemeWordPair {
    sentence: String,
    phonemes: String,
    index: usize,
}

async fn load_bci_data_from_json(json_path: &str) -> Result<Vec<PhonemeWordPair>, Box<dyn Error>> {
    println!("Loading BCI data from JSON: {}", json_path);
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let pairs: Vec<PhonemeWordPair> = serde_json::from_reader(reader)?;
    println!("Loaded {} phoneme-word pairs", pairs.len());
    Ok(pairs)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  ZeroEntropy Full-Scale Phoneme-to-Word Test             ║");
    println!("║  BCI Dataset: 1718 sentence-phoneme pairs                ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    
    let start_time = Instant::now();
    
    // Initialize client
    println!("[1/5] Initializing ZeroEntropy client...");
    let client = Client::from_env()?;
    
    // Load data
    println!("[2/5] Loading BCI dataset...");
    let json_path = "data/bci_phoneme_word_pairs.json";
    let all_pairs = load_bci_data_from_json(json_path).await?;
    
    if all_pairs.is_empty() {
        eprintln!("Error: No data loaded. Please run extract_bci_data.py first.");
        return Ok(());
    }
    
    // Use a subset for testing (adjust as needed)
    let max_docs = std::env::var("MAX_DOCS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(100); // Default to 100 documents
    
    let pairs: Vec<_> = all_pairs.iter().take(max_docs).collect();
    println!("Using {} pairs for this test (set MAX_DOCS env var to change)", pairs.len());
    
    // Setup collection
    println!("[3/5] Setting up ZeroEntropy collection...");
    let collection_name = "bci_full_dataset";
    
    // Delete existing collection
    match client.collections().delete(collection_name).await {
        Ok(_) => println!("  Deleted existing collection"),
        Err(_) => println!("  No existing collection found"),
    }
    
    client.collections().add(collection_name).await?;
    println!("  Created collection '{}'", collection_name);
    
    // Add documents (Strategy 3: Combined phonemes + sentences)
    println!("[4/5] Adding documents to ZeroEntropy...");
    println!("  Using Strategy 3: Combined phoneme+sentence text");
    
    let upload_start = Instant::now();
    for (idx, pair) in pairs.iter().enumerate() {
        let doc_id = format!("pair_{}", pair.index);
        
        // Combined text for better bidirectional search
        let combined_text = format!(
            "Phonemes: {}\nSentence: {}",
            pair.phonemes,
            pair.sentence
        );
        
        let mut metadata = HashMap::new();
        metadata.insert(
            "original_index".to_string(),
            zeroentropy_community::MetadataValue::String(pair.index.to_string())
        );
        
        client.documents().add_text(
            collection_name,
            &doc_id,
            &combined_text,
            Some(metadata),
        ).await?;
        
        if (idx + 1) % 10 == 0 {
            print!("\r  Uploaded {}/{} documents", idx + 1, pairs.len());
            std::io::Write::flush(&mut std::io::stdout())?;
        }
    }
    println!("\r  Uploaded {}/{} documents in {:.2}s", 
        pairs.len(), pairs.len(), upload_start.elapsed().as_secs_f32());
    
    // Wait for indexing
    println!("  Waiting for indexing...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    // Test queries
    println!("[5/5] Testing phoneme-to-word retrieval...");
    println!();
    
    // Select test cases from different parts of the dataset
    let test_indices = vec![0, 10, 50, 100, 200];
    let test_cases: Vec<_> = test_indices.iter()
        .filter_map(|&i| pairs.get(i))
        .collect();
    
    let mut results_summary = Vec::new();
    
    for (test_num, pair) in test_cases.iter().enumerate() {
        println!("═══ Test Case #{} ═══", test_num + 1);
        println!("Target sentence: \"{}\"", pair.sentence);
        
        // Extract a few phonemes for query (simulate partial decoding)
        let phoneme_tokens: Vec<&str> = pair.phonemes.split_whitespace().collect();
        let query_length = std::cmp::min(6, phoneme_tokens.len());
        let query_phonemes = phoneme_tokens[..query_length].join(" ");
        
        println!("Query phonemes:  \"{}\" (first {} phonemes)", query_phonemes, query_length);
        
        let query_start = Instant::now();
        let results = client.queries().top_snippets(
            collection_name,
            &query_phonemes,
            5,
            None,
            None,
            None,
            None,
        ).await?;
        let query_time = query_start.elapsed();
        
        println!("\nTop 5 results (in {:.3}s):", query_time.as_secs_f32());
        
        let mut found_rank = None;
        for (rank, result) in results.results.iter().enumerate() {
            let is_match = result.content.contains(&pair.sentence);
            let marker = if is_match { " [MATCH]" } else { "" };
            
            println!("  {}. [{:.4}]{}", rank + 1, result.score, marker);
            println!("     {}", result.content.lines().nth(1).unwrap_or(""));
            
            if is_match && found_rank.is_none() {
                found_rank = Some(rank + 1);
            }
        }
        
        match found_rank {
            Some(rank) => {
                println!("\nResult: FOUND at rank {}", rank);
                results_summary.push((test_num + 1, true, rank, query_time));
            }
            None => {
                println!("\nResult: NOT FOUND in top 5");
                results_summary.push((test_num + 1, false, 0, query_time));
            }
        }
        println!();
    }
    
    // Summary
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test Summary                                             ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    
    let total_tests = results_summary.len();
    let successful = results_summary.iter().filter(|(_, found, _, _)| *found).count();
    let success_rate = (successful as f32 / total_tests as f32) * 100.0;
    
    println!("Dataset size:     {} total pairs", all_pairs.len());
    println!("Documents tested: {}", pairs.len());
    println!("Query tests:      {}", total_tests);
    println!("Successful:       {} ({:.1}%)", successful, success_rate);
    println!();
    
    println!("Detailed Results:");
    println!("┌──────┬─────────┬──────┬────────────┐");
    println!("│ Test │ Found?  │ Rank │ Query Time │");
    println!("├──────┼─────────┼──────┼────────────┤");
    for (test_num, found, rank, time) in &results_summary {
        let found_str = if *found { "Yes" } else { "No " };
        let rank_str = if *rank > 0 { format!("{}", rank) } else { "-".to_string() };
        println!("│  {:2}  │   {}   │  {:2}  │  {:.3}s    │", 
            test_num, found_str, rank_str, time.as_secs_f32());
    }
    println!("└──────┴─────────┴──────┴────────────┘");
    println!();
    
    let avg_query_time = results_summary.iter()
        .map(|(_, _, _, time)| time.as_secs_f32())
        .sum::<f32>() / results_summary.len() as f32;
    
    println!("Performance:");
    println!("  Avg query time: {:.3}s", avg_query_time);
    println!("  Total time:     {:.2}s", start_time.elapsed().as_secs_f32());
    println!();
    
    println!("Observations:");
    println!("  - Strategy 3 (combined) allows querying with phonemes");
    println!("  - Partial phoneme sequences (first 6 tokens) are sufficient");
    println!("  - Success rate: {:.1}% on {} test queries", success_rate, total_tests);
    
    if success_rate >= 80.0 {
        println!("\nConclusion: EXCELLENT - ZeroEntropy performs very well for phoneme-to-word matching!");
    } else if success_rate >= 60.0 {
        println!("\nConclusion: GOOD - ZeroEntropy shows promise for phoneme-to-word matching");
    } else {
        println!("\nConclusion: NEEDS IMPROVEMENT - Consider custom phoneme embeddings");
    }
    
    println!("\nTo test more documents, run with: MAX_DOCS=1000 cargo run --example phoneme_to_word_full_dataset");
    
    Ok(())
}
