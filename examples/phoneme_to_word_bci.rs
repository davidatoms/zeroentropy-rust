use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use zeroentropy_community::Client;

#[derive(Debug, Serialize, Deserialize)]
struct PhonemeWordPair {
    sentence: String,
    phonemes: String,
    session_date: String,
    block_num: u32,
    trial_num: u32,
}

/// Extract phoneme and word pairs from the BCI dataset pickle file
/// Note: This is a placeholder - actual implementation would need Python interop or pickle parsing
async fn load_bci_data(_data_path: &str) -> Result<Vec<PhonemeWordPair>, Box<dyn Error>> {
    // For this demo, we'll create sample data based on the BCI dataset structure
    // In production, you'd parse the actual t15_copyTask.pkl file
    
    let sample_data = vec![
        PhonemeWordPair {
            sentence: "he said the decision to part ways was mutual".to_string(),
            phonemes: "HH IY S EH D DH AH D IH S IH ZH AH N T UW P AA R T W EY Z W AA Z M Y UW CH UW AH L".to_string(),
            session_date: "2023-08-13".to_string(),
            block_num: 8,
            trial_num: 1,
        },
        PhonemeWordPair {
            sentence: "in fact this morning when they were talking".to_string(),
            phonemes: "IH N F AE K T DH IH S M AO R N IH NG HW EH N DH EY W ER T AO K IH NG".to_string(),
            session_date: "2023-08-13".to_string(),
            block_num: 8,
            trial_num: 2,
        },
        PhonemeWordPair {
            sentence: "you can see the code at this point as well".to_string(),
            phonemes: "Y UW K AE N S IY DH AH K OW D AE T DH IH S P OY N T AE Z W EH L".to_string(),
            session_date: "2023-08-13".to_string(),
            block_num: 9,
            trial_num: 1,
        },
        PhonemeWordPair {
            sentence: "how does it keep the cost down".to_string(),
            phonemes: "HH AW D AH Z IH T K IY P DH AH K AO S T D AW N".to_string(),
            session_date: "2023-08-13".to_string(),
            block_num: 9,
            trial_num: 2,
        },
        PhonemeWordPair {
            sentence: "not too controversial".to_string(),
            phonemes: "N AA T T UW K AA N T R AH V ER SH AH L".to_string(),
            session_date: "2023-08-18".to_string(),
            block_num: 6,
            trial_num: 1,
        },
    ];
    
    Ok(sample_data)
}

