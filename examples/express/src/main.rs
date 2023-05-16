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
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) };

    let mut runtime = Express::new(());
    //let mut grad = patterns::Gradient::update("FFD700,FF69B4,00CED1,FF69B4,FFD700|5");
    let pixels = vec![Rgba::new(0.1, 0.5, 0.3, 0.0); runtime.get_number_of_pixels().into()];

    let mut i = 0;

    loop {
        runtime.display(&pixels);
        runtime.timer.delay_ms(30u8);
    }
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
}
