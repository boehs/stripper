#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{rc::Rc, string::ToString, vec::Vec};
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use stripper::{
    primitives::color::Rgba,
    runtime::{
        wasm::{Wasm, WasmInit},
        Runtime,
    },
    ModR, Module,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let window = web_sys::window().expect("no global `window` exists");
    let params = web_sys::UrlSearchParams::new_with_str_sequence_sequence(
        &window.location().search().expect("No search").into(),
    )
    .expect("Failed parsing params");

    if let Some(imp) = params.get("imp") {
        if let Some(idx) = params.get("idx") {
            if let Ok(i) = usize::from_str_radix(&idx, 10) {
                let mut runtime = Wasm::new(WasmInit {
                    selector: ".pix".to_string(),
                });
                let pixels =
                    vec![Rgba::new(0.0, 0.0, 0.0, 0.0); runtime.get_number_of_pixels().into()];
                // Todo: How to support unconstructed varients
                let mut v: Vec<Rc<dyn Module>> = vec![
                    Rc::new(patterns::Gradient::update(&imp, pixels.len())),
                    Rc::new(patterns::Weather::update(&imp, pixels.len())),
                ];
                if let Some(modt) = v.get_mut(i) {
                    runtime
                        .display(&pixels)
                        .expect("Error setting initial view");
                    let mut i = 0;
                    let mut loop_regulater = Ticker::every(Duration::from_millis(30));
                    loop {
                        let render = Rc::get_mut(modt)
                            .expect("This went poorly")
                            .render(i.try_into().unwrap(), &pixels);
                        match render {
                            ModR::Pixels(pix) => runtime.display(&pix),
                            _ => Ok(()),
                        };
                        i += 1;
                        loop_regulater.next().await
                    }
                }
            }
        }
    }
}
