mod utils;
use std::vec;
pub mod units; use units::*;
pub mod runner; use runner::*;
pub mod game; use game::*;
pub mod neural_network;
pub mod constants;
pub mod player;


pub struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");
}

