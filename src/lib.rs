mod tokenizer;

use pyo3::prelude::*;
use crate::tokenizer::Tokenizer;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn regex_tokenizer(line: &str) -> PyResult<Vec<String>> {
    let mut tokenizer_regex = tokenizer::TokenizerRegex::new();
    let res = tokenizer_regex.tokenize(line);
    Ok(res)
}

/// A Python module implemented in Rust.
#[pymodule]
fn bleuscore(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(regex_tokenizer, m)?)?;
    Ok(())
}
