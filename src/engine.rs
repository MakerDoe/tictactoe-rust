use std::io::{self, Write};

use crate::{
	board::Board,
	minimax::Minimax,
	movegen::{Move, MoveGenerator, MoveList},
	sides::{Side, Sides},
};

type InputMove = Move;

pub struct Engine;
impl Engine {
	pub fn start_normal() {
		let mut board = Board::default();
		let mut winner = Sides::NONE;
		Self::info();

		loop {
			let list = MoveGenerator::generate(&board);

			if list.len() == 0 {
				break;
			}

			if board.is_solved() {
				winner = board.turn() ^ 1;
				break;
			}

			println!(
				"{}'s turn; what is your move?",
				Sides::to_char(board.turn())
			);

			let m = Self::get_user_move(list);

			board.make_move(m);
			println!("{board}");
		}

		match winner != Sides::NONE {
			true => println!("{} won the game!", Sides::to_char(winner)),
			false => println!("The game ended in a draw!"),
		}
	}

	pub fn start_ai() {
		let mut board = Board::default();
		let mut winner = Sides::NONE;
		Self::info();

		let player = Self::get_user_side();

		loop {
			let list = MoveGenerator::generate(&board);
			let turn = board.turn();

			if list.len() == 0 {
				break;
			}

			if board.is_solved() {
				winner = turn ^ 1;
				break;
			}

			let m;

			match turn == player {
				true => {
					println!("It's your turn; what is your move?");
					m = Self::get_user_move(list);
				}
				false => {
					m = Minimax::get(&mut board, true, false).unwrap();
					println!(
						"It's the AI's turn; the AI chose the move: {}",
						Self::invert_move(m)
					);
				}
			}

			board.make_move(m);
			println!("{board}");
		}

		match winner != Sides::NONE {
			true => match winner == player {
				true => println!("WTF! This is impossible!"),
				false => println!("You lost; this is to be expected."),
			},
			false => println!("The game ended in a draw!"),
		}
	}
}

impl Engine {
	fn get_user_side() -> Side {
		loop {
			let mut input = Default::default();

			print!("Choose a side (X/o): ");
			io::stdout().flush().expect("Failure to flush stdout");
			io::stdin()
				.read_line(&mut input)
				.expect("Failure to read line");

			if let Ok(ch) = input.trim().parse::<String>() {
				match ch.as_str() {
					"X" | "x" => {
						println!("");
						return Sides::X;
					}
					"O" | "o" => {
						println!("");
						return Sides::O;
					}
					_ => (),
				}
			}
		}
	}

	fn get_user_move(list: MoveList) -> Move {
		let m;

		loop {
			let mut input = Default::default();

			print!("Choose a number from (0-8): ");
			io::stdout().flush().expect("Failure to flush stdout");
			io::stdin()
				.read_line(&mut input)
				.expect("Failure to read line");

			if let Ok(num) = input.trim().parse() {
				if (0..9).contains(&num) {
					let num = Self::invert_move(num);

					if list.contains(&num) {
						m = num;
						break;
					}
				}
			}
		}

		m
	}

	fn info() {
		println!("Board representation:");
		println!("+———+———+———+");
		println!("| 0 | 1 | 2 |");
		println!("+———+———+———+");
		println!("| 3 | 4 | 5 |");
		println!("+———+———+———+");
		println!("| 6 | 7 | 8 |");
		println!("+———+———+———+\n");
	}

	fn invert_move(m: InputMove) -> Move {
		match m {
			0 => 6,
			1 => 7,
			2 => 8,
			3 => 3,
			4 => 4,
			5 => 5,
			6 => 0,
			7 => 1,
			8 => 2,
			_ => panic!("Move is not recognized, got: {m}"),
		}
	}
}
