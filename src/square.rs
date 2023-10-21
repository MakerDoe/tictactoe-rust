pub type Square = usize;

pub struct Squares;
impl Squares {
	pub const SIZE: usize = 9;

	pub const WIDTH: std::ops::Range<usize> = 0..3;
	pub const HEIGHT: std::ops::Range<usize> = 0..3;

	#[inline(always)]
	pub fn from_x_y(x: usize, y: usize) -> Square {
		y * 3 + x
	}
}
