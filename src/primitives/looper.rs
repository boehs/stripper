use crate::{Pixels, ModR};


/// Run function until timeout
pub struct WithTimeout<'a>(i16, u32, &'a dyn Fn(u32, &Pixels) -> ModR);

impl<'a> WithTimeout<'a> {
    pub fn new(max: i16, fun: &'a dyn Fn(u32, &Pixels) -> ModR) -> Self {
        Self(max, 0, fun)
    }

    pub fn run(&mut self, pixels: &Pixels) -> ModR {
        let res = self.2(self.1, pixels);
        let nr = match res {
            ModR::Pixels(pix) => ModR::Pixels(pix),
            _ => {
                self.1 += 1;
                self.2(self.1, pixels)
            }
        };
        self.1 += 1;
        nr
    }
}

// Forever (ignores kill signals, restarts to original input

// Runs a number of times, ignoring kill signals
