use counter::Counter;
use crate::ngram::get_ngram_counter;

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
    
    for (references, translation) in 
        reference_corpus.iter().zip(translation_corpus.iter()) {
        references_length += references.iter().map(|x| x.len()).min().unwrap();
        translation_length += translation.len();
        let translation_ngram_counts = get_ngram_counter(translation, max_order);
        let mut merged_ref_ngram_counts = Counter::new();
        for reference in references {
            merged_ref_ngram_counts |= get_ngram_counter(&reference, max_order);
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
        let reference_corpus: Vec<Vec<String>> = vec![vec!["Hello".to_string()]];
        let translation_corpus: Vec<String> = vec!["Yellow".to_string()];
        let max_order: usize = 4;
        let smooth: bool = true;
        let res = compute_bleu(reference_corpus, translation_corpus, max_order, smooth);
        // (0.6147881529512643, [0.7142857142857143, 0.6666666666666666, 0.6, 0.5], 1.0, 1.2, 6, 5)
        assert_eq!((res.bleu - 0.6147881529512643) < 1e-10, true);
    }
}