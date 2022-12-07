use rand::seq::IteratorRandom;
use std::{io, str::FromStr};
use strum::{EnumIter, IntoEnumIterator};

fn main() {
	let mut rng = rand::thread_rng();

	loop {
		let computer = Choice::iter().choose(&mut rng).unwrap();

		let player: Choice = loop {
			let mut input = String::new();
			println!("Rock, Paper, Scissors?");
			io::stdin().read_line(&mut input).unwrap();

			match input.parse() {
				Ok(s) => break s,
				Err(_) => println!("Bad spelling, try again."),
			}
		};

		println!("{:?} vs {:?}", &player, &computer);

		use self::Choice::*;
		match (&player, &computer) {
			(Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => println!("You Win"),
			_ if player == computer => println!("Tie, Replay!"),
			_ => println!("Computer Wins!"),
		};
	}
}

#[derive(Debug, PartialEq, EnumIter)]
enum Choice {
	Rock,
	Paper,
	Scissors,
}

// Choice can be parsed from a String.
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
