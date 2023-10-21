use crate::{
	board::Board,
	movegen::{Move, MoveGenerator},
};

pub type BestMove = Move;

pub struct Minimax;
impl Minimax {
	pub fn get(board: &mut Board, pruning: bool, output: bool) -> Option<BestMove> {
		let mut best_move = None;
		let mut best_score = i32::MIN;

		let mut nodes = 0;

		for m in MoveGenerator::generate(board) {
			board.make_move(m);

			let score = match pruning {
				true => Self::alpha_beta(board, false, 0, i32::MIN, i32::MAX, &mut nodes),
				false => Self::driver(board, false, 0, &mut nodes),
			};

			if score > best_score {
				best_score = score;
				best_move = Some(m);
			}

			board.unmake();
		}

		if output {
			println!("Alpha Beta Pruning: {pruning}");
			println!("Nodes Searched:\t{nodes}");
			println!(
				"Best Move:\t{}",
				match best_move {
					Some(m) => m.to_string(),
					None => "None".to_string(),
				}
			);
		}

		best_move
	}

	#[inline(always)]
	fn driver(board: &mut Board, maximizing: bool, depth: i32, nodes: &mut usize) -> i32 {
		if board.is_solved() {
			*nodes += 1;
			return match maximizing {
				true => -10 + depth,
				false => 10 - depth,
			};
		}

		let list = MoveGenerator::generate(board);

		match list.is_empty() {
			true => {
				*nodes += 1;
				return 0;
			}
			false => {
				let mut best_score = match maximizing {
					true => i32::MIN,
					false => i32::MAX,
				};

				for m in list {
					board.make_move(m);

					let score = Self::driver(board, !maximizing, depth + 1, nodes);
					best_score = match maximizing {
						true => best_score.max(score),
						false => best_score.min(score),
					};

					board.unmake();
				}

				best_score
			}
		}
	}

	#[inline(always)]
	fn alpha_beta(
		board: &mut Board,
		maximizing: bool,
		depth: i32,
		mut alpha: i32,
		mut beta: i32,
		nodes: &mut usize,
	) -> i32 {
		if board.is_solved() {
			*nodes += 1;
			return match maximizing {
				true => -10 + depth,
				false => 10 - depth,
			};
		}

		let list = MoveGenerator::generate(board);

		match list.is_empty() {
			true => {
				*nodes += 1;
				return 0;
			}
			false => {
				let mut best_score = match maximizing {
					true => i32::MIN,
					false => i32::MAX,
				};

				for m in list {
					board.make_move(m);

					let score = Self::alpha_beta(board, !maximizing, depth + 1, alpha, beta, nodes);
					best_score = match maximizing {
						true => best_score.max(score),
						false => best_score.min(score),
					};

					board.unmake();

					match maximizing {
						true => {
							if best_score > beta {
								break;
							}

							alpha = alpha.max(best_score);
						}

						false => {
							if best_score < alpha {
								break;
							}

							beta = beta.min(best_score);
						}
					};
				}

				best_score
			}
		}
	}
}
