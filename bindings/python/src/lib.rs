use bleuscore::bleu::compute_score;
use bleuscore::tokenizer::{Tokenizer, Tokenizer13a, TokenizerRegex};

use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
fn tokenizer_regex(line: &str) -> PyResult<Vec<String>> {
    let tokenizer_regex = TokenizerRegex::new();
    let res = tokenizer_regex.tokenize(line);
    Ok(res)
}

#[pyfunction]
fn tokenizer_13a(line: &str) -> PyResult<Vec<String>> {
    let tokenizer_13a_regex = Tokenizer13a::new();
    let res = tokenizer_13a_regex.tokenize(line);
    Ok(res)
}

#[pyfunction]
#[pyo3(signature = (references, predictions, max_order=4, smooth=false))]
fn compute(
    references: Vec<Vec<String>>,
    predictions: Vec<String>,
    max_order: usize,
    smooth: bool,
) -> PyResult<PyObject> {
    let bleu = compute_score(&references, &predictions, max_order, smooth);
    Python::with_gil(|py| {
        let bleu_dict = PyDict::new(py);
        bleu_dict.set_item("bleu", bleu.bleu)?;
        bleu_dict.set_item("precisions", bleu.precisions)?;
        bleu_dict.set_item("brevity_penalty", bleu.brevity_penalty)?;
        bleu_dict.set_item("length_ratio", bleu.length_ratio)?;
        bleu_dict.set_item("translation_length", bleu.translation_length)?;
        bleu_dict.set_item("reference_length", bleu.reference_length)?;
        Ok(bleu_dict.into())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn _bleuscore(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenizer_regex, m)?)?;
    m.add_function(wrap_pyfunction!(tokenizer_13a, m)?)?;
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
