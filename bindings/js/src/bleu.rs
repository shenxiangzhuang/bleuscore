use wasm_bindgen::prelude::*;
use js_sys::{Array, JsString, Object};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub fn compute_score(
    references: &JsValue,
    predictions: &JsValue,
    max_order: usize,
    smooth: bool,
) -> Result<JsValue, JsValue> {
    // Convert JsValue to Rust Vec<Vec<String>>
    let references_js_array = Array::from(references);
    let mut references_vec: Vec<Vec<String>> = Vec::new();

    for i in 0..references_js_array.length() {
        let inner_js_array = Array::from(&references_js_array.get(i));
        let mut inner_vec: Vec<String> = Vec::new();

        for j in 0..inner_js_array.length() {
            let js_str = inner_js_array.get(j);
            let rust_str = js_str.as_string().unwrap_or_default();
            inner_vec.push(rust_str);
        }
        references_vec.push(inner_vec);
    }
    console_log!("references_vec: {:?}", references_vec);

    // Convert JsValue to Rust Vec<String>
    let predictions_js_array = Array::from(predictions);
    let mut predictions_vec: Vec<String> = Vec::new();
    for i in 0..predictions_js_array.length() {
        predictions_vec.push(predictions_js_array.get(i).as_string().unwrap_or_default());
    }
    console_log!("predictions_vec: {:?}", predictions_vec);

    let bleu_result = bleuscore::bleu::compute_score(&references_vec, &predictions_vec, max_order, smooth);
    console_log!("bleu_result: {:?}", bleu_result);
    
    let result = Object::new();
    js_sys::Reflect::set(&result, &"bleu".into(), &bleu_result.bleu.into())?;
    js_sys::Reflect::set(&result, &"length_ratio".into(), &bleu_result.length_ratio.into())?;

    Ok(result.into())
}