# GPT-2 Dataset Search CLI

A command-line tool to search the GPT-2 output dataset using ZeroEntropy semantic search.

## Prerequisites

```powershell
$env:ZEROENTROPY_API_KEY = "your-api-key"
```

## Commands

### 1. Index Dataset

Upload documents to ZeroEntropy collections:

```powershell
# Index 100 documents per collection (default)
cargo run --release --example search_gpt2_dataset -- index

# Index 1000 documents per collection
cargo run --release --example search_gpt2_dataset -- index --limit 1000

# Index only specific collections
cargo run --release --example search_gpt2_dataset -- index --collections webtext,gpt2_small

# Index all 5000 documents per collection (full dataset)
cargo run --release --example search_gpt2_dataset -- index --limit 5000
```

**Collections available:**
- `webtext` - Human-written text (5,000 docs)
- `gpt2_small` - GPT-2 117M (5,000 docs)
- `gpt2_medium` - GPT-2 345M (5,000 docs)
- `gpt2_large` - GPT-2 762M (1,183 docs)
- `gpt2_xl` - GPT-2 1542M (5,000 docs)

### 2. Search

Search indexed collections:

```powershell
# Basic search
cargo run --release --example search_gpt2_dataset -- search "function definition code"

# Search with more results
cargo run --release --example search_gpt2_dataset -- search "python import" --limit 10

# Search specific collections only
cargo run --release --example search_gpt2_dataset -- search "machine learning" --collections webtext,gpt2_xl
```

### 3. Code Search

Run predefined queries to find code patterns:

```powershell
# Search all collections for code
cargo run --release --example search_gpt2_dataset -- code-search

# Search specific collections
cargo run --release --example search_gpt2_dataset -- code-search --collections gpt2_small,gpt2_medium
```

**Predefined queries:**
- Function definitions
- Import statements
- Class methods
- Loops and conditionals
- API/HTTP requests
- Database queries
- Return statements
- Variable assignments

### 4. Interactive Mode

Enter interactive search mode:

```powershell
cargo run --release --example search_gpt2_dataset -- interactive
```

Then type queries interactively. Type `quit` or `exit` to exit.

## Examples

### Find code in the dataset

```powershell
# Index a small sample first
cargo run --release --example search_gpt2_dataset -- index --limit 100

# Run code search
cargo run --release --example search_gpt2_dataset -- code-search
```

### Compare human vs AI text

```powershell
# Search for academic writing
cargo run --release --example search_gpt2_dataset -- search "research methodology study"

# Results will show matches from both webtext (human) and GPT-2 models
```

### Find specific programming languages

```powershell
cargo run --release --example search_gpt2_dataset -- search "javascript function async await"
cargo run --release --example search_gpt2_dataset -- search "python class inheritance"
cargo run --release --example search_gpt2_dataset -- search "rust ownership borrow checker"
```

## Dataset Path

By default, looks for dataset in `../gpt-2-output-dataset/data/`. 

To use a different path:

```powershell
cargo run --release --example search_gpt2_dataset -- --dataset C:\path\to\data index
```

## Upload Estimates

Based on **21,183 total documents** in the dataset:

| Limit per collection | Total docs uploaded | Est. time (5 collections) |
|---------------------|--------------------|-----------------------|
| 100 | 500 | ~5-10 minutes |
| 1000 | 5,000 | ~30-60 minutes |
| 5000 | 21,183 | ~2-3 hours |

## Output Examples

### Search Results
```
============================================================
Searching: "function definition code"
============================================================

📊 webtext - Found 3 matches:

  1. webtext_42.txt (score: 0.8234)
     Here's a simple function definition: function add(a, b) { return a + b; }

📊 gpt2_small - Found 2 matches:

  1. gpt2_small_15.txt (score: 0.7891)
     The function can be defined as follows: def calculate...
```

## Tips

1. **Start small**: Index 100 docs first to test
2. **Use specific queries**: More specific = better results
3. **Compare collections**: Search results across collections show patterns
4. **Build incrementally**: Index more as needed

## Related

- Main tool: `examples/search_gpt2_dataset.rs`
- Documentation: `examples/SEARCH_GPT2_DATASET.md`
- ZeroEntropy SDK: `README.md`
