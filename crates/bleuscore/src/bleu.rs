use crate::ngram::get_token_ngram_counter;
use crate::tokenizer::{Tokenizer, Tokenizer13a};
use ahash::AHashMap;
use rayon::prelude::*;
use std::cmp::min;
use std::ops::Add;

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

/// Used to save the statistic result for every prediction-references pair
#[derive(Debug, Eq, PartialEq)]
struct Stat(usize, usize, Vec<usize>, Vec<usize>);

/// aggregate prediction-references pairs' statistic result into final result
fn agg_stat(s1: Stat, s2: Stat) -> Stat {
    Stat(
        s1.0 + s2.0,
        s1.1 + s2.1,
        add_vectors(s1.2, s2.2),
        add_vectors(s1.3, s2.3),
    )
}

/// compute the BLEU score with `Tokenizer13a` as default tokenizer.
/// The implementation is based on [huggingface/nmt](https://github.com/huggingface/evaluate/blob/main/metrics/bleu/bleu.py)
pub fn compute_score(
    references: &[Vec<String>],
    predictions: &[String],
    max_order: usize,
    smooth: bool,
) -> BleuScore {
    let tokenizer = Tokenizer13a::new();

    // stat calculation
    let stat_result = references
        .into_par_iter()
        .zip(predictions.into_par_iter())
        .map(|(references, translation)| {
            let translation_tokens = tokenizer.tokenize(translation);
            let references_tokens: Vec<Vec<String>> =
                references.iter().map(|x| tokenizer.tokenize(x)).collect();

            let reference_length = references_tokens.iter().map(|x| x.len()).min().unwrap();
            let translation_length = translation_tokens.len();

            let mut possible_matches_by_order = vec![0; max_order];
            for order in 1..=max_order {
                let possible_matches = translation_tokens.len().saturating_sub(order - 1);
                if possible_matches > 0 {
                    possible_matches_by_order[order - 1] += possible_matches
                }
            }

            let translation_ngram_counts = get_token_ngram_counter(&translation_tokens, max_order);
            let mut merged_ref_ngram_counts = AHashMap::new();
            for reference_tokens in references_tokens.iter() {
                let reference_ngram_counts = get_token_ngram_counter(reference_tokens, max_order);
                for (key, value) in reference_ngram_counts {
                    merged_ref_ngram_counts
                        .entry(key)
                        .and_modify(|v| *v += value)
                        .or_insert(value);
                }
            }

            // overlap count
            let mut matches_by_order = vec![0; max_order];
            for (k, v) in translation_ngram_counts {
                if merged_ref_ngram_counts.contains_key(k) {
                    matches_by_order[k.len() - 1] += min(merged_ref_ngram_counts[k], v);
                } else {
                    continue;
                }
            }
            Stat(
                translation_length,
                reference_length,
                possible_matches_by_order,
                matches_by_order,
            )
        })
        .reduce_with(agg_stat);

    let Stat(translation_length, reference_length, possible_matches_by_order, matches_by_order) =
        match stat_result {
            None => panic!("Pair statistics calculation got empty result"),
            Some(stats) => stats,
        };

    // precisions calculation
    let mut precisions: Vec<f64> = vec![0.0; max_order];
    for i in 0..max_order {
        match smooth {
            true => {
                precisions[i] = (matches_by_order[i] as f64 + 1.0)
                    / (possible_matches_by_order[i] as f64 + 1.0);
            }
            false => {
                if possible_matches_by_order[i] > 0 {
                    precisions[i] =
                        (matches_by_order[i] as f64) / (possible_matches_by_order[i] as f64)
                } else {
                    precisions[i] = 0.0;
                }
            }
        }
    }

    let mut geo_mean = 0.0;

    if precisions.iter().fold(f64::INFINITY, |a, &b| a.min(b)) > 0.0 {
        let p_log_sum: f64 =
            (1.0 / max_order as f64) * precisions.iter().map(|&x| x.ln()).sum::<f64>();
        geo_mean = p_log_sum.exp();
    }

    let length_ratio: f64 = translation_length as f64 / reference_length as f64;
    let mut brevity_penalty = 1.0;
    if length_ratio <= 1.0 {
        brevity_penalty = (1.0 - 1.0 / length_ratio).exp();
    }
    let bleu = geo_mean * brevity_penalty;

    BleuScore {
        bleu,
        precisions,
        brevity_penalty,
        length_ratio,
        translation_length,
        reference_length,
    }
}

fn add_vectors<T: Add<Output = T>>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    // Ensure both vectors have the same length
    assert_eq!(vec1.len(), vec2.len(), "Vectors must have the same length");
    // Add elements of vec1 and vec2 element by element
    let result: Vec<_> = vec1.into_iter().zip(vec2).map(|(a, b)| a + b).collect();
    result
}

#[cfg(test)]
mod test {
    use crate::bleu::{add_vectors, agg_stat, compute_score, Stat};
    #[test]
    fn test_bleu() {
        let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
        let predictions: Vec<String> = vec!["Yellow, World!".to_string()];
        let max_order: usize = 4;
        let smooth: bool = true;
        let res = compute_score(&references, &predictions, max_order, smooth);
        // (0.668740304976422, [0.8, 0.75, 0.6666666666666666, 0.5], 1.0, 1.0, 4, 4)
        println!("result: {:?}", res);
        assert!((res.bleu - 0.668740304976422).abs() < 1e-10);
    }
    #[test]
    fn test_add_vectors() {
        let v1 = vec![0, 1];
        let v2 = vec![1, 2];
        assert_eq!(vec![1, 3], add_vectors(v1, v2));
    }

    #[test]
    fn test_stat_agg() {
        let s1 = Stat(0, 1, vec![0, 1], vec![1, 2]);
        let s2 = Stat(1, 2, vec![1, 2], vec![3, 4]);
        assert_eq!(agg_stat(s1, s2), Stat(1, 3, vec![1, 3], vec![4, 6]));
    }
}

#[cfg(test)]
mod benchmark {
    use crate::bleu::compute_score;
    use test::Bencher;
    #[bench]
    fn bench_bleu(b: &mut Bencher) {
        let max_order: usize = 4;
        let smooth: bool = true;
        let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
        let predictions: Vec<String> = vec!["Yellow, World!".to_string()];

        let iter_num: usize = 100;
        b.iter(|| {
            std::hint::black_box(for _ in 1..=iter_num {
                compute_score(&references, &predictions, max_order, smooth);
            });
        });
    }

    #[bench]
    fn bench_batch_bleu(b: &mut Bencher) {
        let batch_size = 100;

        let max_order: usize = 4;
        let smooth: bool = true;
        let mut references: Vec<Vec<String>> = vec![];
        references.resize_with(batch_size, || vec!["Hello, World!".to_string()]);

        let mut predictions: Vec<String> = vec![];
        predictions.resize_with(batch_size, || "Yellow, World!".to_string());

        let iter_num: usize = 100;
        b.iter(|| {
            std::hint::black_box(for _ in 1..=iter_num {
                compute_score(&references, &predictions, max_order, smooth);
            });
        });
    }
}
