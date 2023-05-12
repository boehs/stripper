use core::str::FromStr;

use alloc::{string::String, vec::Vec};
use enterpolation::{linear::Linear, Equidistant, Identity, Curve};
use stripper::{
    primitives::color::{rgb::Rgb, Alpha, FromColor, Hsl, LinSrgb, Rgba, Srgb, Srgba, WithAlpha},
    ModR, Module, Pixels,
};

#[deprecated(
    note = "#ff0000, #ff9a00, #d0de21, #4fdc4a, #3fdad8, #2fc9e2, #1c7fee, #5f15f2, #ba0cf8, #fb07d9, #ff0000"
)]
/// This module renders a rainbow by
pub struct Rainbow;

#[allow(deprecated)]
impl Module for Rainbow {
    fn update(_input: String) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        let steps = 360.0 / pixels.len() as f32;
        let grad = (0..pixels.len())
            .map(|x| {
                let c = Hsl::new_srgb(((steps * x as f32) + i as f32) % 360.0, 1.0, 0.5);
                let b: Alpha<Rgb, f32> = Rgba::from_color(c).into_format();
                // I don't like this. Find better way.
                Srgba::new(b.red * 255.0, b.green * 255.0, b.blue * 255.0, b.alpha)
            })
            .collect();
        ModR::Pixels(grad)
    }
}

/// A gradient is linear interpolation between a series of colors.
/// `x`, equally spaced samples are taken from the gradient. The values are rotated by `i % x` (to move the gradient one right every frame).
/// By default, `x = pixels.len()`. A multiplier (self.1 >= 1.0) can be applied to `x` "spread the gradient out"
///
/// When the gradient is spread out, only part of it will be in view.
/// For instance, a `2.0` multiplier means that only 1/2 of the gradient will be visible at a time
///
/// Spreading the gradient out will make a loop take longer, but that is because it also "zooms in".
/// This is probably what you want.
///
/// To slow the gradient down (without changing the view), change the framerate.
///
/// The first `pixels.len()` of the final pixels are visible.
pub struct Gradient(Vec<LinSrgb>, f64);

impl Module for Gradient {
    /// Comma seperated hex values, 3 or 6 letters, # optional, whitespace optional.
    ///
    /// An optional multiplier (any string that can be parsed to a f64).
    /// If a multiplier is provided, a `|` should precede it.
    ///
    /// # Examples
    ///
    /// ```
    /// Gradient::update("FFD700,FF69B4,00CED1,FF69B4,FFD700|5".to_string())
    /// Gradient::update("#fff,#420,#069".to_string())
    /// ```
    fn update(input: String) -> Self
    where
        Self: Sized,
    {
        let base = input.split("|").collect::<Vec<_>>();
        let colors = base[0]
            .split(",")
            .flat_map(|x| Srgb::from_str(x.trim_start()).map(|c| c.into_linear()))
            .collect::<Vec<_>>();
        Gradient(colors,base.get(1).map(|v| f64::from_str(v).unwrap_or(1.0)).unwrap_or(1.0))
    }
    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        // Better solution! https://github.com/NicolasKlenert/enterpolation/discussions/22
        // Move this to update
        let thing = Linear::new_unchecked(
            self.0.clone(),
            Equidistant::<f32>::normalized(self.0.len()),
            Identity::new(),
        );
        let mut rgba_colors: Vec<Rgba> = thing
            .take((pixels.len() as f64 * self.1) as usize)
            .map(|c| Srgba::<f32>::from_linear(c.with_alpha(1.0)))
            .map(|c| Srgba::new(c.red * 255.0, c.green * 255.0, c.blue * 255.0, 1.0))
            .collect();
        rgba_colors.rotate_right(i as usize % (pixels.len() as f64 * self.1) as usize);
        rgba_colors.truncate(pixels.len());
        ModR::Pixels(rgba_colors)
    }
}
