#[cfg(feature = "wasm")]
pub mod wasm;
pub mod common;

use std::error::Error;

use crate::Pixels;

pub trait Runtime<T> {
    fn new(input: T) -> Self where Self: Sized;
    fn display(&self, pixels: &Pixels) -> Result<(), Box<dyn Error>>;
    fn get_number_of_pixels(&self) -> u16;
}
