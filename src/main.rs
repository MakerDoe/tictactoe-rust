use std::io::{self, Write};

use crate::engine::Engine;

mod bitboard;
mod board;
mod engine;
mod minimax;
mod movegen;
mod sides;
mod square;

fn main() {
	loop {
		let mut input = Default::default();

		print!("Play with AI (Y/n): ");
		io::stdout().flush().expect("Failure to flush stdout");
		io::stdin()
			.read_line(&mut input)
			.expect("Failer to read line");

		match input.trim().to_uppercase().as_str() {
			"Y" => Engine::start_ai(),
			"N" => Engine::start_normal(),
			_ => continue,
		}

		loop {
			let mut input = Default::default();

			print!("Do you want to play again (Y/n): ");
			io::stdout().flush().expect("Failure to flush stdout");
			io::stdin()
				.read_line(&mut input)
				.expect("Failer to read line");

			match input.trim().to_uppercase().as_str() {
				"Y" => break,
				"N" => return,
				_ => continue,
			}
		}
	}
}
