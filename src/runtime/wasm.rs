use core::error::Error;

use super::Runtime;
use wasm_bindgen::prelude::*;
use web_sys::CssStyleDeclaration;
use alloc::{vec::Vec, string::String, boxed::Box};

pub struct Wasm {
    pixels: Vec<CssStyleDeclaration>,
}

pub struct WasmInit {
    pub selector: String,
}

impl Runtime<WasmInit> for Wasm {
    fn new(input: WasmInit) -> Self {
        use web_sys::*;

        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let nodes = document
            .query_selector_all(&input.selector)
            .expect("error with query");

        let mut elements: Vec<CssStyleDeclaration> = vec![];
        for i in 0..nodes.length() {
            let node = nodes.item(i).expect("error getting node");
            if let Some(element) = node.dyn_ref::<HtmlElement>() {
                elements.push(element.clone().style());
            }
        }
        Self { pixels: elements }
    }
    fn display(&mut self, pixels: &crate::Pixels) -> Result<(), Box<dyn Error>> {
        for (i, pixel) in self.pixels.iter().enumerate() {
            let color = pixels[i];
            pixel
                .set_property(
                    "--color",
                    &format!(
                        "{},{},{}",
                        (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8
                    ),
                )
                .expect(&format!("could not set the style for el {}", i));
        }
        Ok(())
    }

    fn get_number_of_pixels(&self) -> u16 {
        self.pixels.len() as u16
    }
}
