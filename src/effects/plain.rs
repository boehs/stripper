use palette::rgb::Rgba;

use crate::{Module, Pixels, ModR};

pub struct Off;

impl Module<()> for Off {
    fn new(input: ()) -> Self {
        Self
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        pixels.fill(Rgba::new(0.0, 0.0, 0.0, 0.0));
        ModR::Pixels(pixels.to_owned())
    }
}


impl Module<Rgba> for Rgba {
    fn new(input: Rgba) -> Self {
        input
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        pixels.fill(*self);
        ModR::Pixels(pixels.to_owned())
    }
}
