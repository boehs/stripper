#![no_std]

#[macro_use]
extern crate alloc;

mod gradient;
mod weather;

pub use gradient::Gradient;
pub use weather::WeatherD as Weather;
