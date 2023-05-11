#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

mod gradient;
mod weather;

use std::{rc::Rc};

use embassy_time::{Duration, Ticker};
use stripper::{
    runtime::{wasm::{WasmInit, Wasm}, Runtime},
    Module, Pixels, primitives::color::Rgba, ModR,
};
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
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
                let mut v: Vec<Rc<dyn Module>> = vec![
                    Rc::new(gradient::Gradient::update(imp.clone())),
                    Rc::new(weather::WeatherD::update(imp.clone())),
                ];
                if let Some(modt) = v.get_mut(i) {
                    let runtime = Wasm::new(WasmInit { selector: ".pix".to_string() });
                    let pixels: Pixels = vec![Rgba::new(0.0, 0.0, 0.0, 0.0); runtime.get_number_of_pixels().into()];
                    runtime.display(&pixels).expect("Error setting initial view");
                    let mut i = 0;
                    let mut loop_regulater = Ticker::every(Duration::from_millis(30));
                    loop {
                        let render = Rc::get_mut(modt).expect("This went poorly").render(i.try_into().unwrap(), &pixels);
                        match render {
                            ModR::Pixels(pix) => runtime.display(&pix),
                            _ => Ok(())
                        };
                        i+=1;
                        loop_regulater.next().await
                    };
                }
            }
        }
    }
}
