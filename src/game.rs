use std::{collections::{HashMap}};
use crate::{units::{Unit, unit}, neural_network::NeuralNetwork, constants::BOARD_SIZE, manager::Manager, Pos};

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub tick: i128,
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
    pub fn init(&mut self, manager: &mut Manager) {

        for i in 0..BOARD_SIZE {

            let unit = Unit::new(manager, self, Pos { x:2, y:2 });
            let clone = unit.clone();
    
            self.board[i][i] = Some(unit.id.clone());
            self.units.insert(unit.id, clone);

            
        }
    }
    pub fn find_unit_at_pos(&mut self, pos: &Pos) -> Option<&Unit>  {

        let x = pos.x as usize;
        let y = pos.y as usize;

        let unit_id = &self.board[x][y];

        if let Some(unit_id) = unit_id {

            return self.units.get(unit_id);
        }

        return None
    }
    pub fn run(&mut self) {

        print!("tick for game of id");
        println!(" {}", self.id);
        println!("tick {}", self.tick);
/* 
        if self.tick > 5 {

            self.winner = Some(NeuralNetwork::new());
            return;
        }
 */
        self.tick += 1;
    }
    pub fn reset(&mut self) {

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