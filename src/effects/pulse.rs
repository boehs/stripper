use palette::rgb::Rgba;

use crate::{primitives::tween::Easing, ModR, Module, Pixels};

// TODO: Don't assume 100% brightness

/// Pulses brightness of input pixels.
pub struct Pulse<T: Easing<f64>> {
    /// The index at time of ::new call
    pub first: u32,
    /// How long a completion will take
    pub duration: u32,
    /// The minimum value
    pub intensity: f64,
    /// The easing function
    pub function: T,
}

impl<E: Easing<f64>> Module<Pulse<E>> for Pulse<E> {
    fn new(input: Pulse<E>) -> Self {
        input
    }
    // TODO: I doubt this actually works
    fn render(&self, i: u32, pixels: &Pixels<Rgba>) -> ModR<Rgba> {
        if i - self.first >= self.duration {
            return ModR::Kill;
        }
        // TODO: Move this to state for perf
        let t = E::ease_in_out(
            (i - self.first).into(),
            0.into(),
            self.intensity,
            self.duration.into(),
        );
        let bright = pixels
            .into_iter()
            .map(|x| {
                let mut y = x.to_owned();
                y.alpha = 1 as f32 - t as f32;
                y
            })
            .collect::<Vec<_>>();
        ModR::Pixels(bright)
    }
}

/// Same as pulse, but also goes out
pub struct Beat<T: Easing<f64>> {
    pub first: u32,
    pub duration: u32,
    pub intensity: f64,
    pub function: T,
    // delay
}

impl<E: Easing<f64>> Module<Beat<E>> for Beat<E> {
    fn new(input: Beat<E>) -> Self {
        input
    }

    fn render(&self, i: u32, pixels: &Pixels<Rgba>) -> ModR<Rgba> {
        if i - self.first >= self.duration * 2 {
            return ModR::Kill;
        }
        let t = E::ease_in_out(
            ((i - self.first) % self.duration).into(),
            0.into(),
            self.intensity,
            self.duration.into(),
        );
        let bright = pixels
            .into_iter()
            .map(|x| {
                let mut y = x.to_owned();
                if i - self.first > self.duration {
                    y.alpha = 1 as f32 - t as f32;
                } else {
                    y.alpha = (1 as f32 - self.intensity as f32) + t as f32
                }
                y
            })
            .collect::<Vec<_>>();
        ModR::Pixels(bright)
    }
}
