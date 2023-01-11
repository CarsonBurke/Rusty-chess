use std::collections::{HashMap};
use crate::{units::Unit, neural_network::NeuralNetwork, constants::BOARD_SIZE, manager::Manager};
use eventual::Timer;
extern crate eventual;

pub struct Game {
    pub tick: i32,
    pub units: HashMap<String, Unit>,
    /* pub units_by_type: HashMap<>, */
    pub board: Vec<Vec<Option<Unit>>>,
    pub winner: Option<NeuralNetwork>,
}

impl Game {
    pub fn new() -> Self {

        let mut board: Vec<Vec<Option<Unit>>> = vec![];
        
        let mut i = 0;
        while i < BOARD_SIZE {

            board.push(vec![]);
            i += 1;
        }

        return Self {
            tick: 0,
            units: HashMap::new(),
            board,
            winner: None,
        }
    }
    pub fn run(&mut self, manager: &mut Manager) {

        let timer = Timer::new();
        let ticks = timer.interval_ms(manager.tick_speed.try_into().unwrap()).iter();
        
        for _ in ticks {

            println!("{}", self.tick);

            self.tick += 1;
            manager.tick += 1;
        }
    }
}