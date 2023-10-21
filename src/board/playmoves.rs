use crate::{bitboard::Bitboard, movegen::Move, sides::Side, square::Squares};

use super::Board;

impl Board {
	#[inline]
	pub fn make_move(&mut self, m: Move) {
		let mut current_gs = self.game_state;
		current_gs.next_move = m;
		self.history.push(current_gs);

		let turn = self.turn();
		self.set_move(turn, m);

		self.game_state.turn ^= 1;
		self.game_state.solved = self._is_solved(turn, m);
	}

	#[inline]
	pub fn unmake(&mut self) {
		if let Some(gs) = self.history.pop() {
			self.game_state = gs;

			let turn = self.turn();
			let m = self.game_state.next_move;
			self.unset_move(turn, m);
		}
	}
}

impl Board {
	const SOLVED_MASK: [&[Bitboard]; Squares::SIZE] = [
		&[0x7, 0x49, 0x111],
		&[0x7, 0x92],
		&[0x7, 0x124, 0x54],
		&[0x38, 0x49],
		&[0x38, 0x92, 0x111, 0x54],
		&[0x38, 0x124],
		&[0x1c0, 0x49, 0x54],
		&[0x1c0, 0x92],
		&[0x1c0, 0x124, 0x111],
	];

	#[inline(always)]
	fn _is_solved(&self, turn: Side, m: Move) -> bool {
		let side = self.sides[turn];

		for mask in Self::SOLVED_MASK[m] {
			if side & *mask == *mask {
				return true;
			}
		}

		false
	}
}
