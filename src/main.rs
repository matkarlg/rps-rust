extern crate rps_rust;

use std::process;

fn main() {
    if let Err(e) = rps_rust::run() {
        println!("{}", e);
        process::exit(1);
    }
}
