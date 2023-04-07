use std::{collections::{HashMap}, borrow::BorrowMut};
use crate::{units::{Unit, unit}, neural_network::NeuralNetwork, constants::{BOARD_SIZE}, manager::Manager, Pos, MoveRequest};

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub tick: u64,
    pub units: HashMap<String, Unit>,
    /* pub units_by_type: HashMap<>, */
    /**
     * A graph with values of unit ids
     */
    pub board: Vec<Vec<String>>,
    pub winner: Option<NeuralNetwork>,
}

impl Game {
    pub fn new(id: String) -> Self {

        let mut board: Vec<Vec<String>> = vec![];

        for y in 0..BOARD_SIZE {
            board.push(vec![]);
            for x in 0..BOARD_SIZE {
                board[y].push("".to_string());
            }
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

        let y: usize = 7;
        for x in 0..BOARD_SIZE {

            let unit = Unit::new(manager, self, Pos { x: x as i32, y: y as i32 }, "black".to_string(), "pawn".to_string());
            let clone = unit.clone();
            
            self.board[y][x] = unit.id.clone();
            self.units.insert(unit.id, clone);
            

        }
    }
    pub fn find_unit_at_pos(&self, pos: &Pos) -> Option<&Unit>  {

        let x = pos.x as usize;
        let y = pos.y as usize;

        let unit_id = &self.board[y][x];
        return self.units.get(unit_id);
    }
    pub fn visualize_grid(&mut self) {

        for y in 0..BOARD_SIZE {
            println!();
            for x in 0..BOARD_SIZE {

                print!(" ");
                print!("{}", x);
                print!(",");
                print!("{}", y);
                print!(" ");
            }
        }

        println!();
        println!();
    }
    pub fn visualize(&mut self, unit_graphics: HashMap<String, HashMap<String, String>>) {

        for y in 0..BOARD_SIZE {
            println!();
            for x in 0..BOARD_SIZE {
            
                let unit = self.units.get(&self.board[y][x]);
                match unit {
                    Some(unit) => { 

                        print!(" ");
                        print!("{}", unit_graphics[&unit.player_type][&unit.unit_type]);
                        print!(" ");
                    },
                    _ => { 

                        print!(" ☒ ");
                    }
                }
            }
        }

        println!();
        println!();
    }
    pub fn run(&mut self, unit_graphics: HashMap<String, HashMap<String, String>>) {

        print!("tick for game of id");
        println!(" {}", self.id);
        println!("tick {}", self.tick);

        /* self.visualize_grid(); */
        self.visualize(unit_graphics);
/* 
        if self.tick > 5 {

            self.winner = Some(NeuralNetwork::new());
            return;
        }
 */

        let mut move_requests: Vec<MoveRequest> = vec![];

        let unit_ids: Vec<String> = self.units.keys().cloned().collect();

        for unit_id in unit_ids {

            let unit = self.units.get(&unit_id).unwrap();

            let move_request = unit.any_move_request(self);
            match move_request {
                Some(move_request) => {

                    move_requests.push(move_request)
                }
                _ => {}
            }
        }

        for move_request in move_requests {

            self.move_unit_to(move_request);
        }

        self.tick += 1;
    }
    pub fn move_unit_to(&mut self, move_request: MoveRequest) -> bool {

        // Delete unit at pos if there is one

        let unit_at_move_pos_id = &self.board[move_request.pos.y as usize][move_request.pos.x as usize];
        if self.units.get(unit_at_move_pos_id).is_some() {

            self.units.remove(unit_at_move_pos_id);
        }

        let unit = self.units.get_mut(&move_request.unit_id).unwrap();

        let old_x = unit.pos.x;
        let old_y = unit.pos.y;

        unit.last_pos = unit.pos.clone();
        self.board[old_y as usize][old_x as usize] = "".to_string();

        unit.pos = move_request.pos.clone();

        self.board[unit.pos.y as usize][unit.pos.x as usize] = unit.id.clone();
        return true
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