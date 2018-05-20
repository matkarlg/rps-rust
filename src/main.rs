extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod rps;

use rps::{play_hands, utils, Hand};

fn main() {
    println!("Rock, Paper, Scissors?");

    let computer: Hand = rand::random();
    let player: Hand = loop {
        if let Ok(s) = utils::readln() {
            break s;
        }

        println!("Bad spelling, try again.");
    };

    let result = play_hands(&player, &computer);

    println!("{} vs {}", player, computer);
    println!("You {}!", result);
}
