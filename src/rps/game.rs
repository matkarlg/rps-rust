use super::types::GameResult::*;
use super::types::Hand::*;
use super::types::*;

crate fn play_hands(p1: &Hand, p2: &Hand) -> GameResult {
	match (p1, p2) {
		(Paper, Rock) => Won,
		(Scissors, Paper) => Won,
		(Rock, Scissors) => Won,
		_ if p1 == p2 => Draw,
		_ => Lost,
	}
}
