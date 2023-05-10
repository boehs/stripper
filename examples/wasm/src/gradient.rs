use std::str::FromStr;

use enterpolation::{linear::Linear, Curve, Equidistant, Identity};
use stripper::{
    primitives::color::{rgb::Rgb, Alpha, FromColor, Hsl, LinSrgb, Rgba, Srgb, Srgba, WithAlpha},
    ModR, Module, Pixels,
};

#[deprecated(
    note = "#ff0000, #ff9a00, #d0de21, #4fdc4a, #3fdad8, #2fc9e2, #1c7fee, #5f15f2, #ba0cf8, #fb07d9, #ff0000"
)]
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

pub struct Gradient(Vec<LinSrgb>, f64);

impl Module for Gradient {
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
