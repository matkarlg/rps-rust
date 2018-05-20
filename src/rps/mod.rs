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

#[derive(Display, EnumString, PartialEq, Rand)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Display)]
pub enum GameResult {
    Won,
    Draw,
    Lost,
}
