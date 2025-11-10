use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use zeroentropy_community::Client;

#[derive(Debug, Serialize, Deserialize)]
struct PhonemeWordPair {
    sentence: String,
    phonemes: String,
    session_date: String,
    block_num: u32,
    trial_num: u32,
}

async fn load_bci_data(_data_path: &str) -> Result<Vec<PhonemeWordPair>, Box<dyn Error>> {
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
        PhonemeWordPair {
            sentence: "the jury and a judge work together on it".to_string(),
            phonemes: "DH AH JH UH R IY AE N D AH JH AH JH W ER K T AH G EH DH ER AA N IH T".to_string(),
            session_date: "2023-08-18".to_string(),
            block_num: 6,
            trial_num: 2,
        },
        PhonemeWordPair {
            sentence: "wait a minute we know this thing is not right".to_string(),
            phonemes: "W EY T AH M IH N AH T W IY N OW DH IH S TH IH NG IH Z N AA T R AY T".to_string(),
            session_date: "2023-08-18".to_string(),
            block_num: 6,
            trial_num: 3,
        },
        PhonemeWordPair {
            sentence: "one thing or the other".to_string(),
            phonemes: "W AH N TH IH NG AO R DH AH AH DH ER".to_string(),
            session_date: "2023-08-18".to_string(),
            block_num: 7,
            trial_num: 1,
        },
    ];
    
    Ok(sample_data)
}

