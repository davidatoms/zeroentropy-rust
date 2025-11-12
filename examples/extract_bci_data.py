#!/usr/bin/env python3
"""
Extract phoneme-word pairs from BCI pickle file for ZeroEntropy testing.
"""

import pickle
import json
import sys
from pathlib import Path

def load_pickle_data(pickle_path):
    """Load the BCI pickle file."""
    print(f"Loading pickle file: {pickle_path}")
    with open(pickle_path, 'rb') as f:
        data = pickle.load(f)
    print(f"Loaded data with keys: {data.keys() if hasattr(data, 'keys') else type(data)}")
    return data

def extract_phoneme_word_pairs(data, output_path, max_samples=None):
    """
    Extract phoneme and word pairs from the pickle data.
    
    Args:
        data: Loaded pickle data
        output_path: Path to save JSON output
        max_samples: Maximum number of samples to extract (None for all)
    """
    pairs = []
    
    # Check if data has the BCI structure with parallel lists
    if isinstance(data, dict) and 'cue_sentence' in data and 'cue_sentence_phonemes' in data:
        print(f"Found BCI data structure with parallel lists")
        
        sentences = data['cue_sentence']
        phonemes_lists = data['cue_sentence_phonemes']
        
        n_items = len(sentences)
        print(f"Found {n_items} sentence-phoneme pairs")
        
        if max_samples:
            n_items = min(n_items, max_samples)
        
        for idx in range(n_items):
            try:
                sentence = sentences[idx]
                phonemes = phonemes_lists[idx]
                
                # Convert phonemes list to string
                if isinstance(phonemes, list):
                    phonemes_str = ' '.join(str(p) for p in phonemes)
                else:
                    phonemes_str = str(phonemes)
                
                pairs.append({
                    'sentence': str(sentence),
                    'phonemes': phonemes_str,
                    'index': idx
                })
                
                # Print progress
                if (idx + 1) % 500 == 0:
                    print(f"Processed {idx + 1}/{n_items} pairs...")
                    
            except Exception as e:
                if idx < 10:
                    print(f"Error processing item {idx}: {e}")
                continue
        
        print(f"\nExtracted {len(pairs)} phoneme-word pairs")
        
    else:
        print("Data structure not recognized. Looking for generic structure...")
        # Fallback to original logic
        if isinstance(data, dict):
            items = list(data.values())
            print(f"Using all values: {len(items)} items")
        elif isinstance(data, list):
            items = data
            print(f"Data is a list with {len(items)} items")
        else:
            print(f"Unexpected data type: {type(data)}")
            return []
        
        # Try to extract from items
        print("Could not extract pairs with this structure.")
        return []
    
    # Save to JSON
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(pairs, f, indent=2, ensure_ascii=False)
    
    print(f"Saved to: {output_path}")
    return pairs

def inspect_pickle_structure(data, max_depth=3, current_depth=0):
    """Recursively inspect the structure of the pickle data."""
    indent = "  " * current_depth
    
    if current_depth >= max_depth:
        return
    
    if isinstance(data, dict):
        print(f"{indent}Dict with {len(data)} keys:")
        for key in list(data.keys())[:10]:  # Show first 10 keys
            print(f"{indent}  '{key}': {type(data[key])}")
            if current_depth < max_depth - 1:
                inspect_pickle_structure(data[key], max_depth, current_depth + 1)
    elif isinstance(data, list):
        print(f"{indent}List with {len(data)} items")
        if len(data) > 0:
            print(f"{indent}  First item type: {type(data[0])}")
            if current_depth < max_depth - 1:
                inspect_pickle_structure(data[0], max_depth, current_depth + 1)
    else:
        print(f"{indent}{type(data)}: {str(data)[:100]}")

def main():
    # Paths
    pickle_path = Path("C:/Users/david/OneDrive/Research/ArtificialIntelligence/BrainComputerInterface/nejm-brain-to-text/data/t15_copyTask.pkl")
    output_path = Path("C:/Users/david/OneDrive/Projects/zeroentropy-rust/data/bci_phoneme_word_pairs.json")
    
    # Create output directory if needed
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Load data
    try:
        data = load_pickle_data(pickle_path)
    except FileNotFoundError:
        print(f"Error: Pickle file not found at {pickle_path}")
        return
    except Exception as e:
        print(f"Error loading pickle file: {e}")
        return
    
    # Inspect structure
    print("\n=== Data Structure ===")
    inspect_pickle_structure(data)
    
    print("\n=== Extracting Pairs ===")
    
    # Try to extract all pairs
    pairs = extract_phoneme_word_pairs(data, output_path, max_samples=None)
    
    if pairs:
        print(f"\nSuccess! Extracted {len(pairs)} pairs")
        print("\nFirst 3 examples:")
        for i, pair in enumerate(pairs[:3]):
            print(f"\n{i+1}. Sentence: {pair['sentence']}")
            print(f"   Phonemes: {pair['phonemes'][:100]}...")
    else:
        print("\nNo pairs extracted. The pickle file structure may be different.")
        print("You may need to inspect the pickle file manually and adjust the extraction logic.")

if __name__ == "__main__":
    main()
