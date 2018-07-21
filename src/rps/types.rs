use rand::{
	distributions::{Distribution, Standard},
	Rng,
};
use strum_macros::{Display, EnumString};

#[derive(Display)]
crate enum GameResult {
	Won,
	Draw,
	Lost,
}

#[derive(Display, EnumString, PartialEq)]
crate enum Hand {
	Rock,
	Paper,
	Scissors,
}

// Create a random instance of Hand
impl Distribution<Hand> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hand {
		match rng.gen_range(0, 3) {
			0 => Hand::Rock,
			1 => Hand::Paper,
			_ => Hand::Scissors,
		}
	}
}
