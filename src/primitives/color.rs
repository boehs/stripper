use std::slice::SliceIndex;

pub use palette::{rgb::Rgba, *};

use crate::Pixels;

pub fn ripple(point: u16, color: Rgba, dist: i8, pixels: &mut Pixels) {
    for i in -dist..dist {
        if let Some(pixel) = pixels.get(usize::from((point + i as u16) as u8)) {
            pixels[usize::from(point + i as u16)] =
                pixel.mix(color, 1.0 - ((1.0 / dist as f32) * i.abs() as f32));
        }
    }
}
