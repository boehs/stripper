#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use alloc_cortex_m::CortexMHeap;
use panic_halt as _;

#[macro_use]
extern crate alloc;

use bsp::{entry, hal::prelude::*};
use circuit_playground_express as bsp;
use stripper::{
    primitives::color::Rgba,
    runtime::{express::Express, Runtime},
    ModR, Module,
};

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 16384 ; // in bytes

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) };

    let mut runtime = Express::new(());
    let mut grad = patterns::Gradient::update("#ff0000, #ff9a00, #d0de21, #4fdc4a, #3fdad8, #2fc9e2, #1c7fee, #5f15f2, #ba0cf8, #fb07d9, #ff0000|30");
    let pixels = vec![Rgba::new(0.0, 5.0, 30.0, 0.0); (runtime.get_number_of_pixels() + 1).into()];

    let mut i = 0;

    loop {
        if let ModR::Pixels(pix) = grad.render(i, &pixels) {
            runtime.display(&pix);
        }
        i+=1;
    }
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
}