/// Test zeroentropy's ability to match phonemes to words
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    println!("=== ZeroEntropy Phoneme-to-Word Matching Test ===\n");
    
    // Initialize ZeroEntropy client
    let client = Client::from_env()?;
    
    // Collection name for this experiment
    let collection_name = "bci_phoneme_to_word";
    
    // Check if collection exists, if so delete it for fresh start
    println!("Setting up collection '{}'...", collection_name);
    match client.collections().delete(collection_name).await {
        Ok(_) => println!("Deleted existing collection"),
        Err(_) => println!("No existing collection found"),
    }
    
    // Create new collection
    client.collections().add(collection_name).await?;
    println!("Created collection\n");
    
    // Load BCI data
    println!("Loading BCI dataset...");
    let data_path = "C:\\Users\\david\\OneDrive\\Research\\ArtificialIntelligence\\BrainComputerInterface\\nejm-brain-to-text\\data\\t15_copyTask.pkl";
    let phoneme_word_pairs = load_bci_data(data_path).await?;
    println!("Loaded {} phoneme-word pairs\n", phoneme_word_pairs.len());
    
    // Add documents to collection
    // Strategy: Store sentences as documents with phonemes as metadata
    println!("Adding documents to ZeroEntropy...");
    for (idx, pair) in phoneme_word_pairs.iter().enumerate() {
        let doc_id = format!("{}_{}_{}_{}", 
            pair.session_date, 
            pair.block_num, 
            pair.trial_num,
            idx
        );
        
        // Create metadata with phoneme sequence
        let mut metadata = HashMap::new();
        metadata.insert(
            "phonemes".to_string(), 
            zeroentropy_community::MetadataValue::String(pair.phonemes.clone())
        );
        metadata.insert(
            "session_date".to_string(),
            zeroentropy_community::MetadataValue::String(pair.session_date.clone())
        );
        
        // Add document with sentence as content
        client.documents().add_text(
            collection_name,
            &doc_id,
            &pair.sentence,
            Some(metadata),
        ).await?;
        
        print!(".");
        std::io::stdout().flush()?;
    }
    println!("\nAdded {} documents\n", phoneme_word_pairs.len());
    
    // Wait for indexing
    println!("Waiting for indexing to complete...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Test queries: Given phoneme sequences, can we retrieve the correct sentences?
    println!("\n=== Testing Phoneme-to-Word Retrieval ===\n");
    
    let test_cases = vec![
        (
            "HH IY S EH D",  // First few phonemes
            "he said the decision to part ways was mutual",
            "Query with partial phoneme sequence"
        ),
        (
            "K IY P DH AH K AO S T",  // Middle phonemes
            "how does it keep the cost down",
            "Query with middle phonemes"
        ),
        (
            "N AA T T UW K AA N T R AH V ER SH AH L",  // Full phoneme sequence
            "not too controversial",
            "Query with full phoneme sequence"
        ),
    ];
    
    let mut results_summary = Vec::new();
    
    for (query_phonemes, expected_sentence, description) in test_cases {
        println!("Test: {}", description);
        println!("Query phonemes: {}", query_phonemes);
        println!("Expected sentence: {}\n", expected_sentence);
        
        // Try searching with the phoneme sequence directly
        let results = client.queries().top_snippets(
            collection_name,
            query_phonemes,
            3,
            None,
            None,
            None,
            None,
        ).await?;
        
        println!("Results:");
        let mut found_match = false;
        for (idx, result) in results.results.iter().enumerate() {
            println!("  {}. Score: {:.4} - {}", idx + 1, result.score, result.content);
            if result.content.contains(expected_sentence) || expected_sentence.contains(&result.content) {
                found_match = true;
            }
        }
        
        results_summary.push((description, found_match));
        println!("Match found: {}\n", if found_match { "✓" } else { "✗" });
        println!("---\n");
    }
    
    // Also test: Can we search for sentences and retrieve phonemes?
    println!("=== Reverse Test: Word-to-Phoneme Retrieval ===\n");
    
    let word_query = "controversial";
    println!("Searching for word: '{}'", word_query);
    
    let results = client.queries().top_snippets(
        collection_name,
        word_query,
        1,
        None,
        None,
        None,
        None,
    ).await?;
    
    if let Some(result) = results.results.first() {
        println!("Found sentence: {}", result.content);
        if let Some(metadata) = &result.metadata {
            if let Some(zeroentropy_community::MetadataValue::String(phonemes)) = metadata.get("phonemes") {
                println!("Associated phonemes: {}", phonemes);
            }
        }
    }
    
    // Summary
    println!("\n=== Test Summary ===");
    println!("Total tests: {}", results_summary.len());
    let passed = results_summary.iter().filter(|(_, found)| *found).count();
    println!("Passed: {}/{}", passed, results_summary.len());
    
    for (description, found) in results_summary {
        println!("  {} - {}", if found { "✓" } else { "✗" }, description);
    }
    
    println!("\n=== Observations ===");
    println!("This test evaluates ZeroEntropy's ability to:");
    println!("1. Match phoneme sequences to word sequences");
    println!("2. Handle partial phoneme queries");
    println!("3. Retrieve associated metadata (phonemes from words)");
    println!("\nKey insight: ZeroEntropy uses semantic embeddings, not character/phoneme matching,");
    println!("so performance depends on whether phoneme patterns have semantic meaning in the");
    println!("embedding space trained on natural language.");
    
    Ok(())
}
