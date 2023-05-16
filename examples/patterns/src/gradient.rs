use core::str::FromStr;

use alloc::vec::Vec;
use enterpolation::{linear::Linear, Curve, DiscreteGenerator, Equidistant, Generator, Identity};
use stripper::{
    primitives::{
        color::{encoding, rgb::Rgb, Alpha, FromColor, Hsl, Rgba, Srgb, Srgba, WithAlpha},
        tween::get_wrapped_from_len,
    },
    ModR, Module, Pixels,
};

#[deprecated(
    note = "#ff0000, #ff9a00, #d0de21, #4fdc4a, #3fdad8, #2fc9e2, #1c7fee, #5f15f2, #ba0cf8, #fb07d9, #ff0000"
)]
/// This module renders a rainbow by
pub struct Rainbow;

#[derive(Clone)]
struct IAmGoingCrazy<T>(pub Vec<T>);
impl<T: Copy> Generator<usize> for IAmGoingCrazy<T> {
    type Output = T;
    fn gen(&self, input: usize) -> Self::Output {
        self.0[input]
    }
}

// Implement DiscreteGenerator for IAmGoingCrazy<T>
impl<T: Copy> DiscreteGenerator for IAmGoingCrazy<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[allow(deprecated)]
impl Module for Rainbow {
    fn update(_input: &str) -> Self
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
#[derive(Clone)]
pub struct Gradient(
    enterpolation::linear::Linear<
        Equidistant<f32>,
        IAmGoingCrazy<Rgb<stripper::primitives::color::encoding::Linear<encoding::Srgb>>>,
        Identity,
    >,
    f64,
);

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
    fn update(input: &str) -> Self
    where
        Self: Sized,
    {
        let base = input.split("|").collect::<Vec<_>>();
        let colors = base[0]
            .split(",")
            .flat_map(|x| Srgb::from_str(x.trim_start()).map(|c| c.into_linear()))
            .collect::<Vec<_>>();
        let len = colors.len();
        let thing = Linear::new_unchecked(
            IAmGoingCrazy(colors),
            Equidistant::<f32>::normalized(len),
            Identity::new(),
        );
        Gradient(
            thing,
            base.get(1)
                .map(|v| f64::from_str(v).unwrap_or(1.0))
                .unwrap_or(1.0),
        )
    }
    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        let rgba_colors: Vec<Rgba> = self
            .0
            .clone()
            .take((pixels.len() as f64 * self.1) as usize)
            .map(|c| Srgba::<f32>::from_linear(c.with_alpha(1.0)))
            .collect();
        // Attempt 1
        /*
        rgba_colors.rotate_right(i as usize % (pixels.len() as f64 * self.1) as usize);
        rgba_colors.truncate(pixels.len());*/
        // Attempt 2
        /*ModR::Pixels(
            rgba_colors
                .into_iter()
                .cycle()
                .skip(i as usize)
                .take(pixels.len())
                .collect::<Vec<_>>(),
        )*/
        // Attempt 3
        ModR::Pixels(get_wrapped_from_len(
            &rgba_colors,
            i as usize % (pixels.len() as f64 * self.1) as usize,
            pixels.len(),
        ))
    }
}
