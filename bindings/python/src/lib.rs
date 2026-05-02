use bleuscore::bleu::{compute_score, RefLenMethod};
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

/// Resolve `ref_len_method` string (including aliases) to `RefLenMethod`.
///
/// Accepted values:
/// - `"shortest"` / `"hf"` → [`RefLenMethod::Shortest`]
///   (HuggingFace evaluate / TF NMT compatible)
/// - `"closest"` / `"sacrebleu"` → [`RefLenMethod::Closest`]
///   (SacreBLEU / NIST mteval-v13a compatible)
fn parse_ref_len_method(value: &str) -> PyResult<RefLenMethod> {
    match value {
        "shortest" | "hf" => Ok(RefLenMethod::Shortest),
        "closest" | "sacrebleu" => Ok(RefLenMethod::Closest),
        other => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unknown ref_len_method {other:?}. \
             Valid values: \"shortest\", \"hf\", \"closest\", \"sacrebleu\"."
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (references, predictions, max_order=4, smooth=false, ref_len_method="shortest"))]
fn compute(
    references: Vec<Vec<String>>,
    predictions: Vec<String>,
    max_order: usize,
    smooth: bool,
    ref_len_method: &str,
) -> PyResult<Py<PyAny>> {
    let method = parse_ref_len_method(ref_len_method)?;
    let bleu = compute_score(&references, &predictions, max_order, smooth, method);
    Python::attach(|py| {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ref_len_method_shortest_variants() {
        assert_eq!(parse_ref_len_method("shortest").unwrap(), RefLenMethod::Shortest);
        assert_eq!(parse_ref_len_method("hf").unwrap(), RefLenMethod::Shortest);
    }

    #[test]
    fn test_parse_ref_len_method_closest_variants() {
        assert_eq!(parse_ref_len_method("closest").unwrap(), RefLenMethod::Closest);
        assert_eq!(parse_ref_len_method("sacrebleu").unwrap(), RefLenMethod::Closest);
    }

    #[test]
    fn test_parse_ref_len_method_invalid() {
        assert!(parse_ref_len_method("unknown").is_err());
        assert!(parse_ref_len_method("").is_err());
    }
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
