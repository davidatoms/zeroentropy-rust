use zeroentropy_community::Client;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use serde_json::Value;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gpt2-search")]
#[command(about = "Search GPT-2 dataset using ZeroEntropy semantic search", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the GPT-2 dataset directory
    #[arg(short, long, default_value = "../gpt-2-output-dataset/data")]
    dataset: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Index dataset into ZeroEntropy collections
    Index {
        /// Number of documents to index per collection (0 = all documents)
        #[arg(short, long, default_value = "0")]
        limit: usize,

        /// Collections to index (comma-separated: webtext,gpt2_small,gpt2_medium,gpt2_large,gpt2_xl)
        #[arg(short, long, value_delimiter = ',')]
        collections: Option<Vec<String>>,
    },
    /// Search indexed collections
    Search {
        /// Search query
        query: String,

        /// Number of results per collection
        #[arg(short, long, default_value = "5")]
        limit: usize,

        /// Collections to search (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        collections: Option<Vec<String>>,
    },
    /// Run predefined code search queries
    CodeSearch {
        /// Collections to search (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        collections: Option<Vec<String>>,
    },
    /// Interactive search mode
    Interactive,
}

fn expand_tilde(path: &Path) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        if path_str.starts_with("~") {
            if let Some(home) = std::env::var("USERPROFILE").ok().or_else(|| std::env::var("HOME").ok()) {
                return PathBuf::from(path_str.replacen("~", &home, 1));
            }
        }
    }
    path.to_path_buf()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    // Create client from ZEROENTROPY_API_KEY environment variable
    let client = Client::from_env()?;

    // Dataset path - expand tilde if present
    let dataset_path = expand_tilde(&cli.dataset);
    
    // Available collections
    let all_collections = vec![
        ("webtext", "webtext.valid.jsonl"),
        ("gpt2_small", "small-117M.valid.jsonl"),
        ("gpt2_medium", "medium-345M.valid.jsonl"),
        ("gpt2_large", "large-762M.valid.jsonl"),
        ("gpt2_xl", "xl-1542M.valid.jsonl"),
    ];

    match cli.command {
        Commands::Index { limit, collections: selected } => {
            let collections_to_index = filter_collections(&all_collections, selected);
            index_collections(&client, &dataset_path, &collections_to_index, limit).await?;
        }
        Commands::Search { query, limit, collections: selected } => {
            let collections_to_search = filter_collections(&all_collections, selected);
            search_collections(&client, &collections_to_search, &query, limit).await?;
        }
        Commands::CodeSearch { collections: selected } => {
            let collections_to_search = filter_collections(&all_collections, selected);
            code_search(&client, &collections_to_search).await?;
        }
        Commands::Interactive => {
            interactive_search(&client, &all_collections).await?;
        }
    }

    Ok(())
}

fn filter_collections<'a>(
    all_collections: &'a [(&'a str, &'a str)],
    selected: Option<Vec<String>>,
) -> Vec<(&'a str, &'a str)> {
    match selected {
        Some(names) => all_collections
            .iter()
            .filter(|(name, _)| names.contains(&name.to_string()))
            .copied()
            .collect(),
        None => all_collections.to_vec(),
    }
}

async fn index_collections(
    client: &Client,
    dataset_path: &Path,
    collections: &[(&str, &str)],
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(60));
    println!("Indexing GPT-2 Dataset");
    println!("{}", "=".repeat(60));
    if limit == 0 {
        println!("Indexing all documents");
    } else {
        println!("Limit: {} documents per collection", limit);
    }
    println!();
    for (collection_name, filename) in collections {
            println!("\nProcessing {}...", collection_name);
            
            // Create collection
            match client.collections().add(*collection_name).await {
                Ok(response) => println!("  {}", response.message),
                Err(zeroentropy_community::Error::Conflict(_)) => {
                    println!("  Collection already exists");
                }
                Err(e) => return Err(e.into()),
            }

            // Load and index samples
            let file_path = dataset_path.join(filename);
            if !file_path.exists() {
                println!("  File not found: {}", file_path.display());
                continue;
            }

            let file = File::open(&file_path)?;
            let reader = BufReader::new(file);
            
            if limit == 0 {
                println!("  Indexing all samples...");
            } else {
                println!("  Indexing up to {} samples...", limit);
            }
            let mut count = 0;
            
            for (idx, line) in reader.lines().enumerate() {
                if limit > 0 && idx >= limit {
                    break;
                }
                
                let line = line?;
                if let Ok(json) = serde_json::from_str::<Value>(&line) {
                    if let Some(text) = json.get("text").and_then(|v| v.as_str()) {
                        // Add metadata
                        let mut metadata = HashMap::new();
                        metadata.insert(
                            "source".to_string(),
                            zeroentropy_community::MetadataValue::String(collection_name.to_string()),
                        );
                        metadata.insert(
                            "index".to_string(),
                            zeroentropy_community::MetadataValue::String(idx.to_string()),
                        );
                        
                        // Index the document
                        let doc_path = format!("{}_{}.txt", collection_name, idx);
                        match client.documents().add_text(
                            *collection_name,
                            &doc_path,
                            text,
                            Some(metadata),
                        ).await {
                            Ok(_) => count += 1,
                            Err(zeroentropy_community::Error::Conflict(_)) => {
                                // Document already exists, skip silently
                                continue;
                            }
                            Err(e) => eprintln!("  Error adding document {}: {}", idx, e),
                        }
                        
                        if count % 10 == 0 {
                            print!(".");
                            std::io::Write::flush(&mut std::io::stdout())?;
                        }
                    }
                }
            }
            
            println!("\n  Indexed {} documents from {}", count, collection_name);
    }
    
    Ok(())
}

