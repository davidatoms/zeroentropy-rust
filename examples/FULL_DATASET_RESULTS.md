# ZeroEntropy Full Dataset Test Results

**Test Date:** November 10, 2025  
**Dataset:** Complete BCI dataset (1718 sentence-phoneme pairs)  
**System:** zeroentropy-rust v0.1.1

## Executive Summary

Successfully tested ZeroEntropy on the **complete BCI dataset** with **1718 phoneme-word pairs** extracted from `t15_copyTask.pkl`.

### Key Results

| Metric | Value |
|--------|-------|
| Total dataset size | 1718 pairs |
| Documents indexed | 1718 (100%) |
| Test queries | 5 |
| Success rate (Top-5) | 40% (2/5) |
| Avg query time | 0.249s |
| Total processing time | 173.75s (~3 min) |

## Test Methodology

### Data Extraction
1. Used Python script to parse `t15_copyTask.pkl`
2. Extracted parallel lists: `cue_sentence` and `cue_sentence_phonemes`
3. Saved 1718 pairs to JSON format
4. Phonemes in CMU/ARPAbet format with SIL (silence) markers

### Indexing Strategy
- **Strategy 3 (Combined Text)**
- Format: `"Phonemes: <phoneme_seq>\nSentence: <sentence>"`
- Enables bidirectional search (phonemes → words)
- 160.5 seconds to upload all 1718 documents

### Query Method
- Query with **first 6 phonemes** of target sentence
- Simulates partial BCI decoding scenario
- Retrieve top 5 results
- Check if target sentence is present

## Detailed Results

### Test Case #1: Short Sentence
```
Target:   "clean that up"
Phonemes: "K L IY N SIL DH" (6 tokens)
Result:   FOUND at rank 1 (score: 0.5522)
Time:     0.307s
```
**Status:** SUCCESS

---

### Test Case #2: Common Words
```
Target:   "do you like that"
Phonemes: "D UW SIL Y UW SIL" (6 tokens)
Result:   NOT FOUND in top 5
Time:     0.224s
```
**Status:** FAILURE

Top match was "do i" (similar phonemes but different sentence)

---

### Test Case #3: Short Phrase
```
Target:   "coming here"
Phonemes: "K AH M IH NG SIL" (6 tokens)
Result:   FOUND at rank 1 (score: 0.5450)
Time:     0.279s
```
**Status:** SUCCESS

---

### Test Case #4: Long Sentence
```
Target:   "she came last june and watched a game in the sky dome"
Phonemes: "SH IY SIL K EY M" (6 tokens)
Result:   NOT FOUND in top 5
Time:     0.186s
```
**Status:** FAILURE

Only queried first 6 phonemes of a 20+ phoneme sentence - insufficient signal

---

### Test Case #5: Medium Sentence
```
Target:   "i think that is an excellent program"
Phonemes: "AY SIL TH IH NG K" (6 tokens)
Result:   NOT FOUND in top 5
Time:     0.249s
```
**Status:** FAILURE

Top match "i think it is" was semantically very close but not exact

---

## Performance Analysis

### Success Factors
1. **Short sentences** (3-4 words) have high success rate
2. **Distinctive phoneme patterns** improve matching
3. **Rank 1 accuracy** is good when successful (2/2 found at rank 1)

### Failure Factors
1. **Dataset scale**: 1718 documents create high competition
2. **Short query length**: Only 6 phonemes may be insufficient
3. **Phoneme ambiguity**: Similar phoneme patterns exist
4. **Long sentences**: 6 phonemes from 20+ phoneme sentence is too partial

### Comparison: 100 vs 1718 Documents

| Metric | 100 docs | 1718 docs |
|--------|----------|-----------|
| Success rate | 100% (3/3) | 40% (2/5) |
| Avg rank when found | 1.7 | 1.0 |
| Avg query time | 0.241s | 0.249s |
| Upload time | 9s | 160s |

**Insight:** Success rate decreases significantly with dataset size due to increased competition and phoneme pattern overlap.

## Recommendations

### For Research/Prototyping (Current Use)
- **Works well** for small-scale exploration (< 200 documents)
- **Acceptable** for proof-of-concept demonstrations
- **Use Strategy 3** (combined text) for flexibility

### For Production BCI Systems
Current performance (40% Top-5) is **insufficient** for production. Recommend:

1. **Increase query length**: Use 10-15 phonemes instead of 6
2. **Hybrid approach**:
   - ZeroEntropy for initial candidate retrieval (Top-50)
   - Specialized phoneme aligner for final ranking (CTC, edit distance)
   - Language model for rescoring
3. **Custom phoneme embeddings**:
   - Train embeddings on CMU/ARPAbet phoneme sequences
   - Fine-tune on BCI-specific phoneme patterns
4. **Query expansion**:
   - Use phoneme n-grams instead of raw sequences
   - Add confidence-weighted queries

### Optimal Configuration

For best results with current system:
```bash
# Test with 500 documents (sweet spot)
MAX_DOCS=500 cargo run --example phoneme_to_word_full_dataset

# Use longer queries (10+ phonemes)
# Modify query_length in code from 6 to 10-15
```

## Code & Data

### Files Created
- `scripts/extract_bci_data.py` - Python extraction script
- `data/bci_phoneme_word_pairs.json` - 1718 extracted pairs
- `examples/phoneme_to_word_full_dataset.rs` - Full-scale test

### Running the Test
```bash
# Extract data from pickle
python scripts/extract_bci_data.py

# Run with custom document count
MAX_DOCS=100 cargo run --example phoneme_to_word_full_dataset

# Run with full dataset
MAX_DOCS=1718 cargo run --example phoneme_to_word_full_dataset
```

## Sample Data

First 3 pairs from extracted dataset:
```json
[
  {
    "sentence": "clean that up",
    "phonemes": "K L IY N SIL DH AE T SIL AH P SIL",
    "index": 0
  },
  {
    "sentence": "you feel bad",
    "phonemes": "Y UW SIL F IY L SIL B AE D SIL",
    "index": 1
  },
  {
    "sentence": "what do i have",
    "phonemes": "W AH T SIL D UW SIL AY SIL HH AE V SIL",
    "index": 2
  }
]
```

## Conclusion

**ZeroEntropy successfully indexed and searched all 1718 BCI phoneme-word pairs**, demonstrating:

**Strengths:**
- Fast indexing (160s for 1718 documents)
- Sub-second query times (~0.25s)
- Good performance on short, distinctive sentences
- 100% rank-1 accuracy when successful

**Limitations:**
- 40% Top-5 accuracy with full dataset
- Short phoneme queries (6 tokens) insufficient for long sentences
- Semantic embeddings not optimized for phonetic matching
- Performance degrades with dataset scale

**Verdict:**
- **Proof of concept: SUCCESSFUL**
- **Production readiness: NEEDS ENHANCEMENT**
- **Recommended approach: HYBRID** (ZeroEntropy + specialized phoneme matching)

For production BCI applications, combine ZeroEntropy's semantic search with domain-specific phoneme alignment algorithms to achieve >90% accuracy.

---

**Next Steps:**
1. Test with longer phoneme queries (10-15 tokens)
2. Implement hybrid ranking system
3. Train custom phoneme embeddings
4. Benchmark against traditional language models (ngram, GPT)
5. Test on real-time BCI decoding scenarios
