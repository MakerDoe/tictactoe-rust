use std::fmt::Display;

use crate::{bitboard::Bitboards, sides::Sides, square::Squares};

use super::Board;

impl Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut board = String::from("+———+———+———+\n");

		let bb_occ = self.occupancy();

		for y in Squares::HEIGHT.rev() {
			for x in Squares::WIDTH {
				let sq = Squares::from_x_y(x, y);

				let ch = match Bitboards::occupied(bb_occ, sq) {
					true => match Bitboards::occupied(self.sides[Sides::O], sq) {
						true => 'O',
						false => 'X',
					},
					false => ' ',
				};

				board += &format!("| {} ", ch);

				if x == (Squares::WIDTH.end - 1) {
					board += "|";
				}
			}

			board += "\n+———+———+———+\n";
		}

		write!(f, "{board}")
	}
}
