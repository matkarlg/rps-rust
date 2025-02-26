//! A simple Rock, Paper, Scissors game.

use rand::seq::IteratorRandom;
use std::{io, str::FromStr};
use strum::{EnumIter, IntoEnumIterator};

fn main() {
	use self::Choice as C;

	let mut rng = rand::rng();

	loop {
		let computer = Choice::iter().choose(&mut rng).unwrap();

		let player: Choice = loop {
			let mut input = String::new();
			println!("Rock, Paper, Scissors?");
			io::stdin().read_line(&mut input).unwrap();

			match input.parse() {
				Ok(s) => break s,
				Err(()) => println!("Bad spelling, try again."),
			}
		};

		println!("{:?} vs {:?}", &player, &computer);

		match (&player, &computer) {
			(C::Paper, C::Rock) | (C::Scissors, C::Paper) | (C::Rock, C::Scissors) => {
				println!("You Win");
			}
			_ if player == computer => println!("Tie, Replay!"),
			_ => println!("Computer Wins!"),
		}
	}
}

#[derive(Debug, PartialEq, EnumIter)]
enum Choice {
	Rock,
	Paper,
	Scissors,
}

impl FromStr for Choice {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.trim().to_lowercase().as_str() {
			"r" | "rock" => Ok(Choice::Rock),
			"p" | "paper" => Ok(Choice::Paper),
			"s" | "scissors" => Ok(Choice::Scissors),
			_ => Err(()),
		}
	}
}
