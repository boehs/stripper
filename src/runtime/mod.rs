#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "pico")]
pub mod pico;
pub mod express;

use core::error::Error;
use alloc::boxed::Box;
use palette::{Alpha, rgb::Rgb};
use smart_leds::{RGB, RGBA};
use crate::Pixels;

pub trait Runtime<T> {
    fn new(input: T) -> Self where Self: Sized;
    fn display(&mut self, pixels: &Pixels) -> Result<(), Box<dyn Error>>;
    fn get_number_of_pixels(&self) -> u16;
}
