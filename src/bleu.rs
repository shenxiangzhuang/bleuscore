use std::cmp::max;
use std::ops::Add;
use counter::Counter;
use crate::ngram::get_ngram_counter;

#[derive(Debug, Default)]
pub struct BleuScore {
    bleu: f64,
    precisions: Vec<f64>,
    bp: f64,
    ratio: f64,
    translation_length: usize,
    reference_length: usize,
}


pub fn compute_bleu(
    reference_corpus: &Vec<Vec<&str>>,
    translation_corpus: &Vec<&str>,
    max_order: usize,
    smooth: bool,
) {
    let mut matches_by_order: Vec<usize> = vec![0; max_order];
    let mut possible_matches_by_order: Vec<usize> = vec![0; max_order];
    let mut references_length: usize = 0;
    let mut translation_length: usize = 0;
    
    for (references, translation) in 
        reference_corpus.iter().zip(translation_corpus.iter()) {
        
        references_length += references.iter().map(|&x| x.len()).min().unwrap();
        translation_length += translation.len();

        let mut translation_ngram_counts = get_ngram_counter(translation, max_order);
        let mut merged_ref_ngram_counts = Counter::new();
        for &reference in references {
            merged_ref_ngram_counts |= get_ngram_counter(reference, max_order);
        }
        let overlap = translation_ngram_counts & merged_ref_ngram_counts;
        
        for ngram in overlap.keys() {
            matches_by_order[ngram.len() - 1] += overlap[ngram]
        }
        for order in 1..=max_order {
            let possible_matches = translation.len() - order + 1;
            if possible_matches > 0 {
                possible_matches_by_order[order - 1] += possible_matches
            }
        }
        
        let mut precisions:Vec<f64> = vec![0.0; max_order];
        for i in 0..max_order {
            match smooth {
                true => {
                    precisions[i] = (matches_by_order[i] as f64 + 1.0) / (possible_matches_by_order[i] as f64 + 1.0);
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
        
        if precisions.iter().fold(f64::INFINITY, |a, &b| a.min(b)) > 0.0 {
            
        }
    }
    
}