mod utils;
use std::vec;
pub mod units; use units::*;
pub mod manager; use manager::*;
pub mod game; use game::*;
pub mod neural_network;
pub mod constants;
pub mod player;

#[derive(Default, Debug, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}
#[derive(Default, Debug, Clone)]
pub struct MoveRequest {
    unit_id: String,
    pos: Pos,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut manager = Manager::new(1, 1000);
    manager.init();
    manager.run().await;
}