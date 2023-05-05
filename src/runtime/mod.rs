#[cfg(feature = "wasm")]
pub mod wasm;
pub mod common;

use std::error::Error;

use palette::rgb::Rgba;

use crate::Pixels;

pub trait Runtime<T> {
    fn new(input: T) -> Self where Self: Sized;
    fn display(&self, pixels: Pixels<Rgba>) -> Result<(), Box<dyn Error>>;
}
