mod tokenizer;
mod ngram;
mod bleu;

use pyo3::prelude::*;
use crate::tokenizer::Tokenizer;


#[pyfunction]
fn tokenizer_regex(line: &str) -> PyResult<Vec<String>> {
    let tokenizer_regex = tokenizer::TokenizerRegex::new();
    let res = tokenizer_regex.tokenize(line);
    Ok(res)
}

#[pyfunction]
fn tokenizer_13a(line: &str) -> PyResult<Vec<String>> {
    let tokenizer_13a_regex = tokenizer::Tokenizer13a::new();
    let res = tokenizer_13a_regex.tokenize(line);
    Ok(res)
}

#[pyfunction]
pub fn compute_bleu(
    reference_corpus: Vec<Vec<String>>,
    translation_corpus: Vec<String>,
    max_order: usize,
    smooth: bool, ) -> PyResult<f64> {
    let bleu = bleu::compute_bleu(reference_corpus, translation_corpus, max_order, smooth);
    Ok(bleu.bleu)
}


/// A Python module implemented in Rust.
#[pymodule]
fn bleuscore(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenizer_regex, m)?)?;
    m.add_function(wrap_pyfunction!(tokenizer_13a, m)?)?;
    m.add_function(wrap_pyfunction!(compute_bleu, m)?)?;
    Ok(())
}
