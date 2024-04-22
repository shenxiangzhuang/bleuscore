use std::cmp::min;
use std::collections::HashMap;
use crate::ngram::{get_token_ngram_counter};
use crate::tokenizer::{Tokenizer, Tokenizer13a};


#[derive(Debug, Default)]
pub struct BleuScore {
    pub bleu: f64,
    pub precisions: Vec<f64>,
    pub bp: f64,
    pub ratio: f64,
    pub translation_length: usize,
    pub reference_length: usize,
}


pub fn compute_bleu(
    reference_corpus: Vec<Vec<String>>,
    translation_corpus: Vec<String>,
    max_order: usize,
    smooth: bool,
) -> BleuScore {
    let mut matches_by_order: Vec<usize> = vec![0; max_order];
    let mut possible_matches_by_order: Vec<usize> = vec![0; max_order];
    let mut references_length: usize = 0;
    let mut translation_length: usize = 0;
    
    let tokenizer = Tokenizer13a::new();
    
    for (references, translation) in 
        reference_corpus.iter().zip(translation_corpus.iter()) {
        
        // tokenize
        let translation_tokens = tokenizer.tokenize(translation);
        let references_tokens: Vec<Vec<String>> = references.iter().map(|x| tokenizer.tokenize(x)).collect();
        // println!("translation_tokens: {:?}\nreferences_tokens: {:?}", translation_tokens, references_tokens);

        references_length += references_tokens.iter().map(|x| x.len()).min().unwrap();
        translation_length += translation_tokens.len();
        let translation_ngram_counts = get_token_ngram_counter(&translation_tokens, max_order);
        let mut merged_ref_ngram_counts = HashMap::new();
        for reference_tokens in references_tokens {
            let reference_ngram_counts = get_token_ngram_counter(&reference_tokens, max_order);
            for (key, value) in reference_ngram_counts {
                merged_ref_ngram_counts.entry(key).and_modify(|v| *v += value).or_insert(value);
            }
        }
        
        // let overlap: Vec<String> = merged_ref_ngram_counts.keys().filter(|&key| translation_ngram_counts.contains_key(key)).cloned().collect();
        let mut overlap_counts = HashMap::new();
        for (k, v) in translation_ngram_counts {
            let key = k.clone();
            if merged_ref_ngram_counts.contains_key(&key) {
                overlap_counts.insert(k, min(merged_ref_ngram_counts[&key], v));
                // println!("({}, {}): trans: {}; ref: {}", key.0, key.1, v, merged_ref_ngram_counts[&key]);
            }
            else { 
                continue
            }
        }
        
        for key in overlap_counts.keys() {
            let (_, order) = key;
            matches_by_order[order - 1] += overlap_counts[&key];
            // println!("order: {order}, match: {}", matches_by_order[order - 1]);
        }
        for order in 1..=max_order {
            let possible_matches = translation_tokens.len().saturating_sub(order - 1);
            if possible_matches > 0 {
                // println!("Order: {order}");
                possible_matches_by_order[order - 1] += possible_matches
            }
        }
    }
        
    let mut precisions:Vec<f64> = vec![0.0; max_order];
    for i in 0..max_order {
        match smooth {
            true => {
                precisions[i] = (matches_by_order[i] as f64 + 1.0) / (possible_matches_by_order[i] as f64 + 1.0);
                // println!("precision[i]: {i}, {} / {} = {}", matches_by_order[i] as f64 + 1.0, possible_matches_by_order[i] as f64 + 1.0, precisions[i]);
            },
            false => {
                if possible_matches_by_order[i] > 0 {
                    precisions[i] = (matches_by_order[i] as f64) / (possible_matches_by_order[i] as f64)
                }
                else { 
                    precisions[i] = 0.0;
                }
            }
        }
    }
    
    let mut geo_mean = 0.0;
    
    if precisions.iter().fold(f64::INFINITY, |a, &b| a.min(b)) > 0.0 {
        let p_log_sum: f64 = (1.0 / max_order as f64) * precisions.iter().map(|&x| x.ln()).sum::<f64>();
        geo_mean = p_log_sum.exp();
    }

    let ratio: f64 = translation_length as f64 / references_length as f64;
    let mut bp = 1.0;
    if ratio <= 1.0 {
        bp = (1.0 - 1.0 / ratio).exp();
    }
    let bleu = geo_mean * bp;
    BleuScore{bleu, precisions, bp, ratio, translation_length, reference_length: references_length}
}


#[cfg(test)]
mod test {
    use crate::bleu::{compute_bleu};
    #[test]
    fn test_bleu() {
        let reference_corpus: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
        let translation_corpus: Vec<String> = vec!["Yellow, World!".to_string()];
        let max_order: usize = 4;
        let smooth: bool = true;
        let res = compute_bleu(reference_corpus, translation_corpus, max_order, smooth);
        // (0.668740304976422, [0.8, 0.75, 0.6666666666666666, 0.5], 1.0, 1.0, 4, 4)
        println!("BLEU: {:?}", res);
        assert_eq!((res.bleu - 0.668740304976422).abs() < 1e-10, true);
    }
}