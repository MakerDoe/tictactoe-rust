use crate::{bitboard::Bitboards, board::Board, square::Square};

pub type Move = Square;
pub type MoveList = Vec<Move>;

pub struct MoveGenerator;
impl MoveGenerator {
	pub fn generate(board: &Board) -> MoveList {
		let mut list = Vec::new();

		let bb_occ = board.occupancy();

		for mask in Bitboards::SQUARES {
			if bb_occ & mask == 0 {
				let sq = Bitboards::get_lsb(mask);
				list.push(sq);
			}
		}

		list
	}
}
