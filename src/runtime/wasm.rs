use super::Runtime;
use palette::rgb::Rgba;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Wasm {
    pixels: Vec<HtmlElement>,
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

        let mut elements: Vec<HtmlElement> = vec![];
        for i in 0..nodes.length() {
            let node = nodes.item(i).expect("error getting node");
            if let Some(element) = node.dyn_ref::<HtmlElement>() {
                elements.push(element.clone());
            }
        }
        Self { pixels: elements }
    }
    fn display(&self, pixels: crate::Pixels<Rgba>) -> Result<(), Box<dyn std::error::Error>> {
        for (i, pixel) in self.pixels.iter().enumerate() {
            let color = pixels[i];
            pixel
                .style()
                .set_property(
                    "--color",
                    &format!(
                        "rgba({},{},{},{})",
                        color.red, color.green, color.blue, color.alpha
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
