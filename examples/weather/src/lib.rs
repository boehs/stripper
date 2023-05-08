use std::collections::VecDeque;

use rand::{Rng, SeedableRng};
use stripper::{
    primitives::color::{Mix, Rgba},
    runtime::{self, wasm::WasmInit, Runtime},
    Module, Pixels,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(PartialEq)]
enum Timeframe {
    Never,
    Sometimes,
}

struct Rain {
    drops: VecDeque<(usize, f32)>,
    drop_color: Rgba,
    bg_color: Rgba,
    lightning: Timeframe,
}

trait Weather {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels;
}

impl Weather for Rain {
    fn simulate(&mut self, i: u32, pixel_size: usize) -> Pixels {
        let mut canvas = vec![self.bg_color; pixel_size];
        let mut rng = rand::rngs::SmallRng::from_entropy();
        if i % 3 == 0 {
            self.drops.push_back((
                rng.gen_range(0..pixel_size),
                rng.gen_range(70..=100) as f32 * 0.01,
            ));
        }
        for (i, drop) in self.drops.clone().iter().enumerate() {
            canvas[drop.0] = self.bg_color.mix(self.drop_color, drop.1);
            if self.drops[i].1 == 0.00 {
                self.drops.remove(i);
            }
            self.drops[i].1 = self.drops[i].1 - 0.02
        }
        if (i % 70) > 19 && self.lightning == Timeframe::Sometimes {
            canvas = canvas.iter().map(|&p| {
                p.mix(
                    Rgba::new(255.0, 255.0, 255.0, 1.0),
                    1.0 - (0.05 * ((i % 70) - 19) as f32),
                )
            }).collect::<Vec<_>>();
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
                drop_color: Rgba::new(107.0, 137.0, 148.0, 1.0),
                bg_color: Rgba::new(62.0, 76.0, 93.0, 1.0),
                lightning: Timeframe::Sometimes
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
