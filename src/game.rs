use std::{collections::{HashMap}, borrow::BorrowMut};
use crate::{units::{Unit, unit}, neural_network::NeuralNetwork, constants::{BOARD_SIZE}, manager::Manager, Pos};

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub tick: i128,
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
    pub fn find_unit_at_pos(&mut self, pos: &Pos) -> Option<&Unit>  {

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
        /* println!("{:?}", self.board); */
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



        let mut units = self.units.clone();

        for (unit_id, unit) in units.borrow_mut() {

            unit.any_move(self);
            print!("Unit pos");
            println!("{:?}", unit.pos);
        }

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