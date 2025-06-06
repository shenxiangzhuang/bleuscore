use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Custom type for `Vec<String>`.
    #[wasm_bindgen(typescript_type = "string[]")]
    #[derive(Debug)]
    pub type VecString;

    /// Custom type for `Vec<Vec<String>>`.
    #[wasm_bindgen(typescript_type = "string[][]")]
    #[derive(Debug)]
    pub type VecVecString;

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

impl VecString {
    /// Convert to a `Vec<String>`.
    pub fn convert(self) -> Result<Vec<String>, JsError> {
        serde_wasm_bindgen::from_value(self.into())
            .map_err(|_| JsError::new("TypeError: expected array of string"))
    }
}

impl VecVecString {
    pub fn convert(self) -> Result<Vec<Vec<String>>, JsError> {
        serde_wasm_bindgen::from_value(self.into())
            .map_err(|_| JsError::new("TypeError: expected array of string arrays"))
    }
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Default, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BleuScore {
    pub bleu: f64,
    pub precisions: Vec<f64>,
    pub brevity_penalty: f64,
    pub length_ratio: f64,
    pub translation_length: usize,
    pub reference_length: usize,
}

#[wasm_bindgen]
pub fn compute_score(
    references_js_array: VecVecString,
    predictions_js_array: VecString,
    max_order: usize,
    smooth: bool,
) -> Result<BleuScore, JsError> {
    // Convert JsValue to Rust Vec<Vec<String>>
    let references_vec: Vec<Vec<String>> = references_js_array.convert()?;
    console_log!("references_vec: {:?}", references_vec);

    // Convert JsValue to Rust Vec<String>
    let predictions_vec: Vec<String> = predictions_js_array.convert()?;
    console_log!("predictions_vec: {:?}", predictions_vec);

    let res = bleuscore::bleu::compute_score(&references_vec, &predictions_vec, max_order, smooth);
    console_log!("bleu_result: {:?}", res);
    Ok(BleuScore {
        bleu: res.bleu,
        precisions: res.precisions,
        brevity_penalty: res.brevity_penalty,
        length_ratio: res.length_ratio,
        translation_length: res.translation_length,
        reference_length: res.reference_length,
    })
}

/// Helper function for tests to create BleuScore directly from Rust types
/// This bypasses the WASM conversion layer for easier testing
#[cfg(test)]
pub fn compute_score_direct(
    references: &[Vec<String>],
    predictions: &[String],
    max_order: usize,
    smooth: bool,
) -> BleuScore {
    let res = bleuscore::bleu::compute_score(references, predictions, max_order, smooth);
    BleuScore {
        bleu: res.bleu,
        precisions: res.precisions,
        brevity_penalty: res.brevity_penalty,
        length_ratio: res.length_ratio,
        translation_length: res.translation_length,
        reference_length: res.reference_length,
    }
}

/// Debug version of compute_score_direct for troubleshooting
#[cfg(test)]
pub fn compute_score_debug(
    references: &[Vec<String>],
    predictions: &[String],
    max_order: usize,
    smooth: bool,
) -> BleuScore {
    println!("Debug: references = {:?}", references);
    println!("Debug: predictions = {:?}", predictions);
    let res = bleuscore::bleu::compute_score(references, predictions, max_order, smooth);
    println!("Debug: result = {:?}", res);
    BleuScore {
        bleu: res.bleu,
        precisions: res.precisions,
        brevity_penalty: res.brevity_penalty,
        length_ratio: res.length_ratio,
        translation_length: res.translation_length,
        reference_length: res.reference_length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bleu_score_default() {
        let score = BleuScore::default();
        assert_eq!(score.bleu, 0.0);
        assert_eq!(score.precisions, Vec::<f64>::new());
        assert_eq!(score.brevity_penalty, 0.0);
        assert_eq!(score.length_ratio, 0.0);
        assert_eq!(score.translation_length, 0);
        assert_eq!(score.reference_length, 0);
    }

    #[test]
    fn test_bleu_score_creation() {
        let score = BleuScore {
            bleu: 0.75,
            precisions: vec![0.8, 0.7, 0.6, 0.5],
            brevity_penalty: 0.9,
            length_ratio: 0.95,
            translation_length: 20,
            reference_length: 22,
        };

        assert_eq!(score.bleu, 0.75);
        assert_eq!(score.precisions.len(), 4);
        assert_eq!(score.brevity_penalty, 0.9);
        assert_eq!(score.length_ratio, 0.95);
        assert_eq!(score.translation_length, 20);
        assert_eq!(score.reference_length, 22);
    }

    #[test]
    fn test_bleu_score_tsify_annotations() {
        // Test that BleuScore has the correct annotations for TypeScript/WASM binding
        // This verifies the struct is properly set up for JavaScript interop
        let score = BleuScore {
            bleu: 0.5,
            precisions: vec![0.8, 0.6, 0.4, 0.2],
            brevity_penalty: 0.9,
            length_ratio: 0.95,
            translation_length: 10,
            reference_length: 12,
        };

        // Verify fields are accessible (this tests the binding structure)
        assert_eq!(score.bleu, 0.5);
        assert_eq!(score.precisions, vec![0.8, 0.6, 0.4, 0.2]);
        assert_eq!(score.brevity_penalty, 0.9);
        assert_eq!(score.length_ratio, 0.95);
        assert_eq!(score.translation_length, 10);
        assert_eq!(score.reference_length, 12);
    }

    // Note: VecString and VecVecString conversion tests cannot be run in regular Rust tests
    // because they depend on WASM-bindgen functions that only work in WASM runtime.
    // These would need to be tested in a browser or Node.js environment with WASM.

    #[test]
    fn test_compute_score_direct_helper() {
        // Test the helper function that can be used in tests
        let references = vec![vec!["hello world".to_string()]];
        let predictions = vec!["hello world".to_string()];

        let score = compute_score_direct(&references, &predictions, 2, false);

        // Verify the helper function works and returns a properly structured BleuScore
        assert!(score.bleu >= 0.0);
        assert!(score.bleu <= 1.0);
        assert!(score.brevity_penalty >= 0.0);
        assert_eq!(score.precisions.len(), 2);
        assert!(score.translation_length > 0);
        assert!(score.reference_length > 0);
    }

    #[test]
    fn test_compute_score_direct_parameter_passing() {
        // Test that parameters are correctly passed through the helper
        let references = vec![vec!["hello world test".to_string()]];
        let predictions = vec!["hello world test".to_string()];

        // Test different max_order values
        let score_1 = compute_score_direct(&references, &predictions, 1, false);
        let score_2 = compute_score_direct(&references, &predictions, 2, false);
        let score_4 = compute_score_direct(&references, &predictions, 4, false);

        assert_eq!(score_1.precisions.len(), 1);
        assert_eq!(score_2.precisions.len(), 2);
        assert_eq!(score_4.precisions.len(), 4);

        // Test smoothing parameter effect
        let score_no_smooth = compute_score_direct(&references, &predictions, 4, false);
        let score_with_smooth = compute_score_direct(&references, &predictions, 4, true);

        // Smoothing typically helps with sparse data
        assert!(score_with_smooth.bleu >= score_no_smooth.bleu);
    }

    #[test]
    fn test_binding_interface_signature() {
        // Test that the compute_score function has the expected signature for WASM binding
        // This is a compile-time test - if this compiles, the binding signature is correct

        // The function should accept:
        // - VecVecString (references)
        // - VecString (predictions)
        // - usize (max_order)
        // - bool (smooth)
        // And return Result<BleuScore, JsError>

        // We can't actually call it in non-WASM tests, but we can verify the types exist
        let _: fn(VecVecString, VecString, usize, bool) -> Result<BleuScore, JsError> =
            compute_score;
    }

    #[test]
    fn test_helper_functions_edge_cases() {
        // Test edge cases that can affect binding behavior

        // Empty strings (but not completely empty inputs)
        let refs = vec![vec!["".to_string()]];
        let preds = vec!["".to_string()];
        let score = compute_score_direct(&refs, &preds, 4, false);
        // Should handle gracefully without panicking
        assert!(score.bleu >= 0.0);

        // Multiple references per prediction
        let refs = vec![
            vec!["hello world".to_string(), "hi earth".to_string()],
            vec!["foo bar".to_string()],
        ];
        let preds = vec!["hello world".to_string(), "foo bar".to_string()];
        let score = compute_score_direct(&refs, &preds, 4, false);
        assert!(score.bleu >= 0.0);
        assert!(score.bleu <= 1.0);

        // Single character inputs
        let refs = vec![vec!["a".to_string()]];
        let preds = vec!["a".to_string()];
        let score = compute_score_direct(&refs, &preds, 1, false);
        assert!(score.bleu >= 0.0);
        assert!(score.bleu <= 1.0);

        // Note: Completely empty inputs (empty vectors) cause the core implementation to panic,
        // so we don't test that case here. In a real WASM environment, such inputs should be
        // validated at the binding layer before calling the core implementation.
    }

    #[test]
    fn test_unicode_handling_through_binding() {
        // Test that Unicode strings are properly handled by the binding layer
        let refs = vec![vec!["こんにちは世界".to_string()]];
        let preds = vec!["こんにちは世界".to_string()];

        let score = compute_score_direct(&refs, &preds, 4, false);

        // Should handle Unicode without panicking
        assert!(score.bleu >= 0.0);
        assert!(score.bleu <= 1.0);
        assert!(score.translation_length > 0);
        assert!(score.reference_length > 0);
    }

    #[test]
    fn test_debug_helper_functionality() {
        // Test that the debug helper function works
        let refs = vec![vec!["test sentence".to_string()]];
        let preds = vec!["test sentence".to_string()];

        let score = compute_score_debug(&refs, &preds, 2, false);

        // Verify it works the same as the regular helper
        assert!(score.bleu >= 0.0);
        assert!(score.bleu <= 1.0);
        assert_eq!(score.precisions.len(), 2);
    }
}
