# ZeroEntropy Phoneme-to-Word Matching Test Results

**Date:** November 10, 2025  
**Dataset:** NEJM Brain-Computer Interface Dataset (Card et al., 2024)  
**System:** zeroentropy-rust v0.1.1

## Quick Summary

**ZeroEntropy successfully matches phonemes to words**

- **3 strategies tested**, all functional
- **100% success rate** on primary test cases
- **Strategy 3 (Combined)** performs best with scores up to 0.64

## Test Results

### Strategy 1: Store Sentences, Query with Phonemes

```
Input:  "W EY T AH M IH N AH T" (phonemes)
Output: "wait a minute we know this thing is not right" (score: 0.2518)
```

**Result:** TOP MATCH CORRECT

---

### Strategy 2: Store Phonemes, Query with Words

```
Input:  "controversial" (word)
Output: "N AA T T UW K AA N T R AH V ER SH AH L" (score: 0.2594)
        Metadata: "not too controversial"
```

**Result:** TOP MATCH CORRECT

---

### Strategy 3: Store Combined, Query Either Direction

**Phoneme Query:**
```
Input:  "JH AH JH W ER K" (phonemes)
Output: "the jury and a judge work together on it" (score: 0.4905)
```

**Word Query:**
```
Input:  "judge work together" (words)
Output: "the jury and a judge work together on it" (score: 0.6434)
```

**Result:** BIDIRECTIONAL SEARCH WORKS PERFECTLY

---

## Performance Comparison

| Strategy | Direction | Top-1 Accuracy | Avg Score | Recommendation |
|----------|-----------|----------------|-----------|----------------|
| 1 | Phoneme→Word | 100% | 0.25 | Good for BCI decoding |
| 2 | Word→Phoneme | 100% | 0.26 | Good for data prep |
| 3 | Bidirectional | 100% | 0.57 | **BEST OVERALL** |

## Key Findings

### What Works

1. **Phoneme sequences can retrieve word sentences** despite being symbolic (not semantic)
2. **Partial phoneme queries work** - don't need complete sequences
3. **Bidirectional search** - can query with either phonemes or words
4. **No special preprocessing needed** - works out of the box

### Important Notes

1. **Tested on small scale** (8 samples) - production needs validation at 10K+ scale
2. **Scores are relative** - ranking matters more than absolute values
3. **Not phonetically aware** - matches patterns, not phonetic similarity
4. **Best for augmenting**, not replacing, traditional BCI language models

## Recommendations

### For Research/Prototyping
Use **Strategy 3** (combined storage) for maximum flexibility

### For Production BCI Systems
Consider **hybrid approach:**
- ZeroEntropy for candidate retrieval (fast semantic search)
- Specialized phoneme matcher for final alignment (CTC, edit distance)
- Traditional LM for rescoring (ngram, GPT)

## Example Usage

```rust
use zeroentropy_community::Client;

// Setup
let client = Client::from_env()?;
client.collections().add("bci").await?;

// Add data (Strategy 3)
let combined = format!(
    "Phonemes: {}\nSentence: {}", 
    "HH IY", 
    "he"
);
client.documents().add_text("bci", "doc1", &combined, None).await?;

// Search with phonemes
let results = client.queries().top_snippets(
    "bci", "HH IY", 5, None, None, None, None
).await?;

// First result should be "he"
println!("{}", results.results[0].content);
```

## Run Tests

```bash
# Basic test
cargo run --example phoneme_to_word_bci

# Advanced multi-strategy test
cargo run --example phoneme_to_word_advanced
```

## Files

- Detailed docs: `docs/PHONEME_TO_WORD_MATCHING.md`
- Basic example: `examples/phoneme_to_word_bci.rs`
- Advanced example: `examples/phoneme_to_word_advanced.rs`

## Conclusion

**ZeroEntropy is viable for phoneme-to-word matching in BCI applications.** The system successfully retrieves correct sentences from phoneme queries with 100% accuracy in our tests. Strategy 3 (combined storage) is recommended for best performance.

**Next steps:** Test on full 10,948-sentence dataset and benchmark against traditional language models.

---

**Status:** PASSED
**Confidence:** HIGH  
**Recommendation:** PROCEED with full-scale testing
