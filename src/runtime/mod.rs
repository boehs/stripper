#[cfg(feature = "wasm")]
pub mod wasm;
pub mod pico;

use core::error::Error;
use alloc::boxed::Box;
use crate::Pixels;

pub trait Runtime<T> {
    fn new(input: T) -> Self where Self: Sized;
    fn display(&mut self, pixels: &Pixels) -> Result<(), Box<dyn Error>>;
    fn get_number_of_pixels(&self) -> u16;
}
