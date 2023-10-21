use crate::square::{Square, Squares};

pub type Bitboard = usize;

pub struct Bitboards;
impl Bitboards {
	pub const EMPTY: Bitboard = 0;
	pub const SQUARES: [Square; Squares::SIZE] = [
		0b1,
		0b10,
		0b100,
		0b1000,
		0b10000,
		0b100000,
		0b1000000,
		0b10000000,
		0b100000000,
	];

	#[inline(always)]
	pub fn set_bit(bb: &mut Bitboard, sq: Square) {
		*bb |= Self::SQUARES[sq];
	}

	#[inline(always)]
	pub fn clear_bit(bb: &mut Bitboard, sq: Square) {
		*bb ^= Self::SQUARES[sq];
	}

	#[inline(always)]
	pub fn occupied(bb: Bitboard, sq: Square) -> bool {
		bb & Self::SQUARES[sq] != 0
	}

	#[inline(always)]
	pub fn get_lsb(bb: Bitboard) -> Square {
		bb.trailing_zeros() as Square
	}

	#[allow(dead_code)]
	pub fn print(bb: Bitboard, name: Option<&str>) {
		if let Some(name) = name {
			println!(" ↓ {name}\n");
		}

		let mut board = String::new();

		for y in Squares::HEIGHT.rev() {
			for x in Squares::WIDTH {
				let sq = Squares::from_x_y(x, y);

				board += match Self::occupied(bb, sq) {
					true => " 1 ",
					false => " 0 ",
				}
			}

			board += "\n";
		}

		println!("{board}");
		println!(" ↑ Bitboard: {bb}\n");
	}
}
