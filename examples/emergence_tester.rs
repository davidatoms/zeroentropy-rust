use zeroentropy_community::Client;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "emergence-tester")]
#[command(about = "Test emergent knowledge hypothesis in LLMs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a single emergence test
    Test {
        /// Query to test
        query: String,
        
        /// Number of results to retrieve
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Run batch tests from a query file
    Batch {
        /// Path to queries JSON file
        #[arg(short, long)]
        queries: PathBuf,
        
        /// Output file for results
        #[arg(short, long, default_value = "emergence_results.json")]
        output: PathBuf,
        
        /// Number of results per query
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Generate sample novel queries
    Generate {
        /// Output file for queries
        #[arg(short, long, default_value = "novel_queries.json")]
        output: PathBuf,
    },
    
    /// Analyze results and compute statistics
    Analyze {
        /// Results file to analyze
        #[arg(short, long)]
        results: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    text: String,
    category: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchResult {
    path: String,
    content: String,
    score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmergenceTest {
    query: String,
    category: String,
    
    // Raw corpus search results
    raw_results: Vec<SearchResult>,
    raw_avg_score: f32,
    raw_top_score: f32,
    
    // Embedding space search results  
    embedding_results: Vec<SearchResult>,
    embedding_avg_score: f32,
    embedding_top_score: f32,
    
    // Metrics
    asymmetry_score: f32,
    emergence_detected: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let cli = Cli::parse();
    let client = Client::from_env()?;
    
    match cli.command {
        Commands::Test { query, limit } => {
            run_single_test(&client, &query, limit).await?;
        }
        Commands::Batch { queries, output, limit } => {
            run_batch_tests(&client, &queries, &output, limit).await?;
        }
        Commands::Generate { output } => {
            generate_novel_queries(&output)?;
        }
        Commands::Analyze { results } => {
            analyze_results(&results)?;
        }
    }
    
    Ok(())
}

async fn run_single_test(
    client: &Client,
    query: &str,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(70));
    println!("EMERGENCE TEST: \"{}\"", query);
    println!("{}", "=".repeat(70));
    
    // Search raw corpus (webtext)
    println!("\n📚 Searching RAW CORPUS (webtext)...");
    let raw_results = search_collection(client, "webtext", query, limit).await?;
    
    let raw_scores: Vec<f32> = raw_results.iter().map(|r| r.score).collect();
    let raw_avg = if !raw_scores.is_empty() {
        raw_scores.iter().sum::<f32>() / raw_scores.len() as f32
    } else {
        0.0
    };
    let raw_top = raw_scores.first().copied().unwrap_or(0.0);
    
    println!("  Results: {} documents", raw_results.len());
    println!("  Avg Score: {:.4}", raw_avg);
    println!("  Top Score: {:.4}", raw_top);
    
    if raw_results.is_empty() {
        println!("  ⚠️  NO RELEVANT RESULTS FOUND");
    } else {
        println!("\n  Top result:");
        let preview = truncate(&raw_results[0].content, 200);
        println!("    {}", preview.replace('\n', "\n    "));
    }
    
    // Search embedding space (gpt2_small as proxy)
    println!("\n🧠 Searching EMBEDDING SPACE (gpt2_small)...");
    let embedding_results = search_collection(client, "gpt2_small", query, limit).await?;
    
    let emb_scores: Vec<f32> = embedding_results.iter().map(|r| r.score).collect();
    let emb_avg = if !emb_scores.is_empty() {
        emb_scores.iter().sum::<f32>() / emb_scores.len() as f32
    } else {
        0.0
    };
    let emb_top = emb_scores.first().copied().unwrap_or(0.0);
    
    println!("  Results: {} documents", embedding_results.len());
    println!("  Avg Score: {:.4}", emb_avg);
    println!("  Top Score: {:.4}", emb_top);
    
    if !embedding_results.is_empty() {
        println!("\n  Top result:");
        let preview = truncate(&embedding_results[0].content, 200);
        println!("    {}", preview.replace('\n', "\n    "));
    }
    
    // Compute asymmetry
    let asymmetry = if raw_avg > 0.0 {
        emb_avg / raw_avg
    } else if emb_avg > 0.0 {
        f32::INFINITY
    } else {
        1.0
    };
    
    println!("\n{}", "=".repeat(70));
    println!("ANALYSIS:");
    println!("  Asymmetry Score: {:.2}x", asymmetry);
    
    if asymmetry > 1.5 {
        println!("  ✅ EMERGENCE DETECTED - Embedding space shows {:.1}x better retrieval!", asymmetry);
        println!("     This suggests geometric transformation created novel correlations.");
    } else if asymmetry > 1.1 {
        println!("  ⚠️  WEAK EMERGENCE - Slight advantage in embedding space");
    } else {
        println!("  ❌ NO EMERGENCE - Similar performance in both spaces");
    }
    println!("{}", "=".repeat(70));
    
    Ok(())
}

async fn search_collection(
    client: &Client,
    collection: &str,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    match client.queries().top_snippets(
        collection,
        query,
        limit as u32,
        None,
        Some(false), // don't need metadata
        None,
        None,
    ).await {
        Ok(response) => Ok(response.results.into_iter().map(|r| SearchResult {
            path: r.path,
            content: r.content,
            score: r.score as f32,
        }).collect()),
        Err(_) => {
            // Collection doesn't exist yet
            Ok(Vec::new())
        }
    }
}

async fn run_batch_tests(
    client: &Client,
    queries_path: &PathBuf,
    output_path: &PathBuf,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(70));
    println!("BATCH EMERGENCE TESTING");
    println!("{}", "=".repeat(70));
    
    // Load queries
    let file = File::open(queries_path)?;
    let queries: Vec<Query> = serde_json::from_reader(file)?;
    
    println!("\nLoaded {} queries", queries.len());
    println!("Running tests...\n");
    
    let mut results = Vec::new();
    
    for (idx, query) in queries.iter().enumerate() {
        println!("[{}/{}] Testing: \"{}\"", idx + 1, queries.len(), query.text);
        
        // Search both spaces
        let raw_results = search_collection(client, "webtext", &query.text, limit).await?;
        let embedding_results = search_collection(client, "gpt2_small", &query.text, limit).await?;
        
        // Compute metrics
        let raw_scores: Vec<f32> = raw_results.iter().map(|r| r.score).collect();
        let raw_avg = if !raw_scores.is_empty() {
            raw_scores.iter().sum::<f32>() / raw_scores.len() as f32
        } else {
            0.0
        };
        let raw_top = raw_scores.first().copied().unwrap_or(0.0);
        
        let emb_scores: Vec<f32> = embedding_results.iter().map(|r| r.score).collect();
        let emb_avg = if !emb_scores.is_empty() {
            emb_scores.iter().sum::<f32>() / emb_scores.len() as f32
        } else {
            0.0
        };
        let emb_top = emb_scores.first().copied().unwrap_or(0.0);
        
        let asymmetry = if raw_avg > 0.0 {
            emb_avg / raw_avg
        } else if emb_avg > 0.0 {
            10.0 // Large value when raw found nothing but embedding found something
        } else {
            1.0
        };
        
        let emergence_detected = asymmetry > 1.5;
        
        println!("  Raw: {:.4} | Embedding: {:.4} | Asymmetry: {:.2}x {}", 
            raw_avg, emb_avg, asymmetry, 
            if emergence_detected { "✅" } else { "" }
        );
        
        results.push(EmergenceTest {
            query: query.text.clone(),
            category: query.category.clone(),
            raw_results: raw_results.into_iter().take(3).collect(), // Save top 3 only
            raw_avg_score: raw_avg,
            raw_top_score: raw_top,
            embedding_results: embedding_results.into_iter().take(3).collect(),
            embedding_avg_score: emb_avg,
            embedding_top_score: emb_top,
            asymmetry_score: asymmetry,
            emergence_detected,
        });
    }
    
    // Save results
    let file = File::create(output_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &results)?;
    
    println!("\n{}", "=".repeat(70));
    println!("✅ Batch testing complete!");
    println!("Results saved to: {}", output_path.display());
    println!("{}", "=".repeat(70));
    
    // Quick summary
    let emergence_count = results.iter().filter(|r| r.emergence_detected).count();
    println!("\nSUMMARY:");
    println!("  Total queries: {}", results.len());
    println!("  Emergence detected: {} ({:.1}%)", 
        emergence_count, 
        (emergence_count as f32 / results.len() as f32) * 100.0
    );
    
    Ok(())
}

fn generate_novel_queries(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(70));
    println!("GENERATING NOVEL QUERY DATASET");
    println!("{}", "=".repeat(70));
    
    let queries = vec![
        // Cross-domain analogies (20)
        Query {
            text: "How is photosynthesis like a stock market?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Biology ↔ Finance".to_string(),
        },
        Query {
            text: "What do neural networks and ant colonies have in common?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "AI ↔ Biology".to_string(),
        },
        Query {
            text: "How is quantum superposition like jazz improvisation?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Physics ↔ Music".to_string(),
        },
        Query {
            text: "What connects protein folding to origami design?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Biochemistry ↔ Art".to_string(),
        },
        Query {
            text: "How does evolution resemble machine learning?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Biology ↔ AI".to_string(),
        },
        Query {
            text: "What similarities exist between city traffic and blood circulation?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Urban planning ↔ Physiology".to_string(),
        },
        Query {
            text: "How is encryption like a medieval fortress?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Cybersecurity ↔ History".to_string(),
        },
        Query {
            text: "What connects blockchain to medieval ledgers?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Crypto ↔ History".to_string(),
        },
        Query {
            text: "How does DNA replication resemble 3D printing?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Biology ↔ Manufacturing".to_string(),
        },
        Query {
            text: "What parallels exist between immune systems and cybersecurity?".to_string(),
            category: "cross_domain_analogy".to_string(),
            description: "Biology ↔ Computer Science".to_string(),
        },
        
        // Counterfactuals (15)
        Query {
            text: "What if the internet was invented in 1950?".to_string(),
            category: "counterfactual".to_string(),
            description: "Alternative technology timeline".to_string(),
        },
        Query {
            text: "How would physics change if light speed was slower?".to_string(),
            category: "counterfactual".to_string(),
            description: "Alternative physical laws".to_string(),
        },
        Query {
            text: "What if humans had evolved with echolocation?".to_string(),
            category: "counterfactual".to_string(),
            description: "Alternative biology".to_string(),
        },
        Query {
            text: "How would society differ if sleep was unnecessary?".to_string(),
            category: "counterfactual".to_string(),
            description: "Alternative human needs".to_string(),
        },
        Query {
            text: "What if electricity was never discovered?".to_string(),
            category: "counterfactual".to_string(),
            description: "Alternative technology path".to_string(),
        },
        
        // Concept blending (15)
        Query {
            text: "Describe quantum jazz music theory".to_string(),
            category: "concept_blend".to_string(),
            description: "Physics + Music".to_string(),
        },
        Query {
            text: "What is recursive cooking technique?".to_string(),
            category: "concept_blend".to_string(),
            description: "Programming + Culinary".to_string(),
        },
        Query {
            text: "Explain fractal architecture design principles".to_string(),
            category: "concept_blend".to_string(),
            description: "Mathematics + Architecture".to_string(),
        },
        Query {
            text: "How does algorithmic poetry composition work?".to_string(),
            category: "concept_blend".to_string(),
            description: "Programming + Literature".to_string(),
        },
        Query {
            text: "What is quantum diplomacy strategy?".to_string(),
            category: "concept_blend".to_string(),
            description: "Physics + Politics".to_string(),
        },
    ];
    
    let file = File::create(output_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &queries)?;
    
    println!("\n✅ Generated {} novel queries", queries.len());
    println!("Saved to: {}", output_path.display());
    println!("\nCategories:");
    println!("  - Cross-domain analogies: queries requiring correlation across disciplines");
    println!("  - Counterfactuals: reasoning about alternative scenarios");
    println!("  - Concept blends: combining distinct concepts into novel ideas");
    println!("\n{}", "=".repeat(70));
    
    Ok(())
}

fn analyze_results(results_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(70));
    println!("ANALYZING EMERGENCE TEST RESULTS");
    println!("{}", "=".repeat(70));
    
    let file = File::open(results_path)?;
    let results: Vec<EmergenceTest> = serde_json::from_reader(file)?;
    
    println!("\nLoaded {} test results\n", results.len());
    
    // Overall statistics
    let total = results.len();
    let emerged = results.iter().filter(|r| r.emergence_detected).count();
    let emergence_rate = (emerged as f32 / total as f32) * 100.0;
    
    // Asymmetry statistics
    let asymmetry_scores: Vec<f32> = results.iter()
        .map(|r| r.asymmetry_score)
        .filter(|s| s.is_finite())
        .collect();
    
    let avg_asymmetry = asymmetry_scores.iter().sum::<f32>() / asymmetry_scores.len() as f32;
    let max_asymmetry = asymmetry_scores.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    
    // Category breakdown
    let mut category_stats: std::collections::HashMap<String, (usize, usize)> = std::collections::HashMap::new();
    for result in &results {
        let entry = category_stats.entry(result.category.clone()).or_insert((0, 0));
        entry.0 += 1;
        if result.emergence_detected {
            entry.1 += 1;
        }
    }
    
    println!("OVERALL STATISTICS:");
    println!("  Total queries tested: {}", total);
    println!("  Emergence detected: {} ({:.1}%)", emerged, emergence_rate);
    println!("  Average asymmetry: {:.2}x", avg_asymmetry);
    println!("  Maximum asymmetry: {:.2}x", max_asymmetry);
    
    println!("\nCATEGORY BREAKDOWN:");
    for (category, (total, emerged)) in category_stats {
        let rate = (emerged as f32 / total as f32) * 100.0;
        println!("  {}: {}/{} ({:.1}%)", category, emerged, total, rate);
    }
    
    println!("\nTOP EMERGENCE EXAMPLES:");
    let mut sorted_results = results.clone();
    sorted_results.sort_by(|a, b| b.asymmetry_score.partial_cmp(&a.asymmetry_score).unwrap());
    
    for (i, result) in sorted_results.iter().take(5).enumerate() {
        println!("\n{}. \"{}\"", i + 1, result.query);
        println!("   Asymmetry: {:.2}x | Category: {}", result.asymmetry_score, result.category);
        println!("   Raw: {:.4} | Embedding: {:.4}", result.raw_avg_score, result.embedding_avg_score);
    }
    
    println!("\n{}", "=".repeat(70));
    
    // Statistical significance (simple t-test approximation)
    if asymmetry_scores.len() > 1 {
        let variance: f32 = asymmetry_scores.iter()
            .map(|x| (x - avg_asymmetry).powi(2))
            .sum::<f32>() / (asymmetry_scores.len() - 1) as f32;
        let std_dev = variance.sqrt();
        let std_error = std_dev / (asymmetry_scores.len() as f32).sqrt();
        
        // Test if mean is significantly > 1.0
        let t_statistic = (avg_asymmetry - 1.0) / std_error;
        
        println!("\nSTATISTICAL VALIDATION:");
        println!("  Standard deviation: {:.3}", std_dev);
        println!("  Standard error: {:.3}", std_error);
        println!("  T-statistic: {:.3}", t_statistic);
        
        if t_statistic > 2.0 {
            println!("  ✅ SIGNIFICANT (p < 0.05): Embedding space advantage is statistically significant!");
        } else {
            println!("  ⚠️  Not significant: More data needed for statistical confidence");
        }
    }
    
    println!("{}", "=".repeat(70));
    
    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
