use sciolyff::interpreter::Interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_results(sciolyff: &str, hide_raw: bool, color: &str) -> String {
    Interpreter::from_yaml(sciolyff).to_html(hide_raw, color)
}