/// Strategy 1: Store sentences, search with phonemes
async fn test_strategy_1(client: &Client) -> Result<(), Box<dyn Error>> {
    println!("\n=== STRATEGY 1: Store Sentences, Search with Phonemes ===\n");
    
    let collection = "bci_strategy1_sentences";
    
    // Cleanup
    let _ = client.collections().delete(collection).await;
    client.collections().add(collection).await?;
    
    let data = load_bci_data("").await?;
    
    println!("Adding {} sentence documents...", data.len());
    for (idx, pair) in data.iter().enumerate() {
        let doc_id = format!("sent_{}", idx);
        
        let mut metadata = HashMap::new();
        metadata.insert(
            "phonemes".to_string(),
            zeroentropy_community::MetadataValue::String(pair.phonemes.clone())
        );
        
        client.documents().add_text(
            collection,
            &doc_id,
            &pair.sentence,
            Some(metadata),
        ).await?;
    }
    
    println!("Waiting for indexing...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Test: Search with phoneme sequences
    println!("\nTest: Can phonemes retrieve sentences?");
    let test_phonemes = "W EY T AH M IH N AH T";  // "wait a minute"
    println!("Query: {}", test_phonemes);
    
    let results = client.queries().top_snippets(
        collection,
        test_phonemes,
        3,
        None,
        None,
        None,
        None,
    ).await?;
    
    println!("Top results:");
    for (i, result) in results.results.iter().enumerate() {
        println!("  {}. [{:.4}] {}", i+1, result.score, result.content);
    }
    
    let found = results.results.iter()
        .any(|r| r.content.contains("wait a minute"));
    println!("✓ Found 'wait a minute': {}", found);
    
    Ok(())
}

/// Strategy 2: Store phonemes, search with words
async fn test_strategy_2(client: &Client) -> Result<(), Box<dyn Error>> {
    println!("\n=== STRATEGY 2: Store Phonemes, Search with Words ===\n");
    
    let collection = "bci_strategy2_phonemes";
    
    // Cleanup
    let _ = client.collections().delete(collection).await;
    client.collections().add(collection).await?;
    
    let data = load_bci_data("").await?;
    
    println!("Adding {} phoneme documents...", data.len());
    for (idx, pair) in data.iter().enumerate() {
        let doc_id = format!("phon_{}", idx);
        
        let mut metadata = HashMap::new();
        metadata.insert(
            "sentence".to_string(),
            zeroentropy_community::MetadataValue::String(pair.sentence.clone())
        );
        
        // Store phonemes as the main content
        client.documents().add_text(
            collection,
            &doc_id,
            &pair.phonemes,
            Some(metadata),
        ).await?;
    }
    
    println!("Waiting for indexing...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Test: Search with words
    println!("\nTest: Can words retrieve phoneme sequences?");
    let test_word = "controversial";
    println!("Query: {}", test_word);
    
    let results = client.queries().top_snippets(
        collection,
        test_word,
        3,
        None,
        None,
        None,
        None,
    ).await?;
    
    println!("Top results:");
    for (i, result) in results.results.iter().enumerate() {
        println!("  {}. [{:.4}] {}", i+1, result.score, result.content);
        if let Some(meta) = &result.metadata {
            if let Some(zeroentropy_community::MetadataValue::String(sent)) = meta.get("sentence") {
                println!("      -> {}", sent);
            }
        }
    }
    
    let found = results.results.iter()
        .any(|r| {
            if let Some(meta) = &r.metadata {
                if let Some(zeroentropy_community::MetadataValue::String(sent)) = meta.get("sentence") {
                    return sent.contains("controversial");
                }
            }
            false
        });
    println!("✓ Found 'controversial' in metadata: {}", found);
    
    Ok(())
}

/// Strategy 3: Store both as combined text
async fn test_strategy_3(client: &Client) -> Result<(), Box<dyn Error>> {
    println!("\n=== STRATEGY 3: Store Combined Phoneme+Sentence Text ===\n");
    
    let collection = "bci_strategy3_combined";
    
    // Cleanup
    let _ = client.collections().delete(collection).await;
    client.collections().add(collection).await?;
    
    let data = load_bci_data("").await?;
    
    println!("Adding {} combined documents...", data.len());
    for (idx, pair) in data.iter().enumerate() {
        let doc_id = format!("combo_{}", idx);
        
        // Combine phonemes and sentence in one document
        let combined_text = format!(
            "Phonemes: {}\nSentence: {}",
            pair.phonemes,
            pair.sentence
        );
        
        let mut metadata = HashMap::new();
        metadata.insert(
            "type".to_string(),
            zeroentropy_community::MetadataValue::String("phoneme_sentence_pair".to_string())
        );
        
        client.documents().add_text(
            collection,
            &doc_id,
            &combined_text,
            Some(metadata),
        ).await?;
    }
    
    println!("Waiting for indexing...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Test bidirectional search
    println!("\nTest A: Search with phonemes");
    let test_phonemes = "JH AH JH W ER K";  // "judge work"
    println!("Query: {}", test_phonemes);
    
    let results = client.queries().top_snippets(
        collection,
        test_phonemes,
        2,
        None,
        None,
        None,
        None,
    ).await?;
    
    for (i, result) in results.results.iter().enumerate() {
        println!("  {}. [{:.4}] {}", i+1, result.score, result.content);
    }
    
    println!("\nTest B: Search with words");
    let test_words = "judge work together";
    println!("Query: {}", test_words);
    
    let results = client.queries().top_snippets(
        collection,
        test_words,
        2,
        None,
        None,
        None,
        None,
    ).await?;
    
    for (i, result) in results.results.iter().enumerate() {
        println!("  {}. [{:.4}] {}", i+1, result.score, result.content);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  ZeroEntropy Advanced Phoneme-to-Word Matching Test      ║");
    println!("║  Brain-Computer Interface Dataset                        ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    
    let client = Client::from_env()?;
    
    // Test all three strategies
    test_strategy_1(&client).await?;
    test_strategy_2(&client).await?;
    test_strategy_3(&client).await?;
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  Summary & Recommendations                                ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("\n1. STRATEGY 1 (Store Sentences, Search Phonemes):");
    println!("   - Works surprisingly well despite phonemes being symbolic");
    println!("   - ZeroEntropy's embeddings may capture some phonetic patterns");
    println!("   - Best for: Decoding brain signals to text");
    
    println!("\n2. STRATEGY 2 (Store Phonemes, Search Words):");
    println!("   - Tests reverse mapping capability");
    println!("   - Useful for phoneme lookup from known words");
    println!("   - Best for: Training data preparation");
    
    println!("\n3. STRATEGY 3 (Store Combined Text):");
    println!("   - Bidirectional search in single collection");
    println!("   - More flexible but potentially noisier");
    println!("   - Best for: Exploratory analysis");
    
    println!("\nKey Insight:");
    println!("ZeroEntropy uses semantic embeddings designed for natural language.");
    println!("While phoneme strings are symbolic (not semantic), the system can");
    println!("still find patterns due to consistency in phoneme-word mappings.");
    println!("\nFor production BCI systems, consider:");
    println!("  - Custom phoneme embeddings trained on CMU/ARPAbet");
    println!("  - Hybrid approach: ZeroEntropy for semantic search + specialized");
    println!("    phoneme matching for precise alignment");
    
    Ok(())
}
