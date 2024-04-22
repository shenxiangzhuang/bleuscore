mod tokenizer;
pub use crate::tokenizer::{Tokenizer, Tokenizer13a, TokenizerRegex};
mod ngram;
mod bleu;
pub use crate::bleu::{BleuScore, bleu_score};

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;


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
fn compute_bleu(
    references: Vec<Vec<String>>,
    predictions: Vec<String>,
    max_order: usize,
    smooth: bool,
) -> PyResult<PyObject> {
    let bleu = bleu::bleu_score(references, predictions, max_order, smooth);
    Python::with_gil(|py| {
        let bleu_dict = [
            ("bleu", bleu.bleu.to_object(py)), 
            ("precisions", bleu.precisions.to_object(py)),
            ("brevity_penalty", bleu.brevity_penalty.to_object(py)),
            ("length_ratio", bleu.length_ratio.to_object(py)),
            ("translation_length", bleu.translation_length.to_object(py)),
            ("reference_length", bleu.reference_length.to_object(py)),
        ].into_py_dict_bound(py);
        Ok(bleu_dict.into())
    })

}


/// A Python module implemented in Rust.
#[pymodule]
fn bleuscore(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenizer_regex, m)?)?;
    m.add_function(wrap_pyfunction!(tokenizer_13a, m)?)?;
    m.add_function(wrap_pyfunction!(compute_bleu, m)?)?;
    Ok(())
}
