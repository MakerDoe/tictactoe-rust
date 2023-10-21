use rand::Rng;

pub type Side = usize;

pub struct Sides;
impl Sides {
	pub const X: Side = 0;
	pub const O: Side = 1;
	pub const NONE: Side = 2;

	pub const SIZE: usize = 2;

	#[inline(always)]
	pub fn random() -> Side {
		let mut rng = rand::thread_rng();
		let num = rng.gen_range(0..=1);

		match num {
			0 => Self::X,
			1 => Self::O,
			_ => panic!("Invalid random number, got: {num}"),
		}
	}

	#[inline(always)]
	pub fn to_char(side: Side) -> char {
		match side {
			Self::X => 'X',
			Self::O => 'O',
			_ => panic!("Side not recognized, got: {side}"),
		}
	}
}
