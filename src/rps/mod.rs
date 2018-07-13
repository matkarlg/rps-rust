pub mod utils;

use rand::{
	distributions::{Distribution, Standard},
	Rng,
};
use rps::GameResult::*;
use rps::Hand::*;

pub fn play_hands(p1: &Hand, p2: &Hand) -> GameResult {
	match (p1, p2) {
		(Paper, Rock) => Won,
		(Scissors, Paper) => Won,
		(Rock, Scissors) => Won,
		_ if p1 == p2 => Draw,
		_ => Lost,
	}
}

#[derive(Display, EnumString, PartialEq)]
pub enum Hand {
	Rock,
	Paper,
	Scissors,
}

impl Distribution<Hand> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hand {
		match rng.gen_range(0, 3) {
			0 => Rock,
			1 => Paper,
			_ => Scissors,
		}
	}
}

#[derive(Display)]
pub enum GameResult {
	Won,
	Draw,
	Lost,
}
