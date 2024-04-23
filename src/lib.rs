/*!
bleuscore is a [BLEU](https://en.wikipedia.org/wiki/BLEU) score calculator written in pure rust.

# Install:

The crate is called `bleuscore` and you can depend on it via cargo:

```ini
[dependencies]
bleuscore = "*"
```

# Features:

- Tokenized BLEU score calculation like
[huggingface evaluate](https://github.com/huggingface/evaluate/blob/main/metrics/bleu/bleu.py)
and [sacrebleu](https://github.com/mjpost/sacrebleu)


# Basic usage:

```rust
// get the references and prediction data:
let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
let predictions: Vec<String> = vec!["Yellow, World!".to_string()];

// set the parameters:
let max_order: usize = 4;
let smooth: bool = true;

// calculate the BLEU score:
let res = compute_score(references, predictions, max_order, smooth);
println!("result: {:?}", res);
// result: BleuScore { bleu: 0.668740304976422, precisions: [0.8, 0.75, 0.6666666666666666, 0.5], 
// brevity_penalty: 1.0, length_ratio: 1.0, translation_length: 4, reference_length: 4 }
```
!*/
mod tokenizer;
pub use crate::tokenizer::{Tokenizer, Tokenizer13a, TokenizerRegex};
mod ngram;
mod bleu;
pub use crate::bleu::{BleuScore, compute_score};

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
#[pyo3(signature = (references, predictions, max_order=4, smooth=false))]
fn compute(
    references: Vec<Vec<String>>,
    predictions: Vec<String>,
    max_order: usize,
    smooth: bool,
) -> PyResult<PyObject> {
    let bleu = compute_score(references, predictions, max_order, smooth);
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
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
