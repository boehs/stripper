mod gradient;
mod weather;

use std::rc::Rc;

use stripper::{
    runtime::{self, wasm::WasmInit, Runtime},
    Module,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn main() {
    let window = web_sys::window().expect("no global `window` exists");
    let params = web_sys::UrlSearchParams::new_with_str_sequence_sequence(
        &window
            .location()
            .search()
            .expect("No search")
            .to_string()
            .into(),
    )
    .expect("Failed parsing params");

    if let Some(imp) = params.get("imp") {
        if let Some(idx) = params.get("idx") {
            if let Ok(i) = usize::from_str_radix(&idx, 10) {
                // Todo: How to support unconstructed varients
                let v: Vec<Rc<dyn Module>> = vec![
                    Rc::new(gradient::Gradient::update(imp.clone())),
                    Rc::new(weather::WeatherD::update(imp.clone())),
                ];
                if let Some(modt) = v.get(i) {
                    runtime::common::run(
                        modt.clone(),
                        Box::new(runtime::wasm::Wasm::new(WasmInit {
                            selector: ".pix".to_string(),
                        })),
                    )
                }
            }
        }
    }
}
