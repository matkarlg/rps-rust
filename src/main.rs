// Opt in to unstable features expected for Rust 2018
#![feature(rust_2018_preview)]
// Opt in to warnings about new 2018 idioms
#![warn(rust_2018_idioms)]

mod rps;
mod utils;

use self::rps::game::play_hands;
use self::rps::types::Hand;

fn main() {
	println!("Rock, Paper, Scissors?");

	let computer: Hand = rand::random();
	let player: Hand = loop {
		match utils::readln() {
			Ok(s) => break s,
			Err(_) => println!("Bad spelling, try again."),
		}
	};

	let result = play_hands(&player, &computer);

	println!("{} vs {}", player, computer);
	println!("You {}!", result);
}