async fn code_search(
    client: &Client,
    collections: &[(&str, &str)],
) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(60));
        println!("Code Search Mode");
        println!("{}", "=".repeat(60));
        println!();
        
        // Search queries for code
        let code_queries = vec![
            "function definition programming code",
            "import module python java javascript",
            "class method implementation",
            "for loop while loop iteration",
            "if else conditional statement",
            "return variable assignment",
            "API endpoint HTTP request",
            "database query SQL",
        ];

        for query in code_queries {
            println!("\nSearching for: \"{}\"", query);
            println!("{}", "-".repeat(60));
            
            // Search each collection
            for (collection_name, _) in collections {
                let results = match client.queries().top_snippets(
                    *collection_name,
                    query,
                    3, // Top 3 results per collection
                    None,
                    Some(true), // include metadata
                    None,
                    None,
                ).await {
                    Ok(r) => r,
                    Err(e) => {
                        println!("  Error searching {}: {}", collection_name, e);
                        continue;
                    }
                };

                if !results.results.is_empty() {
                    println!("\n  {} ({} results):", collection_name, results.results.len());
                    
                    for (i, result) in results.results.iter().take(2).enumerate() {
                        println!("\n    {}. {} (score: {:.4})", i + 1, result.path, result.score);
                        
                        // Show first 200 chars of content
                        let preview = if result.content.len() > 200 {
                            format!("{}...", &result.content[..200])
                        } else {
                            result.content.clone()
                        };
                        println!("       {}", preview.replace('\n', "\n       "));
                    }
                }
            }
            
            println!(); // Blank line between queries
        }
    
    Ok(())
}

async fn search_collections(
    client: &Client,
    collections: &[(&str, &str)],
    query: &str,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(60));
    println!("Searching: \"{}\"", query);
    println!("{}", "=".repeat(60));
    println!();

    for (collection_name, _) in collections {
        let results = match client.queries().top_snippets(
            *collection_name,
            query,
            limit as u32,
            None,
            Some(true),
            None,
            None,
        ).await {
            Ok(r) => r,
            Err(e) => {
                println!("Error searching {}: {}", collection_name, e);
                continue;
            }
        };

        if !results.results.is_empty() {
            println!("{} - Found {} matches:", collection_name, results.results.len());
            
            for (i, result) in results.results.iter().enumerate() {
                println!("\n  {}. {} (score: {:.4})", i + 1, result.path, result.score);
                
                let preview = if result.content.len() > 300 {
                    format!("{}...", &result.content[..300])
                } else {
                    result.content.clone()
                };
                println!("     {}", preview.replace('\n', "\n     "));
            }
            println!();
        }
    }
    
    Ok(())
}

async fn interactive_search(
    client: &Client,
    collections: &[(&str, &str)],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("Interactive Search (type 'quit' to exit)");
    println!("{}", "=".repeat(60));
    
    loop {
        println!("\nEnter search query: ");
        let mut query = String::new();
        std::io::stdin().read_line(&mut query)?;
        let query = query.trim();
        
        if query.is_empty() {
            continue;
        }
        
        if query.eq_ignore_ascii_case("quit") || query.eq_ignore_ascii_case("exit") {
            break;
        }
        
        println!("\nSearching all collections for: \"{}\"", query);
        println!("{}", "-".repeat(60));
        
        // Search all collections
        for (collection_name, _) in collections {
            let results = match client.queries().top_snippets(
                *collection_name,
                query,
                5,
                None,
                Some(true),
                None,
                None,
            ).await {
                Ok(r) => r,
                Err(e) => {
                    println!("  Error: {}", e);
                    continue;
                }
            };

            if !results.results.is_empty() {
                println!("\n  {} - Found {} matches:", collection_name, results.results.len());
                
                for (i, result) in results.results.iter().take(3).enumerate() {
                    println!("\n    {}. {} (score: {:.4})", i + 1, result.path, result.score);
                    
                    let preview = if result.content.len() > 300 {
                        format!("{}...", &result.content[..300])
                    } else {
                        result.content.clone()
                    };
                    println!("       {}", preview.replace('\n', "\n       "));
                }
            }
        }
    }
    
    Ok(())
}
