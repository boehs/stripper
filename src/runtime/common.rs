use crate::Module;

use super::Runtime;

enum Status {
    Active,
    Dead
}

struct ModDef<I, C: Module<I>> {
    module: C,
    config: I,
    status: Status
}

pub trait ModDefTrait {}

impl<I, C: Module<I>> ModDefTrait for ModDef<I, C> {}

pub fn run<T>(modules: Vec<Box<dyn ModDefTrait>>, runtime: Box<dyn Runtime<T>>) {
    runtime.display(pixels)
}
