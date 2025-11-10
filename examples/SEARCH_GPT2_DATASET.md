# GPT-2 Dataset Code Search with ZeroEntropy

This tool indexes and searches the GPT-2 output dataset using ZeroEntropy's semantic search capabilities.

## Prerequisites

1. **ZeroEntropy API Key** - Set your API key:
   ```powershell
   $env:ZEROENTROPY_API_KEY = "your-api-key-here"
   ```

2. **GPT-2 Dataset** - The dataset should be in `../gpt-2-output-dataset/data/` with these files:
   - `webtext.valid.jsonl` (human text)
   - `small-117M.valid.jsonl` (GPT-2 small)
   - `medium-345M.valid.jsonl` (GPT-2 medium)
   - `large-762M.valid.jsonl` (GPT-2 large)
   - `xl-1542M.valid.jsonl` (GPT-2 XL)

## Usage

### Run the Program

```powershell
cargo run --release --example search_gpt2_dataset
```

### Options

When you run it, you'll see three options:

1. **Index dataset** - Required first time. Indexes the first 100 samples from each dataset (takes ~10-20 minutes)
2. **Search existing collections** - Search if you've already indexed
3. **Index and search** - Do both

### What It Does

#### Indexing Phase
- Creates 5 ZeroEntropy collections (one for each dataset)
- Indexes first 100 samples from each dataset
- Adds metadata: source and index number

#### Search Phase
- **Automated Code Search**: Searches for common code patterns:
  - Function definitions
  - Import statements
  - Class methods
  - Loops and conditionals
  - API/HTTP requests
  - Database queries
  
- **Interactive Search**: Enter custom queries to search all collections

### Example Queries

Try searching for:
- `"function definition programming code"`
- `"import module python java javascript"`
- `"class method implementation"`
- `"for loop while loop iteration"`
- `"API endpoint HTTP request"`

### Output

For each query, you'll see:
- Which collection (webtext, gpt2_small, etc.)
- Number of matches
- Top results with relevance scores
- Preview of matching content

### Finding Code in the Dataset

This tool helps answer: **"Is there code in the GPT-2 training/output data?"**

By comparing search results across:
- **webtext** (human-written, WebText corpus)
- **gpt2_small/medium/large/xl** (AI-generated)

You can see:
1. If code exists in the corpus
2. Whether GPT-2 generates more/less code than humans
3. Quality differences in generated code

## Performance

- **Indexing**: ~100 documents × 5 collections = ~10-20 minutes
- **Search**: <1 second per query across all collections

## Scaling Up

To index more samples, edit line 73 in `search_gpt2_dataset.rs`:

```rust
if idx >= 100 {  // Change to 1000, 5000, etc.
    break;
}
```

The full dataset has 5,000 validation samples per collection.

## Next Steps

After running this, you can:

1. **Analyze code density** - Count how many code-like samples exist
2. **Compare patterns** - See if GPT-2 has different code "signatures" than humans
3. **Feed into watermark detector** - Use results to improve the detector in `gpt2-watermark-detector`
4. **Build a corpus map** - Understand what's in the training data

## Related Projects

- [zeroentropy-rust](../) - The ZeroEntropy Rust SDK
- [gpt-2](../../gpt-2) - GPT-2 text generation
- [gpt2-watermark-detector](../../gpt2-watermark-detector) - Semantic watermark detection
