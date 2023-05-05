use palette::{rgb::{Rgba}, IntoColor};

pub mod primitives;
pub mod scene;
pub mod effects;
pub mod runtime;

pub type Pixels<T: IntoColor<Rgba>> = Vec<T>;

// TODO: ModR should a Vec of anything that can be converted to Color
#[derive(Debug)]
pub enum ModR<T> {
	/// Update specified pixels
	Pixels(Pixels<T>),
	/// Do nothing (loop again)
	Pass,
	/// Kill (stop & reset i)
	Kill
}

pub trait Module<T, C = Rgba> {
	fn new(input: T) -> Self;
	fn render(&mut self, i: u32, pixels: &Pixels<Rgba>) -> ModR<C>;
}
