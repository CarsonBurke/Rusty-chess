use std::{collections::{HashMap}, borrow::BorrowMut};
use rand::{random, Rng};

use crate::{units::{Unit, unit}, neural_network::NeuralNetwork, constants::{BOARD_SIZE}, manager::Manager, Pos, MoveRequest, player::Player};

#[derive(Debug)]
pub struct Game {
    id_index: u64,
    pub id: String,
    pub tick: u64,
    pub units: HashMap<String, Unit>,
    /* pub units_by_type: HashMap<>, */
    /**
     * A graph with values of unit ids
     */
    pub players: HashMap<String, Player>,
    pub board: Vec<Vec<String>>,
    /**
     * The ID of the winning neural network, if exists
     */
    pub winner: Option<String>,
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
            id_index: 0,
            id,
            tick: 0,
            units: HashMap::new(),
            board,
            winner: None,
            players: HashMap::new(),
        }
    }
    pub fn new_id(&mut self) -> String {
        
        self.id_index += 1;
        return (&self.id_index).to_string()
    }
    fn new_unit(&mut self, pos: Pos, player_type: String, unit_type: String) {

        let unit = Unit::new(self, pos, player_type, unit_type);
        let clone = unit.clone();
        
        self.board[pos.y as usize][pos.x as usize] = unit.id.clone();
        self.units.insert(unit.id, clone);
    }
    pub fn init(&mut self, manager: &mut Manager) {

        // White

        let mut player_type = "white";
        self.players.insert(player_type.to_string(), Player::new(player_type.to_string(), false));

        let mut y: i32 = 6;
        for x in 0..BOARD_SIZE as i32 {

            self.new_unit(Pos { x: x, y: y }, player_type.to_string(), "pawn".to_string());
        }

        y = 7;

        self.new_unit(Pos { x: 0, y: y }, player_type.to_string(), "castle".to_string());
        self.new_unit(Pos { x: 7, y: y }, player_type.to_string(), "castle".to_string());
        self.new_unit(Pos { x: 1, y: y }, player_type.to_string(), "knight".to_string());
        self.new_unit(Pos { x: 6, y: y }, player_type.to_string(), "knight".to_string());
        self.new_unit(Pos { x: 2, y: y }, player_type.to_string(), "bishop".to_string());
        self.new_unit(Pos { x: 5, y: y }, player_type.to_string(), "bishop".to_string());
        self.new_unit(Pos { x: 3, y: y }, player_type.to_string(), "king".to_string());
        self.new_unit(Pos { x: 4, y: y }, player_type.to_string(), "queen".to_string());

        // Black

        player_type = "black";
        self.players.insert(player_type.to_string(), Player::new(player_type.to_string(), true));

        y = 1;
        for x in 0..BOARD_SIZE as i32 {

            self.new_unit(Pos { x: x, y: y }, player_type.to_string(), "pawn".to_string());
        }

        y = 0;

        self.new_unit(Pos { x: 0, y: y }, player_type.to_string(), "castle".to_string());
        self.new_unit(Pos { x: 7, y: y }, player_type.to_string(), "castle".to_string());
        self.new_unit(Pos { x: 1, y: y }, player_type.to_string(), "knight".to_string());
        self.new_unit(Pos { x: 6, y: y }, player_type.to_string(), "knight".to_string());
        self.new_unit(Pos { x: 2, y: y }, player_type.to_string(), "bishop".to_string());
        self.new_unit(Pos { x: 5, y: y }, player_type.to_string(), "bishop".to_string());
        self.new_unit(Pos { x: 3, y: y }, player_type.to_string(), "king".to_string());
        self.new_unit(Pos { x: 4, y: y }, player_type.to_string(), "queen".to_string());

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

        let mut turn_player_type: Option<String> = None;
        for (player_id, player) in &self.players {

            if (self.tick % 2 == 0) != player.turn_mod { continue };

            turn_player_type = Some(player.player_type.clone());
            break
        }

        let mut kings: HashMap<String, bool> = HashMap::new();
        kings.insert("black".to_string(), false);
        kings.insert("white".to_string(), false);

        let mut move_requests: Vec<MoveRequest> = vec![];

        let unit_ids: Vec<String> = self.units.keys().cloned().collect();
        for unit_id in unit_ids {

            let unit = self.units.get(&unit_id).unwrap();

            if unit.unit_type == "king" {
                kings.insert(unit.player_type.clone(), true);
            }

            match turn_player_type.as_deref() {
                Some(turn_player_type) => {

                    if unit.player_type != turn_player_type { continue };
                },
                _ => {}
            }

            let move_request = unit.any_move_request(self);
            match move_request {
                Some(move_request) => {

                    move_requests.push(move_request)
                }
                _ => {}
            }
        }

        for (player_type, has_king) in kings {
            if has_king { continue }

            // Black lost

            if player_type == "black" {

                self.winner = Some(self.players.get(&"white".to_string()).unwrap().network_id.clone());
                return
            }

            // White lost


            self.winner = Some(self.players.get(&"black".to_string()).unwrap().network_id.clone());
            return
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, move_requests.len() - 1);
        let move_request = move_requests.get(index).unwrap().clone();

        self.move_unit_to(move_request);

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

        let mut board: Vec<Vec<String>> = vec![];

        for y in 0..BOARD_SIZE {
            board.push(vec![]);
            for x in 0..BOARD_SIZE {
                board[y].push("".to_string());
            }
        }

        self.board = board;
    }
}