use smart_leds::{brightness, RGB8};

use crate::{primitives::tween::Easing, ModR, Module, Pixels};

// TODO: Don't assume 100% brightness

pub struct Pulse<T: Easing<f64>> {
    first: u32,
    duration: i16,
    intensity: i8,
    function: T,
}

impl<E: Easing<f64>> Module<Pulse<E>> for Pulse<E> {
    fn new(input: Pulse<E>) -> Self {
        input
    }
    // TODO: I doubt this actually works
    fn render(&self, i: u32, pixels: &Pixels) -> ModR {
        let t = E::ease_in_out(
            (i - self.first).into(),
            0.into(),
            self.intensity.into(),
            self.duration.into(),
        );
        let bright = brightness(pixels.iter().cloned(), 100 as u8 - t as u8);
        ModR::Some(bright.into_iter().map(|x| RGB8::from(x)).collect::<Vec<_>>())
    }
}
