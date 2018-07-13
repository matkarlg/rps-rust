extern crate rand;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod rps;

use rps::{play_hands, utils, Hand};

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
