use std::collections::VecDeque;

use rand::{Rng, SeedableRng};
use stripper::{
    primitives::color::{Rgba, Mix},
    runtime::{self, wasm::WasmInit, Runtime},
    Module, Pixels,
};
use wasm_bindgen::prelude::wasm_bindgen;

struct Rain {
    drops: VecDeque<(usize, f32)>,
    drop_color: Rgba,
    bg_color: Rgba,
}

trait Weather {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels;
}

impl Weather for Rain {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels {
        let mut canvas = vec![self.bg_color; pixel_size];
        let mut rng = rand::rngs::SmallRng::from_entropy();
        // Every pixel has a 1/10 chance every second to be added
        for i in 0..pixel_size {
            if rng.gen_ratio(1, 25 * 30) {
                self.drops
                    .push_back((i, rng.gen_range(70..=100) as f32 * 0.01))
            }
        }
        for (i, drop) in self.drops.clone().iter().enumerate() {
            canvas[drop.0] = self.bg_color.mix(self.drop_color, drop.1);
            if self.drops[i].1 == 0.00 {
                self.drops.remove(i);
            }
            self.drops[i].1 = self.drops[i].1 - 0.01
        }
        canvas
    }
}

struct WeatherD {
    instance: Box<dyn Weather>,
}

impl Module for WeatherD {
    fn update(input: String) -> Self
    where
        Self: Sized,
    {
        WeatherD {
            instance: Box::new(Rain {
                drops: vec![].into(),
                drop_color: Rgba::new(112.0, 200.0, 230.0, 1.0),
                bg_color: Rgba::new(39.0, 95.0, 115.0, 1.0),
            }),
        }
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> stripper::ModR {
        stripper::ModR::Pixels(self.instance.simulate(i, pixels.len()))
    }
}

#[wasm_bindgen(start)]
fn main() {
    web_sys::console::log_1(&"done".into());
    runtime::common::run(
        Box::new(WeatherD::update("".to_string())),
        Box::new(runtime::wasm::Wasm::new(WasmInit {
            selector: ".pix".to_string(),
        })),
    )
}
