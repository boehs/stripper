use stripper::{
    primitives::color::{FromColor, Hsla, Rgba, Srgba, LinSrgba, IntoColor, rgb::Rgb, Hsl, Srgb},
    ModR, Module, Pixels,
};

pub struct Rainbow;

impl Module for Rainbow {
    fn update(input: String) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        let steps = 360.0 / pixels.len() as f32;
        web_sys::console::log_1(&steps.into());
        let grad = (0..pixels.len())
            .map(|x| {
                let c = Hsl::new((steps * x as f32) as f32, 1.0, 0.5);
                let b = Rgba::from_color(c).into_format();
                b
            })
            .collect();
        ModR::Pixels(grad)
    }
}
