use std::collections::{HashMap};
use crate::{units::Unit, neural_network::NeuralNetwork, constants::BOARD_SIZE};

pub struct Game {
    tick: i32,
    units: HashMap<String, Unit>,
    board: Vec<Vec<Option<Unit>>>,
    winner: Option<NeuralNetwork>,
}

impl Game {
    pub fn new() -> Self {

        let mut board: Vec<Vec<Option<Unit>>> = vec![];
        
        let mut i = 0;
        while i < BOARD_SIZE {

            board.push(vec![]);
            i += 1;
        }

        return Game {
            tick: 0,
            units: HashMap::new(),
            board,
            winner: None,
        }
    }
}