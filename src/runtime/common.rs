use palette::rgb::Rgba;

use crate::{Module, Pixels, ModR};

use super::Runtime;

pub fn run<T>(mut modules: Box<dyn Module>, runtime: Box<dyn Runtime<T>>) {
    let mut pixels: Pixels = vec![Rgba::new(0.0, 0.0, 0.0, 0.0); runtime.get_number_of_pixels().into()];
    runtime.display(&pixels);

    let mut i = 0;
    loop {
        if let ModR::Pixels(pix) = modules.as_mut().render(i, &pixels) {
            runtime.display(&pix);
        }
        i = i+1
    }
}
