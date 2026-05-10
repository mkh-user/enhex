use wasm_bindgen::prelude::*;
use crate::compile as core_compile;

/// Compile an EnhEx pattern string to a Regex string.
/// Returns the regex string or throws an error.
#[wasm_bindgen]
pub fn compile(pattern: &str) -> Result<String, JsValue> {
    core_compile(pattern)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
