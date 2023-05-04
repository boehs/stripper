use smart_leds::RGB8;

pub mod primitives;
pub mod scene;
pub mod effects;

pub type Pixels = Vec<RGB8>;

// TODO: ModR should a Vec of anything that can be converted to Color
pub enum ModR {
	/// Update specified pixels
	Some(Pixels),
	/// Do nothing (loop again)
	None,
	/// Kill (stop & reset i)
	Kill
}

pub trait Module<T> {
	fn new(input: T) -> Self;
	fn render(&self, i: u32, pixels: &Pixels) -> ModR;
}
