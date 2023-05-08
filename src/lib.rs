use palette::rgb::{Rgba};

pub mod primitives;
pub mod scene;
pub mod effects;
pub mod runtime;

pub type Pixels = Vec<Rgba>;

// TODO: ModR should a Vec of anything that can be converted to Color
#[derive(Debug)]
pub enum ModR {
	/// Update specified pixels
	Pixels(Pixels),
	/// Do nothing (loop again)
	Pass,
	/// Kill (stop & reset i)
	Kill
}

pub trait Module<T> {
	fn new(input: T) -> Self;
	fn render(&mut self, i: u32, pixels: &Pixels) -> ModR;
}
