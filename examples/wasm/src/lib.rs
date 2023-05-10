mod gradient;
mod weather;

use stripper::{
    runtime::{self, wasm::WasmInit, Runtime},
    Module,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use weather::WeatherD;

#[wasm_bindgen(start)]
fn main() {
    let window = web_sys::window().expect("no global `window` exists");
    let params =
        web_sys::UrlSearchParams::new_with_str_sequence_sequence(&window.location().to_string())
            .expect("Failed parsing params");

    if let Some(imp) = params.get("imp") {
        runtime::common::run(
            Box::new(WeatherD::update(imp.clone())),
            Box::new(runtime::wasm::Wasm::new(WasmInit {
                selector: ".pix".to_string(),
            })),
        )
    }
}
