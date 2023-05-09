use std::str::FromStr;

use enterpolation::{
    linear::ConstEquidistantLinear,
    Curve,
};
use stripper::{
    primitives::color::{
        rgb::Rgb,
        Alpha, FromColor, Hsl, LinSrgb, Rgba, Srgb, Srgba, WithAlpha,
    },
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

fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

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
        let thing =
            ConstEquidistantLinear::<f32, _, 2>::equidistant_unchecked(vec_to_arr(self.0.clone()));
        let rgba_colors: Vec<Rgba> = thing
            .take(pixels.len())
            .map(|c| Srgba::<f32>::from_linear(c.with_alpha(1.0)))
            .map(|c| Srgba::new(c.red * 255.0, c.green * 255.0, c.blue * 255.0, 1.0))
            .collect();
        ModR::Pixels(rgba_colors)
    }
}
