mod weather;
mod gradient;

use wasm_bindgen::prelude::wasm_bindgen;
use stripper::{runtime::{self, wasm::WasmInit, Runtime}, Module};
use weather::WeatherD;

#[wasm_bindgen(start)]
fn main() {
    runtime::common::run(
        Box::new(gradient::Gradient::update("#f222ad,#2253f2".to_string())),
        Box::new(runtime::wasm::Wasm::new(WasmInit {
            selector: ".pix".to_string(),
        })),
    )
}
