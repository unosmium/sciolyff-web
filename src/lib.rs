use sciolyff::interpreter::{html::HTMLOptions, Interpreter};
use std::default::Default;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_results(sciolyff: &str, hide_raw: bool, color: &str) -> String {
    Interpreter::from_yaml(sciolyff).to_html(&HTMLOptions {
        hide_raw,
        color: color.to_string(),
        ..Default::default()
    })
}
