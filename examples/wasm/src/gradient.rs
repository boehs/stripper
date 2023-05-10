use std::str::FromStr;

use enterpolation::{
    linear::Linear,
    Curve, Equidistant, Identity,
};
use stripper::{
    primitives::color::{rgb::Rgb, Alpha, FromColor, Hsl, LinSrgb, Rgba, Srgb, Srgba, WithAlpha},
    ModR, Module, Pixels,
};

pub struct Rainbow;

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

pub struct Gradient(Vec<LinSrgb>);

impl Module for Gradient {
    fn update(input: String) -> Self
    where
        Self: Sized,
    {
        let colors = input
            .split(",")
            .flat_map(|x| Srgb::from_str(x.trim_start()).map(|c| c.into_linear()))
            .collect::<Vec<_>>();
        Gradient(colors)
    }
    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        // Better solution! https://github.com/NicolasKlenert/enterpolation/discussions/22
        let thing = Linear::new_unchecked(
            self.0.clone(),
            Equidistant::<f32>::normalized(self.0.len()),
            Identity::new(),
        );
        let mut rgba_colors: Vec<Rgba> = thing
            .take(pixels.len())
            .map(|c| Srgba::<f32>::from_linear(c.with_alpha(1.0)))
            .map(|c| Srgba::new(c.red * 255.0, c.green * 255.0, c.blue * 255.0, 1.0))
            .collect();
        rgba_colors.rotate_right(i as usize);
        ModR::Pixels(rgba_colors)
    }
}
