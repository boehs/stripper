use palette::rgb::Rgba;

use crate::{Module, Pixels, ModR};

use super::Runtime;

enum Status {
    Active,
    Dead,
}

trait ModuleObject {
    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR;
    fn get_config<C>(&mut self) -> C;
    fn clone_with_config<C>(&self, config: C) -> Box<dyn ModuleObject>;
}

impl<C, M: Module<C>> ModuleObject for (M, C) {
    fn render(&mut self, i: u32, pixels: &Pixels) -> ModR {
        self.0.render(i, pixels)
    }
    fn get_config(&mut self) -> C {
        self.1
    }
    fn clone_with_config(&self, config: C) -> Box<dyn ModuleObject> {
        Box::new((self.0.new(config), config))
    }
}

pub fn run<T>(modules: Vec<Box<&mut dyn ModuleObject>>, runtime: Box<dyn Runtime<T>>) {
    let mut pixels: Pixels =
        Vec::with_capacity(runtime.get_number_of_pixels()).fill(Rgba::new(0, 0, 0, 0));
    runtime.display(pixels);

    modules[0].as_ref().set_status(Status::Active);
    let first_active = modules.first();
    if let Some(active) = first_active {
        loop {
            active.clone_with_config(active.get_config())
        };
    }
}
