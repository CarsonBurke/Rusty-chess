mod utils;
use std::vec;
pub mod units; use units::*;
pub mod manager; use manager::*;
pub mod game; use game::*;
pub mod neural_network;
pub mod constants;
pub mod player;

#[derive(Default, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");

    let mut manager = Manager::new(1, 1000);
}

