use std::collections::{HashMap};
use crate::{units::Unit, neural_network::NeuralNetwork, constants::BOARD_SIZE, manager::Manager};
use eventual::Timer;
extern crate eventual;

pub struct Game {
    id: String,
    pub tick: i32,
    pub units: HashMap<String, Unit>,
    /* pub units_by_type: HashMap<>, */
    /**
     * A graph with values of unit ids
     */
    pub board: Vec<Vec<Option<String>>>,
    pub winner: Option<NeuralNetwork>,
}

impl Game {
    pub fn new(manager: &mut Manager) -> Self {

        let mut board: Vec<Vec<Option<String>>> = vec![];
        
        let mut i = 0;
        while i < BOARD_SIZE {

            board.push(vec![]);
            i += 1;
        }

        return Self {
            id: manager.new_id(),
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