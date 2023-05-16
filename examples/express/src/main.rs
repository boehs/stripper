#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use circuit_playground_express as bsp;
use bsp::{hal::prelude::*, entry};
use stripper::{runtime::{express::Express, Runtime}, Module, ModR, primitives::color::Rgba};

#[entry]
fn main() -> ! {
    let mut runtime = Express::new(());
    let mut grad = patterns::Gradient::update("FFD700,FF69B4,00CED1,FF69B4,FFD700|5");
    let pixels = vec![Rgba::new(0.0, 0.0, 0.0, 0.0); runtime.get_number_of_pixels().into()];

    let mut i = 0;

    loop {
        let render = grad.render(i, &pixels);
        match render {
            ModR::Pixels(pix) => runtime.display(&pix),
            _ => Ok(())
        };
        i+=1;
        runtime.timer.delay_ms(30u8);
    };
}
