use std::fmt;
use std::str::FromStr;

pub mod utils;

pub fn play_hands(p1: &Hand, p2: &Hand) -> GameResult {
    use rps::GameResult::*;
    use rps::Hand::*;

    match (p1, p2) {
        (Paper, Rock) => Won,
        (Scissors, Paper) => Won,
        (Rock, Scissors) => Won,
        _ if p1 == p2 => Draw,
        _ => Lost,
    }
}

#[derive(Rand, PartialEq, Debug)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum GameResult {
    Won,
    Draw,
    Lost,
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Rock" => Ok(Hand::Rock),
            "Paper" => Ok(Hand::Paper),
            "Scissors" => Ok(Hand::Scissors),
            _ => Err("not a valid value"),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
