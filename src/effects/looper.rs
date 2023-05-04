use palette::{rgb::Rgba, IntoColor};

use crate::{ModR, Module, Pixels};

pub struct WithResetI<R, F: FnMut(u32, &Pixels<Rgba>) -> ModR<R>> {
    pub fun: F,
}
pub struct WithReset<R, F: FnMut(u32, &Pixels<Rgba>) -> ModR<R>> {
    i: u32,
    f: F,
}

impl<T, F: FnMut(u32, &Pixels<Rgba>) -> ModR<T>> Module<WithResetI<T, F>, T> for WithReset<T, F> {
    fn new(input: WithResetI<T, F>) -> Self {
        WithReset { i: 0, f: input.fun }
    }

    fn render(&mut self, i: u32, pixels: &Pixels<Rgba>) -> ModR<T> {
        loop {
            let res = (self.f)(self.i, pixels);
            self.i = match res {
                ModR::Kill => 0,
                _ => self.i + 1
            };
            if let ModR::Pixels(p) = res {
                break ModR::Pixels(p)
            }
        }
    }
}
