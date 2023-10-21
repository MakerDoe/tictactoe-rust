use crate::{
	movegen::Move,
	sides::{Side, Sides},
};

#[derive(Clone, Copy)]
pub struct GameState {
	pub turn: Side,
	pub solved: bool,

	pub next_move: Move,
}

impl Default for GameState {
	fn default() -> Self {
		Self {
			turn: Sides::random(),
			solved: false,
			next_move: Default::default(),
		}
	}
}
