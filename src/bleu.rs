use std::cmp::min;
use std::collections::HashMap;
use crate::ngram::{get_token_ngram_counter};
use crate::tokenizer::{Tokenizer, Tokenizer13a};


/// The BLEU score data struct
#[derive(Debug, Default)]
pub struct BleuScore {
    pub bleu: f64,
    pub precisions: Vec<f64>,
    pub brevity_penalty: f64,
    pub length_ratio: f64,
    pub translation_length: usize,
    pub reference_length: usize,
}

/// compute the BLEU score with `Tokenizer13a` as default tokenizer
pub fn compute_score(
    references: Vec<Vec<String>>,
    predictions: Vec<String>,
    max_order: usize,
    smooth: bool,
) -> BleuScore {
    // init
    let mut matches_by_order: Vec<usize> = vec![0; max_order];
    let mut possible_matches_by_order: Vec<usize> = vec![0; max_order];
    let mut reference_length: usize = 0;
    let mut translation_length: usize = 0;
    let tokenizer = Tokenizer13a::new();
    
    for (references, translation) in 
        references.iter().zip(predictions.iter()) {
        // tokenize
        let translation_tokens = tokenizer.tokenize(translation);
        let references_tokens: Vec<Vec<String>> = references.iter()
                                                            .map(|x| tokenizer.tokenize(x))
                                                            .collect();
        // lengths
        reference_length += references_tokens.iter().map(|x| x.len()).min().unwrap();
        translation_length += translation_tokens.len();
        
        // ngram count
        let translation_ngram_counts = get_token_ngram_counter(&translation_tokens, max_order);
        let mut merged_ref_ngram_counts = HashMap::new();
        for reference_tokens in references_tokens {
            let reference_ngram_counts = get_token_ngram_counter(&reference_tokens, max_order);
            for (key, value) in reference_ngram_counts {
                merged_ref_ngram_counts.entry(key).and_modify(|v| *v += value).or_insert(value);
            }
        }
        
        // overlap count
        let mut overlap_counts = HashMap::new();
        for (k, v) in translation_ngram_counts {
            let key = k.clone();
            if merged_ref_ngram_counts.contains_key(&key) {
                overlap_counts.insert(k, min(merged_ref_ngram_counts[&key], v));
            }
            else { 
                continue
            }
        }
        for key in overlap_counts.keys() {
            let (_, order) = key;
            matches_by_order[order - 1] += overlap_counts[&key];
        }
        
        // possible match
        for order in 1..=max_order {
            let possible_matches = translation_tokens.len().saturating_sub(order - 1);
            if possible_matches > 0 {
                possible_matches_by_order[order - 1] += possible_matches
            }
        }
    }
    
    // precisions calculation
    let mut precisions:Vec<f64> = vec![0.0; max_order];
    for i in 0..max_order {
        match smooth {
            true => {
                precisions[i] = (matches_by_order[i] as f64 + 1.0) / 
                    (possible_matches_by_order[i] as f64 + 1.0);
            },
            false => {
                if possible_matches_by_order[i] > 0 {
                    precisions[i] = (matches_by_order[i] as f64) / 
                        (possible_matches_by_order[i] as f64)
                }
                else { 
                    precisions[i] = 0.0;
                }
            }
        }
    }
    
    let mut geo_mean = 0.0;
    
    if precisions.iter().fold(f64::INFINITY, |a, &b| a.min(b)) > 0.0 {
        let p_log_sum: f64 = (1.0 / max_order as f64) * precisions.iter()
                                                                  .map(|&x| x.ln())
                                                                  .sum::<f64>();
        geo_mean = p_log_sum.exp();
    }

    let length_ratio: f64 = translation_length as f64 / reference_length as f64;
    let mut brevity_penalty = 1.0;
    if length_ratio <= 1.0 {
        brevity_penalty = (1.0 - 1.0 / length_ratio).exp();
    }
    let bleu = geo_mean * brevity_penalty;
    BleuScore{bleu, precisions, brevity_penalty, length_ratio, translation_length, reference_length}
}


#[cfg(test)]
mod test {
    use crate::bleu::{compute_score};
    #[test]
    fn test_bleu() {
        let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
        let predictions: Vec<String> = vec!["Yellow, World!".to_string()];
        let max_order: usize = 4;
        let smooth: bool = true;
        let res = compute_score(references, predictions, max_order, smooth);
        // (0.668740304976422, [0.8, 0.75, 0.6666666666666666, 0.5], 1.0, 1.0, 4, 4)
        println!("result: {:?}", res);
        assert!((res.bleu - 0.668740304976422).abs() < 1e-10);
    }
}