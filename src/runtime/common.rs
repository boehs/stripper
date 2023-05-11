use std::{rc::Rc, sync::Arc};

use palette::rgb::Rgba;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{ModR, Module, Pixels};

use super::Runtime;

// Obviously eventually this will not be browser based.
pub fn run<T: 'static>(mut modules: Rc<dyn Module>, runtime: Box<dyn Runtime<T>>) {
    let pixels: Pixels = vec![Rgba::new(0.0, 0.0, 0.0, 0.0); runtime.get_number_of_pixels().into()];
    runtime.display(&pixels);

    let mut i = 0;
    let window = web_sys::window().unwrap();

    let callback = Closure::wrap(Box::new(move || {
        if let ModR::Pixels(pix) = Rc::<(dyn Module + 'static)>::get_mut(&mut modules)
            .unwrap()
            .render(i, &pixels)
        {
            runtime.display(&pix);
        }
        i = i + 1
    }) as Box<dyn FnMut()>);

    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            30,
        )
        .unwrap();

    callback.forget()
}
