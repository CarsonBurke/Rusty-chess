use std::collections::{HashMap};
use crate::{units::{Unit, unit}, neural_network::NeuralNetwork, constants::BOARD_SIZE, manager::Manager, Pos};
use eventual::Timer;
extern crate eventual;

pub struct Game {
    pub id: String,
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
    pub fn new(id: String) -> Self {

        let mut board: Vec<Vec<Option<String>>> = vec![];
        
        let mut i = 0;
        while i < BOARD_SIZE {

            board.push(vec![]);
            i += 1;
        }

        return Self {
            id,
            tick: 0,
            units: HashMap::new(),
            board,
            winner: None,
        }
    }
    pub fn find_unit_at_pos(&mut self, pos: &Pos) -> Option<&Unit>  {

        let x = pos.x as usize;
        let y = pos.y as usize;

        let mut unit_id = &self.board[x][y];

        if let Some(unit_id) = unit_id {

            return self.units.get(unit_id);
        }

        return None
    }
    pub fn run(&mut self, tick_speed: i32) {

        let timer = Timer::new();
        let ticks = timer.interval_ms(tick_speed.try_into().unwrap()).iter();
        
        for _ in ticks {

            print!("tick for game");
            println!(" {}", self.id);
            println!("tick {}", self.tick);

            self.tick += 1;
        }
    }
    pub fn reset(&mut self) {

        self.tick = 0;
        self.units = HashMap::new();
        self.winner = None;

        // Reset each coord of the board

        let mut i: usize = 0;
        while i < BOARD_SIZE {

            self.board[i].clear();
            i += 1;
        } 
    }
}