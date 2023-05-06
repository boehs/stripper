use crate::{ModR, Module, Pixels};

pub struct WithResetI<F: FnMut(u32, &Pixels) -> ModR> {
    pub fun: F,
}
pub struct WithReset<F: FnMut(u32, &Pixels) -> ModR> {
    i: u32,
    f: F,
}

impl<F: FnMut(u32, &Pixels) -> ModR> Module<WithResetI<F>> for WithReset<F> {
    fn new(input: WithResetI<F>) -> Self {
        WithReset { i: 0, f: input.fun }
    }

    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
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
