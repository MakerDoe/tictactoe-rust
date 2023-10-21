mod display;
mod game_state;
mod playmoves;

use crate::{
	bitboard::{Bitboard, Bitboards},
	sides::{Side, Sides},
	square::Square,
};

use self::game_state::GameState;

pub struct Board {
	sides: [Side; Sides::SIZE],
	game_state: GameState,

	history: Vec<GameState>,
}

impl Default for Board {
	fn default() -> Self {
		Self {
			sides: [Bitboards::EMPTY; Sides::SIZE],
			game_state: GameState::default(),
			history: Vec::new(),
		}
	}
}

impl Board {
	#[inline(always)]
	pub fn turn(&self) -> Side {
		self.game_state.turn
	}

	#[inline(always)]
	pub fn is_solved(&self) -> bool {
		self.game_state.solved
	}

	#[inline(always)]
	pub fn occupancy(&self) -> Bitboard {
		self.sides[Sides::O] | self.sides[Sides::X]
	}

	#[inline(always)]
	pub fn set_move(&mut self, side: Side, sq: Square) {
		Bitboards::set_bit(&mut self.sides[side], sq);
	}

	#[inline(always)]
	pub fn unset_move(&mut self, side: Side, sq: Square) {
		Bitboards::clear_bit(&mut self.sides[side], sq);
	}
}
