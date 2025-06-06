use wasm_bindgen::prelude::*;
use tsify::Tsify;
use serde::{Serialize, Deserialize};

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
    Ok(BleuScore{
        bleu: res.bleu,
        precisions: res.precisions,
        brevity_penalty: res.brevity_penalty,
        length_ratio: res.length_ratio,
        translation_length: res.translation_length,
        reference_length: res.reference_length,
    })
}