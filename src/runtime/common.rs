use palette::rgb::Rgba;

use crate::{Module, Pixels};

use super::Runtime;

enum Status {
    Active,
    Dead,
}

struct ModDef<I, C: Module<I>> {
    module: C,
    config: I,
    status: Status,
}

pub trait ModDefTrait {
    fn set_status(&mut self, status: Status);
    fn get_status(&self) -> Status;
}

impl<I, C: Module<I>> ModDefTrait for ModDef<I, C> {
    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
    fn get_status(&self) -> Status {
        self.status
    }
}

pub fn run<T>(modules: Vec<Box<&mut dyn ModDefTrait>>, runtime: Box<dyn Runtime<T>>) {
    let mut pixels: Pixels<Rgba> =
        Vec::with_capacity(runtime.get_number_of_pixels()).fill(Rgba::new(0, 0, 0, 0));
    runtime.display(pixels);

    modules[0].as_ref().set_status(Status::Active);
    let first_active = modules.iter().find(|x| x.get_status() == Status::Active);
    if let Some(active) = first_active {
        loop {

        };
    }
}
